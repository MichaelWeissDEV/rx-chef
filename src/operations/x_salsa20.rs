/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XSalsa20 operation.
 * -----------------------------------------------------------------------------
 */

use salsa20::{
    cipher::{KeyIvInit, StreamCipher, StreamCipherSeek},
    XSalsa12, XSalsa20, XSalsa8,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// XSalsa20 operation
///
/// XSalsa20 is a variant of the Salsa20 stream cipher designed by Daniel J. Bernstein; XSalsa uses longer nonces.
pub struct XSalsa20Op;

impl Operation for XSalsa20Op {
    fn name(&self) -> &'static str {
        "XSalsa20"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "XSalsa is an extended-nonce Salsa stream cipher designed by Daniel J. Bernstein. It uses a 32-byte key, a 24-byte nonce, and a 64-bit block counter. The standard 20-round cipher and reduced-round XSalsa12 and XSalsa8 variants are supported."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Key to use for encryption/decryption",
                default_value: "",
            },
            ArgSchema {
                name: "Nonce",
                description: "Nonce to use",
                default_value: "",
            },
            ArgSchema {
                name: "Counter",
                description: "Starting counter value",
                default_value: "0",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds",
                default_value: "20",
            },
            ArgSchema {
                name: "Input",
                description: "Input format",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output",
                description: "Output format",
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
        let key_bytes =
            Utils::convert_to_byte_array(args.first().ok_or(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Missing".to_string(),
            })?)?;
        let nonce_bytes =
            Utils::convert_to_byte_array(args.get(1).ok_or(OperationError::InvalidArgument {
                name: "Nonce".to_string(),
                reason: "Missing".to_string(),
            })?)?;
        let counter = args.get(2).and_then(|a| a.as_f64()).unwrap_or(0.0) as u64;
        let rounds = args.get(3).and_then(|a| a.as_str()).unwrap_or("20");
        let input_format = args.get(4).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_format = args.get(5).and_then(|a| a.as_str()).unwrap_or("Raw");

        if key_bytes.len() != 32 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes. XSalsa uses a 32-byte key.",
                    key_bytes.len()
                ),
            });
        }

        if nonce_bytes.len() != 24 {
            return Err(OperationError::InvalidArgument {
                name: "Nonce".to_string(),
                reason: format!(
                    "Invalid nonce length: {} bytes. XSalsa20 uses a nonce of 24 bytes.",
                    nonce_bytes.len()
                ),
            });
        }

        let input_data = if input_format == "Hex" {
            hex::decode(String::from_utf8_lossy(&input).trim())
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let mut output_data = input_data;

        match rounds {
            "20" => {
                let mut cipher =
                    XSalsa20::new_from_slices(&key_bytes, &nonce_bytes).map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid key or nonce: {}", e))
                    })?;
                cipher.seek(counter * 64);
                cipher.apply_keystream(&mut output_data);
            }
            "12" => {
                let mut cipher =
                    XSalsa12::new_from_slices(&key_bytes, &nonce_bytes).map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid key or nonce: {e}"))
                    })?;
                cipher.seek(counter * 64);
                cipher.apply_keystream(&mut output_data);
            }
            "8" => {
                let mut cipher =
                    XSalsa8::new_from_slices(&key_bytes, &nonce_bytes).map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid key or nonce: {e}"))
                    })?;
                cipher.seek(counter * 64);
                cipher.apply_keystream(&mut output_data);
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Rounds".to_string(),
                    reason: "Invalid number of rounds. Choose 20, 12, or 8.".to_string(),
                });
            }
        }

        if output_format == "Hex" {
            Ok(hex::encode(output_data).into_bytes())
        } else {
            Ok(output_data)
        }
    }
}
