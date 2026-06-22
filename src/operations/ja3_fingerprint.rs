/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JA3 Fingerprint operation.
 * -----------------------------------------------------------------------------
 */

use md5::{Digest, Md5};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JA3 Fingerprint operation
pub struct JA3Fingerprint;

impl Operation for JA3Fingerprint {
    fn name(&self) -> &'static str {
        "JA3 Fingerprint"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates a JA3 fingerprint to help identify TLS clients based on hashing together values from the Client Hello.<br><br>Input: A hex stream of the TLS Client Hello packet application layer."
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
        let length = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for length.".to_string())
        })?;
        if s.remaining() < length as usize {
            // CyberChef might be more lenient, but let's be strict for now or adjust if needed.
            // Actually, some packets might just be the handshake message without the record header.
            // But the code explicitly checks for 0x16.
        }

        // Handshake type
        if s.read_u8() != Some(1) {
            return Err(OperationError::ProcessingError(
                "Not a Client Hello.".to_string(),
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

        // Cipher suites
        let cipher_suites_length = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError(
                "Insufficient data for cipher suites length.".to_string(),
            )
        })? as usize;
        let cipher_suites_bytes = s.read_bytes(cipher_suites_length).ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for cipher suites.".to_string())
        })?;
        let cipher_segment = parse_ja3_segment(cipher_suites_bytes, 2);

        // Compression Methods
        let compression_methods_length = s.read_u8().ok_or_else(|| {
            OperationError::ProcessingError(
                "Insufficient data for compression methods length.".to_string(),
            )
        })? as usize;
        s.skip(compression_methods_length);

        // Extensions
        if s.remaining() < 2 {
            // No extensions
        } else {
            let extensions_length = s.read_u16().ok_or_else(|| {
                OperationError::ProcessingError(
                    "Insufficient data for extensions length.".to_string(),
                )
            })? as usize;
            let extensions_bytes = s.read_bytes(extensions_length).ok_or_else(|| {
                OperationError::ProcessingError("Insufficient data for extensions.".to_string())
            })?;

            let mut es = TlsParser::new(extensions_bytes);
            let mut elliptic_curves = String::new();
            let mut elliptic_curve_point_formats = String::new();
            let mut exts = Vec::new();

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

                match ext_type {
                    0x000a => {
                        // Elliptic curves (supported_groups)
                        let mut ecs = TlsParser::new(ext_data);
                        if let Some(len) = ecs.read_u16() {
                            if let Some(data) = ecs.read_bytes(len as usize) {
                                elliptic_curves = parse_ja3_segment(data, 2);
                            }
                        }
                    }
                    0x000b => {
                        // Elliptic curve point formats (ec_point_formats)
                        let mut ecs = TlsParser::new(ext_data);
                        if let Some(len) = ecs.read_u8() {
                            if let Some(data) = ecs.read_bytes(len as usize) {
                                elliptic_curve_point_formats = parse_ja3_segment(data, 1);
                            }
                        }
                    }
                    _ => {}
                }

                if !GREASE_VALUES.contains(&ext_type) {
                    exts.push(ext_type.to_string());
                }
            }

            let ja3_str = format!(
                "{},{},{},{},{}",
                hello_version,
                cipher_segment,
                exts.join("-"),
                elliptic_curves,
                elliptic_curve_point_formats
            );

            let mut hasher = Md5::new();
            hasher.update(ja3_str.as_bytes());
            let ja3_hash = hex::encode(hasher.finalize());

            return match output_format {
                "JA3 string" => Ok(ja3_str.into_bytes()),
                "Full details" => {
                    let details = format!("Hash digest:\n{}\n\nFull JA3 string:\n{}\n\nTLS Version:\n{}\nCipher Suites:\n{}\nExtensions:\n{}\nElliptic Curves:\n{}\nElliptic Curve Point Formats:\n{}",
                        ja3_hash, ja3_str, hello_version, cipher_segment, exts.join("-"), elliptic_curves, elliptic_curve_point_formats);
                    Ok(details.into_bytes())
                }
                "Hash digest" | _ => Ok(ja3_hash.into_bytes()),
            };
        }

        // Fallback for no extensions
        let ja3_str = format!("{},{},,,,", hello_version, cipher_segment);
        let mut hasher = Md5::new();
        hasher.update(ja3_str.as_bytes());
        let ja3_hash = hex::encode(hasher.finalize());

        match output_format {
            "JA3 string" => Ok(ja3_str.into_bytes()),
            "Full details" => {
                let details = format!("Hash digest:\n{}\n\nFull JA3 string:\n{}\n\nTLS Version:\n{}\nCipher Suites:\n{}\nExtensions:\n{}\nElliptic Curves:\n{}\nElliptic Curve Point Formats:\n{}",
                    ja3_hash, ja3_str, hello_version, cipher_segment, "", "", "");
                Ok(details.into_bytes())
            }
            "Hash digest" | _ => Ok(ja3_hash.into_bytes()),
        }
    }
}

const GREASE_VALUES: [u16; 16] = [
    0x0a0a, 0x1a1a, 0x2a2a, 0x3a3a, 0x4a4a, 0x5a5a, 0x6a6a, 0x7a7a, 0x8a8a, 0x9a9a, 0xaaaa, 0xbaba,
    0xcaca, 0xdada, 0xeaea, 0xfafa,
];

fn parse_ja3_segment(data: &[u8], size: usize) -> String {
    let mut s = TlsParser::new(data);
    let mut segment = Vec::new();
    while s.has_more() {
        let element = if size == 2 {
            s.read_u16().unwrap_or(0) as u32
        } else {
            s.read_u8().unwrap_or(0) as u32
        };
        if size == 2 {
            if !GREASE_VALUES.contains(&(element as u16)) {
                segment.push(element.to_string());
            }
        } else {
            segment.push(element.to_string());
        }
    }
    segment.join("-")
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
