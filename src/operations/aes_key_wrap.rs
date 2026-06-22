/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AES Key Wrap operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AES Key Wrap operation
///
/// A key wrapping algorithm defined in RFC3394, which is used to protect keys
/// in untrusted storage or communications, using AES.
pub struct AesKeyWrap;

impl Operation for AesKeyWrap {
    fn name(&self) -> &'static str {
        "AES Key Wrap"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A key wrapping algorithm defined in RFC3394, which is used to protect keys in untrusted storage or communications, using AES."
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

        // Validate input length: must be 8n bytes where n >= 2 (at least 16 bytes)
        if input_data.len() % 8 != 0 || input_data.len() < 16 {
            return Err(OperationError::InvalidArgument {
                name: "Input".to_string(),
                reason: format!(
                    "Input must be 8n bytes (n>=2) (currently {} bytes)",
                    input_data.len()
                ),
            });
        }

        // Use the aes crate with ECB mode
        // A = IV (6A6A6A6A6A6A6A6A)
        let mut a: [u8; 8] =
            iv.clone()
                .try_into()
                .map_err(|_| OperationError::InvalidArgument {
                    name: "IV".to_string(),
                    reason: "IV must be 8 bytes".to_string(),
                })?;

        let n = input_data.len() / 8; // Number of 8-byte blocks
        let mut r: Vec<[u8; 8]> = input_data
            .chunks_exact(8)
            .map(|chunk| {
                chunk
                    .try_into()
                    .map_err(|_| OperationError::ProcessingError("Invalid chunk size".to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // RFC3394: 6 iterations (j = 0..5)
        for j in 0..6 {
            for i in 0..n {
                // Counter = (n * j) + (i + 1)
                // i is 0-indexed, so we use (i + 1) for 1-indexed counter
                let counter: u32 = ((n as u32) * (j as u32)) + (i as u32) + 1;

                // Create combined block: A || R[i] (16 bytes)
                let mut combined = [0u8; 16];
                combined[..8].copy_from_slice(&a);
                combined[8..].copy_from_slice(&r[i]);

                // Encrypt with AES-ECB
                let result = Self::process_ecb(&kek, &combined, true)?;

                // A = MSB(Encipher(K, A || R[i])) XOR counter - first 8 bytes XORed with counter
                let mut a_block: [u8; 8] = result[0..8].try_into().unwrap();
                // XOR counter into all 8 bytes (big-endian, 64-bit)
                let counter_u64 = counter as u64;
                for idx in 0..8 {
                    a_block[idx] ^= ((counter_u64 >> ((7 - idx) * 8)) & 0xff) as u8;
                }
                a = a_block;

                // R[i] = LSB(Encipher(K, A || R[i])) - last 8 bytes unchanged
                r[i] = result[8..16].try_into().unwrap();
            }
        }

        // Combine A || R[0] || R[1] || ...
        let mut result = Vec::with_capacity(input_data.len() + 8);
        result.extend_from_slice(&a);
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

impl AesKeyWrap {
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

    /// Process ECB mode encryption/decryption
    /// Input is padded to block size (16 bytes) if needed
    fn process_ecb(key: &[u8], input: &[u8], encrypt: bool) -> Result<Vec<u8>, OperationError> {
        // Pad input to block size if needed
        let block_size = 16; // AES block size is always 16 bytes
        let padded_len = ((input.len() + block_size - 1) / block_size) * block_size;

        let mut result = Vec::with_capacity(padded_len);
        let mut padded_input = input.to_vec();
        padded_input.resize(padded_len, 0u8);

        match key.len() {
            16 => {
                use aes::Aes128;
                use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
                use generic_array::GenericArray;
                let key_arr = GenericArray::from_slice(key);
                let cipher = Aes128::new(key_arr);
                for chunk in padded_input.chunks(block_size) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    if encrypt {
                        cipher.encrypt_block(&mut block);
                    } else {
                        cipher.decrypt_block(&mut block);
                    }
                    result.extend_from_slice(block.as_slice());
                }
            }
            24 => {
                use aes::Aes192;
                use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
                use generic_array::GenericArray;
                let key_arr = GenericArray::from_slice(key);
                let cipher = Aes192::new(key_arr);
                for chunk in padded_input.chunks(block_size) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    if encrypt {
                        cipher.encrypt_block(&mut block);
                    } else {
                        cipher.decrypt_block(&mut block);
                    }
                    result.extend_from_slice(block.as_slice());
                }
            }
            32 => {
                use aes::Aes256;
                use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
                use generic_array::GenericArray;
                let key_arr = GenericArray::from_slice(key);
                let cipher = Aes256::new(key_arr);
                for chunk in padded_input.chunks(block_size) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    if encrypt {
                        cipher.encrypt_block(&mut block);
                    } else {
                        cipher.decrypt_block(&mut block);
                    }
                    result.extend_from_slice(block.as_slice());
                }
            }
            _ => {
                return Err(OperationError::ProcessingError(
                    "Invalid KEK length".to_string(),
                ))
            }
        }

        Ok(result)
    }
}
