/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Salsa20 operation.
 * -----------------------------------------------------------------------------
 */

use salsa20::{
    cipher::{KeyIvInit, StreamCipher, StreamCipherSeek},
    Salsa12, Salsa20, Salsa8,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// Salsa20 operation
pub struct Salsa20Op;

impl Operation for Salsa20Op {
    fn name(&self) -> &'static str {
        "Salsa20"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Salsa20 is a stream cipher designed by Daniel J. Bernstein and submitted to the eSTREAM project; Salsa20/8 and Salsa20/12 are round-reduced variants. It is closely related to the ChaCha stream cipher.<br><br><b>Key:</b> Salsa20 uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> Salsa20 uses a nonce of 8 bytes (64 bits).<br><br><b>Counter:</b> Salsa uses a counter of 8 bytes (64 bits). The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Key (16 or 32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Nonce",
                description: "Nonce (8 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Counter",
                description: "Initial counter value",
                default_value: "0",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds (20, 12, or 8)",
                default_value: "20",
            },
            ArgSchema {
                name: "Input",
                description: "Input format (Raw, Hex)",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output",
                description: "Output format (Raw, Hex)",
                default_value: "Raw",
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
        let key_bytes = args
            .get(0)
            .map(Utils::convert_to_byte_array)
            .transpose()?
            .unwrap_or_default();
        let nonce_bytes = args
            .get(1)
            .map(Utils::convert_to_byte_array)
            .transpose()?
            .unwrap_or_default();
        let counter = args.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0) as u64;
        let rounds = args.get(3).and_then(|v| v.as_str()).unwrap_or("20");
        let input_format = args.get(4).and_then(|v| v.as_str()).unwrap_or("Raw");
        let output_format = args.get(5).and_then(|v| v.as_str()).unwrap_or("Raw");

        if key_bytes.len() != 16 && key_bytes.len() != 32 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes. Salsa20 uses 16 or 32 bytes.",
                    key_bytes.len()
                ),
            });
        }
        if nonce_bytes.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "Nonce".to_string(),
                reason: format!(
                    "Invalid nonce length: {} bytes. Salsa20 uses 8 bytes.",
                    nonce_bytes.len()
                ),
            });
        }

        let mut data = if input_format == "Hex" {
            hex::decode(
                String::from_utf8_lossy(&input)
                    .trim()
                    .replace(' ', "")
                    .replace('\n', ""),
            )
            .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let final_key = if key_bytes.len() == 16 {
            let mut k = Vec::with_capacity(32);
            k.extend_from_slice(&key_bytes);
            k.extend_from_slice(&key_bytes);
            k
        } else {
            key_bytes
        };

        match rounds {
            "20" => {
                let mut cipher =
                    Salsa20::new_from_slices(&final_key, &nonce_bytes).map_err(|e| {
                        OperationError::ProcessingError(format!("Salsa20 init failed: {}", e))
                    })?;
                cipher.seek(counter * 64);
                cipher.apply_keystream(&mut data);
            }
            "12" => {
                let mut cipher =
                    Salsa12::new_from_slices(&final_key, &nonce_bytes).map_err(|e| {
                        OperationError::ProcessingError(format!("Salsa12 init failed: {}", e))
                    })?;
                cipher.seek(counter * 64);
                cipher.apply_keystream(&mut data);
            }
            "8" => {
                let mut cipher =
                    Salsa8::new_from_slices(&final_key, &nonce_bytes).map_err(|e| {
                        OperationError::ProcessingError(format!("Salsa8 init failed: {}", e))
                    })?;
                cipher.seek(counter * 64);
                cipher.apply_keystream(&mut data);
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Rounds".to_string(),
                    reason: format!("Unsupported rounds: {}", rounds),
                })
            }
        }

        if output_format == "Hex" {
            Ok(hex::encode(data).into_bytes())
        } else {
            Ok(data)
        }
    }
}
