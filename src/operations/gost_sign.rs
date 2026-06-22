/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Sign operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockCipher, BlockEncrypt, KeyInit};
use generic_array::GenericArray;
use kuznyechik::Kuznyechik;
use magma::Magma;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// GOST Sign operation
pub struct GostSign;

impl Operation for GostSign {
    fn name(&self) -> &'static str {
        "GOST Sign"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Sign a plaintext message (calculate MAC) using one of the GOST block ciphers."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The encryption key.",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "The initialization vector.",
                default_value: "",
            },
            ArgSchema {
                name: "Input type",
                description: "Input encoding (Raw, Hex)",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output type",
                description: "Output encoding (Hex, Raw)",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Algorithm",
                description: "The GOST algorithm to use.",
                default_value: "GOST 28147 (1989)",
            },
            ArgSchema {
                name: "sBox",
                description: "The sBox to use (only for GOST 28147 (1989)).",
                default_value: "E-TEST",
            },
            ArgSchema {
                name: "MAC length",
                description: "The length of the MAC in bits.",
                default_value: "32",
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
        let key = Self::parse_arg_bytes(args.first())?;
        let iv = Self::parse_arg_bytes(args.get(1))?;
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let algorithm = args
            .get(4)
            .and_then(|a| a.as_str())
            .unwrap_or("GOST 28147 (1989)");
        let mac_length_bits = args.get(6).and_then(|a| a.as_str()).unwrap_or("32");
        let mac_length = mac_length_bits.parse::<usize>().unwrap_or(32) / 8;

        let input_bytes = if input_type == "Hex" {
            hex::decode(&input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        let result = match algorithm {
            "GOST 28147 (1989)" | "GOST R 34.12 (Magma, 2015)" => {
                if key.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "Key must be 32 bytes".to_string(),
                    });
                }
                self.calculate_mac::<Magma>(&key, &iv, &input_bytes, mac_length)?
            }
            "GOST R 34.12 (Kuznyechik, 2015)" => {
                if key.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "Key must be 32 bytes".to_string(),
                    });
                }
                self.calculate_mac::<Kuznyechik>(&key, &iv, &input_bytes, mac_length)?
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Algorithm".to_string(),
                    reason: format!("Unsupported algorithm: {}", algorithm),
                })
            }
        };

        let output_bytes = if output_type == "Hex" {
            hex::encode(result).into_bytes()
        } else {
            result
        };

        Ok(output_bytes)
    }
}

impl GostSign {
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

    fn calculate_mac<C>(
        &self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
        mac_length: usize,
    ) -> Result<Vec<u8>, OperationError>
    where
        C: BlockCipher + cipher::BlockSizeUser + KeyInit + BlockEncrypt,
    {
        let block_size = C::block_size();
        let key_arr = GenericArray::from_slice(key);
        let cipher = C::new(key_arr);

        let mut register = if iv.is_empty() {
            vec![0u8; block_size]
        } else if iv.len() != block_size {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!("IV must be {} bytes", block_size),
            });
        } else {
            iv.to_vec()
        };

        // GOST MAC (Imitovstavka)
        // For each block: register = Encrypt(register XOR block)
        // If the last block is partial, it is zero-padded.
        let mut padded_input = input.to_vec();
        if padded_input.is_empty() {
            // If input is empty, return zeros or handle accordingly.
            // Usually MAC of empty input is Encrypt(0) or similar.
            padded_input.resize(block_size, 0);
        } else if padded_input.len() % block_size != 0 {
            let pad_len = block_size - (padded_input.len() % block_size);
            padded_input.extend(vec![0u8; pad_len]);
        }

        for chunk in padded_input.chunks(block_size) {
            let mut block = GenericArray::clone_from_slice(chunk);
            for i in 0..block_size {
                block[i] ^= register[i];
            }
            cipher.encrypt_block(&mut block);
            register.copy_from_slice(block.as_slice());
        }

        let mut mac = register;
        mac.truncate(mac_length);
        Ok(mac)
    }
}
