/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AES Key Unwrap operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AES Key Unwrap operation
///
/// Decryptor for a key wrapping algorithm defined in RFC3394, which is used to
/// protect keys in untrusted storage or communications, using AES.
pub struct AesKeyUnwrap;

impl Operation for AesKeyUnwrap {
    fn name(&self) -> &'static str {
        "AES Key Unwrap"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Decryptor for a key wrapping algorithm defined in RFC3394, which is used to protect keys in untrusted storage or communications, using AES."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key (KEK)",
                description: "Key-encryption key (16, 24, or 32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (8 bytes, defaults to a6a6a6a6a6a6a6a6)",
                default_value: "a6a6a6a6a6a6a6a6",
            },
            ArgSchema {
                name: "Input",
                description: "Input encoding (Raw, Hex)",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output",
                description: "Output encoding (Raw, Hex)",
                default_value: "Hex",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let kek = Self::parse_arg_bytes(args.first())?;
        let iv = Self::parse_arg_bytes(args.get(1))?;
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");

        // Validate KEK length
        if ![16, 24, 32].contains(&kek.len()) {
            return Err(OperationError::InvalidArgument {
                name: "Key (KEK)".to_string(),
                reason: format!(
                    "KEK must be 16, 24, or 32 bytes (currently {} bytes)",
                    kek.len()
                ),
            });
        }

        // Validate IV length
        if iv.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!("IV must be 8 bytes (currently {} bytes)", iv.len()),
            });
        }

        // Parse input
        let input_data = if input_type == "Hex" {
            hex::decode(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        // Validate input length: must be 8n bytes where n >= 3 (at least 24 bytes for unwrap)
        if input_data.len() % 8 != 0 || input_data.len() < 24 {
            return Err(OperationError::InvalidArgument {
                name: "Input".to_string(),
                reason: format!(
                    "Input must be 8n bytes (n>=3) (currently {} bytes)",
                    input_data.len()
                ),
            });
        }

        // Use the aes crate with ECB mode
        // A = first 8 bytes of input
        let mut a: [u8; 8] = input_data[0..8]
            .try_into()
            .map_err(|_| OperationError::ProcessingError("Invalid A size".to_string()))?;

        let n = (input_data.len() - 8) / 8; // Number of R blocks
        let mut r: Vec<[u8; 8]> = input_data[8..]
            .chunks_exact(8)
            .map(|chunk| {
                chunk
                    .try_into()
                    .map_err(|_| OperationError::ProcessingError("Invalid chunk size".to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Calculate initial counter value
        // For RFC3394, the counter starts at n*6 and counts down
        // RFC3394 reverse: 6 iterations from 5 down to 0
        for j in (0..6).rev() {
            for i in (0..n).rev() {
                // Counter = (n * j) + (i + 1)
                let counter: u32 = ((n as u32) * (j as u32)) + (i as u32) + 1;

                // XOR counter with A (all 8 bytes, big-endian, 64-bit)
                let mut a_block: [u8; 8] = a;
                let counter_u64 = counter as u64;
                for idx in 0..8 {
                    a_block[idx] ^= ((counter_u64 >> ((7 - idx) * 8)) & 0xff) as u8;
                }

                // Create combined block: A || R[i] (16 bytes for ECB decryption)
                let mut combined = [0u8; 16];
                combined[..8].copy_from_slice(&a_block);
                combined[8..].copy_from_slice(&r[i]);

                // Decrypt with AES-ECB
                let result = match kek.len() {
                    16 => {
                        use aes::Aes128;
                        use cipher::{BlockDecrypt, KeyInit};
                        use generic_array::GenericArray;
                        let key_arr = GenericArray::from_slice(&kek);
                        let cipher = Aes128::new(key_arr);
                        let mut block = GenericArray::clone_from_slice(&combined);
                        cipher.decrypt_block(&mut block);
                        block.to_vec()
                    }
                    24 => {
                        use aes::Aes192;
                        use cipher::{BlockDecrypt, KeyInit};
                        use generic_array::GenericArray;
                        let key_arr = GenericArray::from_slice(&kek);
                        let cipher = Aes192::new(key_arr);
                        let mut block = GenericArray::clone_from_slice(&combined);
                        cipher.decrypt_block(&mut block);
                        block.to_vec()
                    }
                    32 => {
                        use aes::Aes256;
                        use cipher::{BlockDecrypt, KeyInit};
                        use generic_array::GenericArray;
                        let key_arr = GenericArray::from_slice(&kek);
                        let cipher = Aes256::new(key_arr);
                        let mut block = GenericArray::clone_from_slice(&combined);
                        cipher.decrypt_block(&mut block);
                        block.to_vec()
                    }
                    _ => {
                        return Err(OperationError::ProcessingError(
                            "Invalid KEK length".to_string(),
                        ))
                    }
                };

                a = result[0..8].try_into().unwrap();
                r[i] = result[8..16].try_into().unwrap();
            }
        }

        // Verify IV matches
        let iv_array: [u8; 8] = iv
            .try_into()
            .map_err(|_| OperationError::ProcessingError("Invalid IV size".to_string()))?;
        if a != iv_array {
            return Err(OperationError::ProcessingError(format!(
                "IV mismatch: expected {:?}, got {:?}",
                iv_array, a
            )));
        }

        // Combine R blocks to get the original key
        let mut result = Vec::with_capacity(r.len() * 8);
        for block in &r {
            result.extend_from_slice(block);
        }

        let output_bytes = if output_type == "Hex" {
            hex::encode(result).into_bytes()
        } else {
            result
        };

        Ok(output_bytes)
    }
}

impl AesKeyUnwrap {
    /// Parse an ArgValue as bytes (supports Hex and UTF8)
    fn parse_arg_bytes(arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else if s.starts_with("0x") {
                    hex::decode(&s[2..]).map_err(|e| OperationError::InvalidArgument {
                        name: "Argument".to_string(),
                        reason: e.to_string(),
                    })
                } else {
                    Ok(s.as_bytes().to_vec())
                }
            }
            Some(ArgValue::Bytes(b)) => Ok(b.clone()),
            _ => Ok(vec![]),
        }
    }
}
