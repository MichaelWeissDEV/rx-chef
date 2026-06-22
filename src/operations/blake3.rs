/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the BLAKE3 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// BLAKE3 operation
///
/// Hashes the input using BLAKE3 (UTF-8 encoded), with an optional key
/// (also UTF-8), and outputs the result in hexadecimal format.
pub struct BLAKE3;

impl Operation for BLAKE3 {
    fn name(&self) -> &'static str {
        "BLAKE3"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Hashes the input using BLAKE3 (UTF-8 encoded), with an optional key (also UTF-8), and outputs the result in hexadecimal format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Size (bytes)",
                description: "Output size in bytes",
                default_value: "32",
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
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let output_size = args.first().and_then(|a| a.as_i64()).unwrap_or(32) as usize;
        let key = args.get(1).and_then(|a| a.as_str()).unwrap_or("");

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let key_bytes = if key.is_empty() {
            None
        } else {
            Some(key.as_bytes().to_vec())
        };

        // Validate key length if provided
        if let Some(ref k) = key_bytes {
            if k.len() != 32 {
                return Err(OperationError::InvalidInput(
                    "The key must be exactly 32 bytes long".to_string(),
                ));
            }
        }

        // Compute hash
        let mut output = vec![0u8; output_size];

        match key_bytes {
            Some(k) => {
                // Keyed hash: blake3::keyed_hash expects a 32-byte array and input, returns a hash
                let mut key_arr = [0u8; 32];
                key_arr.copy_from_slice(&k[..32]);
                let hash = blake3::keyed_hash(&key_arr, input_str.as_bytes());
                let hash_bytes = hash.as_bytes();
                let copy_len = output_size.min(hash_bytes.len());
                output[..copy_len].copy_from_slice(&hash_bytes[..copy_len]);
            }
            None => {
                // Standard hash
                let hash = blake3::hash(input_str.as_bytes());
                let hash_bytes = hash.as_bytes();
                let copy_len = output_size.min(hash_bytes.len());
                output[..copy_len].copy_from_slice(&hash_bytes[..copy_len]);
            }
        }

        let hex_output = hex::encode(&output);

        Ok(hex_output.into_bytes())
    }
}
