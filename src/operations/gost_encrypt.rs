/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit};
use generic_array::GenericArray;
use kuznyechik::Kuznyechik;
use magma::Magma;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// GOST Encrypt operation
pub struct GostEncrypt;

impl Operation for GostEncrypt {
    fn name(&self) -> &'static str {
        "GOST Encrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "The GOST block cipher (Magma), defined in the standard GOST 28147-89 (RFC 5830), is a Soviet and Russian government standard symmetric key block cipher with a block size of 64 bits. The original standard, published in 1989, did not give the cipher any name, but the most recent revision of the standard, GOST R 34.12-2015 (RFC 7801, RFC 8891), specifies that it may be referred to as Magma. The GOST hash function is based on this cipher. The new standard also specifies a new 128-bit block cipher called Kuznyechik."
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
                name: "Block mode",
                description: "The block cipher mode to use.",
                default_value: "ECB",
            },
            ArgSchema {
                name: "Key meshing mode",
                description: "The key meshing mode to use.",
                default_value: "NO",
            },
            ArgSchema {
                name: "Padding",
                description: "The padding scheme to use.",
                default_value: "PKCS5",
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
        let block_mode = args.get(6).and_then(|a| a.as_str()).unwrap_or("ECB");
        let padding = args.get(8).and_then(|a| a.as_str()).unwrap_or("PKCS5");

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
                self.process::<Magma>(&key, &iv, &input_bytes, block_mode, padding, true)?
            }
            "GOST R 34.12 (Kuznyechik, 2015)" => {
                if key.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "Key must be 32 bytes".to_string(),
                    });
                }
                self.process::<Kuznyechik>(&key, &iv, &input_bytes, block_mode, padding, true)?
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

impl GostEncrypt {
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

    fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
        let padding_len = block_size - (data.len() % block_size);
        let mut padded = data.to_vec();
        padded.extend(vec![padding_len as u8; padding_len]);
        padded
    }

    fn process<C>(
        &self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
        mode: &str,
        padding: &str,
        encrypt: bool,
    ) -> Result<Vec<u8>, OperationError>
    where
        C: BlockCipher + cipher::BlockSizeUser + KeyInit + BlockEncrypt + BlockDecrypt,
    {
        let block_size = C::block_size();
        let key_arr = GenericArray::from_slice(key);
        let cipher = C::new(key_arr);

        let data = if padding == "PKCS5" || padding == "PKCS7" {
            Self::pkcs7_pad(input, block_size)
        } else {
            input.to_vec()
        };

        if data.len() % block_size != 0 {
            return Err(OperationError::InvalidArgument {
                name: "Input".to_string(),
                reason: format!("Input length must be a multiple of {} bytes", block_size),
            });
        }

        match mode {
            "ECB" => {
                let mut result = Vec::with_capacity(data.len());
                for chunk in data.chunks(block_size) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    if encrypt {
                        cipher.encrypt_block(&mut block);
                    } else {
                        cipher.decrypt_block(&mut block);
                    }
                    result.extend_from_slice(block.as_slice());
                }
                Ok(result)
            }
            "CBC" => {
                let mut result = Vec::with_capacity(data.len());
                let mut prev_block = if iv.is_empty() {
                    vec![0u8; block_size]
                } else if iv.len() != block_size {
                    return Err(OperationError::InvalidArgument {
                        name: "IV".to_string(),
                        reason: format!("IV must be {} bytes", block_size),
                    });
                } else {
                    iv.to_vec()
                };

                for chunk in data.chunks(block_size) {
                    let mut block = GenericArray::clone_from_slice(chunk);
                    if encrypt {
                        for i in 0..block_size {
                            block[i] ^= prev_block[i];
                        }
                        cipher.encrypt_block(&mut block);
                        prev_block.copy_from_slice(block.as_slice());
                    } else {
                        let next_prev = chunk.to_vec();
                        cipher.decrypt_block(&mut block);
                        for i in 0..block_size {
                            block[i] ^= prev_block[i];
                        }
                        prev_block = next_prev;
                    }
                    result.extend_from_slice(block.as_slice());
                }
                Ok(result)
            }
            _ => Err(OperationError::InvalidArgument {
                name: "Block mode".to_string(),
                reason: format!("Unsupported mode: {}", mode),
            }),
        }
    }
}
