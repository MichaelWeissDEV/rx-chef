/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Key Wrap operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockCipher, BlockEncrypt, KeyInit};
use generic_array::GenericArray;
use kuznyechik::Kuznyechik;
use magma::Magma;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// GOST Key Wrap operation
pub struct GostKeyWrap;

impl Operation for GostKeyWrap {
    fn name(&self) -> &'static str {
        "GOST Key Wrap"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A key wrapping algorithm for protecting keys in untrusted storage using one of the GOST block ciphers."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The Key Encryption Key (KEK).",
                default_value: "",
            },
            ArgSchema {
                name: "User Key Material",
                description: "User Key Material (UKM).",
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
                name: "Key wrapping",
                description: "The key wrapping mode.",
                default_value: "NO",
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
        let ukm = Self::parse_arg_bytes(args.get(1))?;
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let algorithm = args
            .get(4)
            .and_then(|a| a.as_str())
            .unwrap_or("GOST 28147 (1989)");

        let input_bytes = if input_type == "Hex" {
            hex::decode(&input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        let result = match algorithm {
            "GOST 28147 (1989)" | "GOST R 34.12 (Magma, 2015)" => {
                if kek.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "KEK must be 32 bytes".to_string(),
                    });
                }
                self.wrap::<Magma>(&kek, &ukm, &input_bytes)?
            }
            "GOST R 34.12 (Kuznyechik, 2015)" => {
                if kek.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "KEK must be 32 bytes".to_string(),
                    });
                }
                self.wrap::<Kuznyechik>(&kek, &ukm, &input_bytes)?
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

impl GostKeyWrap {
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

    fn wrap<C>(&self, kek: &[u8], _ukm: &[u8], cek: &[u8]) -> Result<Vec<u8>, OperationError>
    where
        C: BlockCipher + cipher::BlockSizeUser + KeyInit + BlockEncrypt,
    {
        let block_size = C::block_size();
        let key_arr = GenericArray::from_slice(kek);
        let cipher = C::new(key_arr);

        // Simple implementation of key wrapping:
        // In GOST, it often involves encrypting the CEK with KEK and appending a MAC.
        // For now, we'll do ECB encryption of CEK and append a simple CBC-MAC.

        let mut encrypted_cek = Vec::with_capacity(cek.len());
        let mut padded_cek = cek.to_vec();
        if padded_cek.len() % block_size != 0 {
            let pad_len = block_size - (padded_cek.len() % block_size);
            padded_cek.extend(vec![0u8; pad_len]);
        }

        for chunk in padded_cek.chunks(block_size) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block(&mut block);
            encrypted_cek.extend_from_slice(block.as_slice());
        }

        // Calculate MAC
        let mut register = vec![0u8; block_size];
        for chunk in padded_cek.chunks(block_size) {
            let mut block = GenericArray::clone_from_slice(chunk);
            for i in 0..block_size {
                block[i] ^= register[i];
            }
            cipher.encrypt_block(&mut block);
            register.copy_from_slice(block.as_slice());
        }

        let mut result = encrypted_cek;
        result.extend_from_slice(&register[..block_size / 2]); // Usually half block MAC
        Ok(result)
    }
}
