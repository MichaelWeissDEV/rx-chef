/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JA4 Fingerprint operation.
 * -----------------------------------------------------------------------------
 */

use sha2::{Digest, Sha256};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JA4 Fingerprint operation
pub struct JA4Fingerprint;

impl Operation for JA4Fingerprint {
    fn name(&self) -> &'static str {
        "JA4 Fingerprint"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates a JA4 fingerprint to help identify TLS clients based on hashing together values from the Client Hello.<br><br>Input: A hex stream of the TLS or QUIC Client Hello packet application layer."
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
                default_value: "JA4",
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
        let output_format = args.get(1).and_then(|v| v.as_str()).unwrap_or("JA4");

        let bytes = convert_to_byte_array(input, input_format)?;
        let mut s = TlsParser::new(&bytes);

        // Record Header
        if s.read_u8() != Some(0x16) {
            return Err(OperationError::ProcessingError(
                "Not handshake data.".to_string(),
            ));
        }
        s.skip(2); // Version
        let length = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for length.".to_string())
        })?;
        if s.remaining() < length as usize {
            // return Err(OperationError::ProcessingError("Incorrect handshake length.".to_string()));
        }

        // Handshake
        if s.read_u8() != Some(1) {
            return Err(OperationError::ProcessingError(
                "Not a Client Hello.".to_string(),
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

        let cipher_suites_len = s.read_u16().ok_or_else(|| {
            OperationError::ProcessingError(
                "Insufficient data for cipher suites length.".to_string(),
            )
        })? as usize;
        let cipher_suites_bytes = s.read_bytes(cipher_suites_len).ok_or_else(|| {
            OperationError::ProcessingError("Insufficient data for cipher suites.".to_string())
        })?;

        let mut cs_parser = TlsParser::new(cipher_suites_bytes);
        let mut original_ciphers = Vec::new();
        while cs_parser.has_more() {
            let cs = cs_parser.read_u16().unwrap_or(0);
            if !GREASE_VALUES.contains(&cs) {
                original_ciphers.push(format!("{:04x}", cs));
            }
        }

        let comp_methods_len = s.read_u8().ok_or_else(|| {
            OperationError::ProcessingError(
                "Insufficient data for compression methods length.".to_string(),
            )
        })? as usize;
        s.skip(comp_methods_len);

        let mut sni = "i";
        let mut alpn = "00".to_string();
        let mut original_extensions = Vec::new();
        let mut signature_algorithms = String::new();

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

                if !GREASE_VALUES.contains(&ext_type) {
                    original_extensions.push(format!("{:04x}", ext_type));
                }

                match ext_type {
                    0x0000 => sni = "d", // server_name
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
                    0x000d => {
                        // signature_algorithms
                        if ext_data.len() > 2 {
                            let mut sig_parser = TlsParser::new(&ext_data[2..]);
                            let mut sigs = Vec::new();
                            while sig_parser.has_more() {
                                let sig = sig_parser.read_u16().unwrap_or(0);
                                sigs.push(format!("{:04x}", sig));
                            }
                            signature_algorithms = sigs.join(",");
                        }
                    }
                    0x002b => {
                        // supported_versions
                        let mut sv_parser = TlsParser::new(ext_data);
                        if let Some(len) = sv_parser.read_u8() {
                            let mut highest = 0;
                            let mut sv_bytes_read = 0;
                            while sv_parser.has_more() && sv_bytes_read < len as usize {
                                let v = sv_parser.read_u16().unwrap_or(0);
                                sv_bytes_read += 2;
                                if !GREASE_VALUES.contains(&v) && v > highest {
                                    highest = v;
                                }
                            }
                            if highest > 0 {
                                hello_version = highest;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        let ptype = "t";
        let version = tls_version_mapper(hello_version);
        let cipher_len = format!("{:02}", original_ciphers.len().min(99));
        let ext_len = format!("{:02}", original_extensions.len().min(99));

        let mut sorted_ciphers = original_ciphers.clone();
        sorted_ciphers.sort();
        let sorted_ciphers_raw = sorted_ciphers.join(",");
        let original_ciphers_raw = original_ciphers.join(",");

        let sorted_ciphers_hash = &hash_sha256(&sorted_ciphers_raw)[0..12];
        let original_ciphers_hash = &hash_sha256(&original_ciphers_raw)[0..12];

        let mut sorted_extensions: Vec<String> = original_extensions
            .iter()
            .filter(|&e| e != "0000" && e != "0010")
            .cloned()
            .collect();
        sorted_extensions.sort();
        let sorted_extensions_raw =
            format!("{}_{}", sorted_extensions.join(","), signature_algorithms);
        let original_extensions_raw =
            format!("{}_{}", original_extensions.join(","), signature_algorithms);

        let sorted_extensions_hash = &hash_sha256(&sorted_extensions_raw)[0..12];
        let original_extensions_hash = &hash_sha256(&original_extensions_raw)[0..12];

        let ja4 = format!(
            "{}{}{}{}{}{}_{}_{}",
            ptype,
            version,
            sni,
            cipher_len,
            ext_len,
            alpn,
            sorted_ciphers_hash,
            sorted_extensions_hash
        );
        let ja4_o = format!(
            "{}{}{}{}{}{}_{}_{}",
            ptype,
            version,
            sni,
            cipher_len,
            ext_len,
            alpn,
            original_ciphers_hash,
            original_extensions_hash
        );
        let ja4_r = format!(
            "{}{}{}{}{}{}_{}_{}",
            ptype,
            version,
            sni,
            cipher_len,
            ext_len,
            alpn,
            sorted_ciphers_raw,
            sorted_extensions_raw
        );
        let ja4_ro = format!(
            "{}{}{}{}{}{}_{}_{}",
            ptype,
            version,
            sni,
            cipher_len,
            ext_len,
            alpn,
            original_ciphers_raw,
            original_extensions_raw
        );

        match output_format {
            "JA4" => Ok(ja4.into_bytes()),
            "JA4 Original Rendering" => Ok(ja4_o.into_bytes()),
            "JA4 Raw" => Ok(ja4_r.into_bytes()),
            "JA4 Raw Original Rendering" => Ok(ja4_ro.into_bytes()),
            "All" | _ => {
                let all = format!(
                    "JA4:    {}\nJA4_o:  {}\nJA4_r:  {}\nJA4_ro: {}",
                    ja4, ja4_o, ja4_r, ja4_ro
                );
                Ok(all.into_bytes())
            }
        }
    }
}

const GREASE_VALUES: [u16; 16] = [
    0x0a0a, 0x1a1a, 0x2a2a, 0x3a3a, 0x4a4a, 0x5a5a, 0x6a6a, 0x7a7a, 0x8a8a, 0x9a9a, 0xaaaa, 0xbaba,
    0xcaca, 0xdada, 0xeaea, 0xfafa,
];

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
