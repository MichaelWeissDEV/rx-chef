/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the BLAKE2b operation.
 * -----------------------------------------------------------------------------
 */

use blake2::{
    digest::{Digest, Mac},
    Blake2b512, Blake2bMac512,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// BLAKE2b operation
///
/// BLAKE2b is a flavour of the BLAKE cryptographic hash function that is
/// optimized for 64-bit platforms and produces digests of any size between
/// 1 and 64 bytes. Supports the use of an optional key.
pub struct BLAKE2b;

impl Operation for BLAKE2b {
    fn name(&self) -> &'static str {
        "BLAKE2b"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Performs BLAKE2b hashing on the input. BLAKE2b is a flavour of the BLAKE cryptographic hash function that is optimized for 64-bit platforms and produces digests of any size between 1 and 64 bytes. Supports the use of an optional key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Size",
                description: "Output size in bits (512, 384, 256, 160, 128)",
                default_value: "512",
            },
            ArgSchema {
                name: "Output Encoding",
                description: "Output encoding (Hex, Base64, Raw)",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Key",
                description: "Optional key for keyed hashing",
                default_value: "",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let size_bits = args
            .get(0)
            .and_then(|a| a.as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(512) as usize;
        let output_format = args.get(1).and_then(|a| a.as_str()).unwrap_or("Hex");
        let key = args.get(2).and_then(|a| a.as_str()).unwrap_or("");

        // Validate size
        if ![512, 384, 256, 160, 128].contains(&size_bits) {
            return Err(OperationError::InvalidInput(format!(
                "Invalid size: {}. Must be 512, 384, 256, 160, or 128",
                size_bits
            )));
        }

        let key_bytes = if key.is_empty() {
            None
        } else {
            Some(key.as_bytes().to_vec())
        };

        // Validate key length
        if let Some(ref k) = key_bytes {
            if k.len() > 64 {
                return Err(OperationError::InvalidInput(format!(
                    "Key cannot be greater than 64 bytes. Currently {} bytes.",
                    k.len()
                )));
            }
        }

        let mut hash = [0u8; 64]; // Max BLAKE2b output size

        let hash_len = if let Some(ref k) = key_bytes {
            // Keyed hashing using Blake2bMac
            let mut mac = Blake2bMac512::new_from_slice(k)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid key: {}", e)))?;
            mac.update(&input);
            let result = mac.finalize();
            hash[..64].copy_from_slice(&result.into_bytes());
            size_bits / 8
        } else {
            // Standard hashing
            let mut hasher = Blake2b512::new();
            hasher.update(&input);
            let result = hasher.finalize();
            hash[..64].copy_from_slice(&result);
            size_bits / 8
        };

        let hash_bytes = &hash[..hash_len];

        let output = match output_format {
            "Hex" => hex::encode(hash_bytes),
            "Base64" => data_encoding::BASE64.encode(hash_bytes),
            "Raw" => {
                let mut result = String::new();
                for &b in hash_bytes {
                    if b.is_ascii_graphic() || b == b' ' {
                        result.push(b as char);
                    } else {
                        result.push('?');
                    }
                }
                result
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported output format: {}",
                    output_format
                )));
            }
        };

        Ok(output.into_bytes())
    }
}
