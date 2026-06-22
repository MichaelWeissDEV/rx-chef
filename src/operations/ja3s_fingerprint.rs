/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JA3S Fingerprint operation.
 * -----------------------------------------------------------------------------
 */

use md5::{Digest, Md5};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JA3S Fingerprint operation
pub struct JA3SFingerprint;

impl Operation for JA3SFingerprint {
    fn name(&self) -> &'static str {
        "JA3S Fingerprint"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates a JA3S fingerprint to help identify TLS servers based on hashing together values from the Server Hello.<br><br>Input: A hex stream of the TLS Server Hello record application layer."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input format",
                description: "Input format",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output format",
                description: "Output format",
                default_value: "Hash digest",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_format = args.first().and_then(|v| v.as_str()).unwrap_or("Hex");
        let output_format = args
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("Hash digest");

        let bytes = convert_to_byte_array(input, input_format)?;
        let mut s = TlsParser::new(&bytes);

        // Handshake
        if s.read_u8() != Some(0x16) {
            return Err(OperationError::ProcessingError(
                "Not handshake data.".to_string(),
            ));
        }

        // Version
        s.skip(2);

        // Length
        let _length = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for length.".to_string())
        })?;

        // Handshake type
        if s.read_u8() != Some(2) {
            return Err(OperationError::ProcessingError(
                "Not a Server Hello.".to_string(),
            ));
        }

        // Handshake length
        let _handshake_length = s.read_u24().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for handshake length.".to_string())
        })?;

        // Hello version
        let hello_version = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for hello version.".to_string())
        })?;

        // Random
        s.skip(32);

        // Session ID
        let session_id_length = s.read_u8().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for session ID length.".to_string())
        })? as usize;
        s.skip(session_id_length);

        // Cipher suite
        let cipher_suite = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for cipher suite.".to_string())
        })?;

        // Compression Method
        s.skip(1);

        // Extensions
        let mut exts = Vec::new();
        if s.remaining() >= 2 {
            let extensions_length = s.read_u16().ok_or_else(|| {
                OperationError::ProcessingError(
                    "Insufficient data for extensions length.".to_string(),
                )
            })? as usize;
            let extensions_bytes = s.read_bytes(extensions_length).ok_or_else(|| {
                OperationError::ProcessingError("Insufficient data for extensions.".to_string())
            })?;

            let mut es = TlsParser::new(extensions_bytes);

            while es.has_more() {
                let ext_type = es.read_u16().ok_or_else(|| {
                    OperationError::ProcessingError(
                        "Insufficient data for extension type.".to_string(),
                    )
                })?;
                let ext_len = es.read_u16().ok_or_else(|| {
                    OperationError::ProcessingError(
                        "Insufficient data for extension length.".to_string(),
                    )
                })? as usize;
                es.skip(ext_len);
                exts.push(ext_type.to_string());
            }
        }

        let ja3s_str = format!("{},{},{}", hello_version, cipher_suite, exts.join("-"));

        let mut hasher = Md5::new();
        hasher.update(ja3s_str.as_bytes());
        let ja3s_hash = hex::encode(hasher.finalize());

        match output_format {
            "JA3S string" => Ok(ja3s_str.into_bytes()),
            "Full details" => {
                let details = format!("Hash digest:\n{}\n\nFull JA3S string:\n{}\n\nTLS Version:\n{}\nCipher Suite:\n{}\nExtensions:\n{}",
                    ja3s_hash, ja3s_str, hello_version, cipher_suite, exts.join("-"));
                Ok(details.into_bytes())
            }
            "Hash digest" | _ => Ok(ja3s_hash.into_bytes()),
        }
    }
}

struct TlsParser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> TlsParser<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            let val = self.data[self.pos];
            self.pos += 1;
            Some(val)
        } else {
            None
        }
    }

    fn read_u16(&mut self) -> Option<u16> {
        if self.pos + 1 < self.data.len() {
            let val = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
            self.pos += 2;
            Some(val)
        } else {
            None
        }
    }

    fn read_u24(&mut self) -> Option<u32> {
        if self.pos + 2 < self.data.len() {
            let val = ((self.data[self.pos] as u32) << 16)
                | ((self.data[self.pos + 1] as u32) << 8)
                | (self.data[self.pos + 2] as u32);
            self.pos += 3;
            Some(val)
        } else {
            None
        }
    }

    fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.pos + len <= self.data.len() {
            let val = &self.data[self.pos..self.pos + len];
            self.pos += len;
            Some(val)
        } else {
            None
        }
    }

    fn skip(&mut self, len: usize) {
        self.pos += len;
    }

    fn has_more(&self) -> bool {
        self.pos < self.data.len()
    }

    fn remaining(&self) -> usize {
        if self.pos > self.data.len() {
            0
        } else {
            self.data.len() - self.pos
        }
    }
}

fn convert_to_byte_array(input: Vec<u8>, format: &str) -> Result<Vec<u8>, OperationError> {
    match format {
        "Hex" => {
            let input_str = String::from_utf8(input)
                .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
            let clean_input = input_str.replace(|c: char| !c.is_ascii_hexdigit(), "");
            hex::decode(clean_input)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))
        }
        "Base64" => {
            let input_str = String::from_utf8(input)
                .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
            use base64::{engine::general_purpose, Engine as _};
            general_purpose::STANDARD
                .decode(input_str.trim())
                .map_err(|e| OperationError::InvalidInput(format!("Invalid base64: {}", e)))
        }
        "Raw" => Ok(input),
        _ => Err(OperationError::InvalidArgument {
            name: "Input format".to_string(),
            reason: "Invalid format".to_string(),
        }),
    }
}
