/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JA4Server Fingerprint operation.
 * -----------------------------------------------------------------------------
 */

use sha2::{Digest, Sha256};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JA4Server Fingerprint operation
pub struct JA4ServerFingerprint;

impl Operation for JA4ServerFingerprint {
    fn name(&self) -> &'static str {
        "JA4Server Fingerprint"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates a JA4Server Fingerprint (JA4S) to help identify TLS servers or sessions based on hashing together values from the Server Hello.<br><br>Input: A hex stream of the TLS or QUIC Server Hello packet application layer."
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
                default_value: "JA4S",
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
        let output_format = args.get(1).and_then(|v| v.as_str()).unwrap_or("JA4S");

        let bytes = convert_to_byte_array(input, input_format)?;
        let mut s = TlsParser::new(&bytes);

        // Record Header
        if s.read_u8() != Some(0x16) {
            return Err(OperationError::ProcessingError(
                "Not handshake data.".to_string(),
            ));
        }
        s.skip(2); // Version
        let _length = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for length.".to_string())
        })?;

        // Handshake
        if s.read_u8() != Some(2) {
            // Server Hello
            return Err(OperationError::ProcessingError(
                "Not a Server Hello.".to_string(),
            ));
        }
        let _handshake_length = s.read_u24().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for handshake length.".to_string())
        })?;

        let mut hello_version = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for hello version.".to_string())
        })?;
        s.skip(32); // Random
        let session_id_len = s.read_u8().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for session ID length.".to_string())
        })? as usize;
        s.skip(session_id_len);

        let cipher_suite = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for cipher suite.".to_string())
        })?;
        let _comp_method = s.read_u8().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for compression method.".to_string())
        })?;

        let mut alpn = "00".to_string();
        let mut extensions_list = Vec::new();

        if s.remaining() >= 2 {
            let extensions_len = s.read_u16().ok_or_else(|| {
                OperationError::ProcessingError(
                    "Insufficient data for extensions length.".to_string(),
                )
            })? as usize;
            let extensions_bytes = s.read_bytes(extensions_len).ok_or_else(|| {
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
                let ext_data = es.read_bytes(ext_len).ok_or_else(|| {
                    OperationError::ProcessingError(
                        "Insufficient data for extension data.".to_string(),
                    )
                })?;

                extensions_list.push(format!("{:04x}", ext_type));

                match ext_type {
                    0x0010 => {
                        // alpn
                        let mut as_parser = TlsParser::new(ext_data);
                        if let Some(total_len) = as_parser.read_u16() {
                            if total_len >= 1 {
                                if let Some(first_len) = as_parser.read_u8() {
                                    if let Some(first_alpn) =
                                        as_parser.read_bytes(first_len as usize)
                                    {
                                        alpn = alpn_fingerprint(first_alpn);
                                    }
                                }
                            }
                        }
                    }
                    0x002b => {
                        // supported_versions
                        if ext_data.len() >= 2 {
                            hello_version = u16::from_be_bytes([ext_data[0], ext_data[1]]);
                        }
                    }
                    _ => {}
                }
            }
        }

        let ptype = "t";
        let version = tls_version_mapper(hello_version);
        let ext_len = format!("{:02}", extensions_list.len().min(99));
        let cipher_hex = format!("{:04x}", cipher_suite);

        let extensions_raw = extensions_list.join(",");
        let extensions_hash = &hash_sha256(&extensions_raw)[0..12];

        let ja4s = format!(
            "{}{}{}{}_{}_{}",
            ptype, version, ext_len, alpn, cipher_hex, extensions_hash
        );
        let ja4s_r = format!(
            "{}{}{}{}_{}_{}",
            ptype, version, ext_len, alpn, cipher_hex, extensions_raw
        );

        match output_format {
            "JA4S" => Ok(ja4s.into_bytes()),
            "JA4S Raw" => Ok(ja4s_r.into_bytes()),
            "Both" | _ => {
                let all = format!("JA4S:   {}\nJA4S_r: {}", ja4s, ja4s_r);
                Ok(all.into_bytes())
            }
        }
    }
}

fn tls_version_mapper(version: u16) -> &'static str {
    match version {
        0x0304 => "13", // TLS 1.3
        0x0303 => "12", // TLS 1.2
        0x0302 => "11", // TLS 1.1
        0x0301 => "10", // TLS 1.0
        0x0300 => "s3", // SSL 3.0
        0x0200 => "s2", // SSL 2.0
        0x0100 => "s1", // SSL 1.0
        _ => "00",      // Unknown
    }
}

fn alpn_fingerprint(raw_bytes: &[u8]) -> String {
    if raw_bytes.is_empty() {
        return "00".to_string();
    }
    let first = raw_bytes[0];
    let last = raw_bytes[raw_bytes.len() - 1];
    if is_alphanumeric(first) && is_alphanumeric(last) {
        let mut s = String::new();
        s.push(first as char);
        s.push(last as char);
        s
    } else {
        let first_hex = format!("{:02x}", first);
        let last_hex = format!("{:02x}", last);
        format!("{}{}", &first_hex[0..1], &last_hex[1..2])
    }
}

fn is_alphanumeric(b: u8) -> bool {
    ((0x30..=0x39).contains(&b)) || ((0x41..=0x5a).contains(&b)) || ((0x61..=0x7a).contains(&b))
}

fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
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
