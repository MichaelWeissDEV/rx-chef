/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the DES Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use generic_array::GenericArray;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// DES Decrypt operation
pub struct DesDecrypt;

impl Operation for DesDecrypt {
    fn name(&self) -> &'static str {
        "DES Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "DES is a previously dominant algorithm for encryption, and was published as an official U.S. Federal Information Processing Standard (FIPS). It is now considered to be insecure due to its small key size.<br><br><b>Key:</b> DES uses a key length of 8 bytes (64 bits).<br><br><b>IV:</b> The Initialization Vector should be 8 bytes long. If not entered, it will default to 8 null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, PKCS#7 padding will be used as a default."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Decryption key (8 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (8 bytes, optional, defaults to null)",
                default_value: "",
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode (CBC, CFB, OFB, CTR, ECB, CBC/NoPadding, ECB/NoPadding)",
                default_value: "CBC",
            },
            ArgSchema {
                name: "Input",
                description: "Input encoding (Hex, Raw)",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output",
                description: "Output encoding (Raw, Hex)",
                default_value: "Raw",
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
        let mode_full = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Raw");

        if key.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes\n\nDES uses a key length of 8 bytes (64 bits).",
                    key.len()
                ),
            });
        }

        let mode = mode_full.split('/').next().unwrap_or("CBC");

        let mut actual_iv = iv.clone();
        if actual_iv.is_empty() && mode != "ECB" {
            actual_iv = vec![0u8; 8];
        } else if actual_iv.len() != 8 && mode != "ECB" {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!(
                    "Invalid IV length: {} bytes\n\nDES uses an IV length of 8 bytes (64 bits).\nMake sure you have specified the type correctly (e.g. Hex vs UTF8).",
                    actual_iv.len()
                ),
            });
        }

        let input_bytes = if input_type == "Hex" {
            hex::decode(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        if input_bytes.is_empty() {
            return Err(OperationError::ProcessingError("Empty input".to_string()));
        }

        let no_padding = mode_full.ends_with("/NoPadding");

        use des::Des;

        let result = match mode {
            "CBC" => {
                if input_bytes.len() % 8 != 0 {
                    return Err(OperationError::ProcessingError(
                        "Input must be multiple of block size".to_string(),
                    ));
                }
                let cipher_output = Self::process_cbc::<Des>(&key, &actual_iv, &input_bytes, false);
                if no_padding {
                    cipher_output
                } else {
                    Self::pkcs7_unpad(&cipher_output, 8)?
                }
            }
            "ECB" => {
                if input_bytes.len() % 8 != 0 {
                    return Err(OperationError::ProcessingError(
                        "Input must be multiple of block size".to_string(),
                    ));
                }
                let cipher_output = Self::process_ecb::<Des>(&key, &input_bytes, false);
                if no_padding {
                    cipher_output
                } else {
                    Self::pkcs7_unpad(&cipher_output, 8)?
                }
            }
            "CTR" => Self::process_ctr::<Des>(&key, &actual_iv, &input_bytes),
            "OFB" => Self::process_ofb::<Des>(&key, &actual_iv, &input_bytes),
            "CFB" => Self::process_cfb::<Des>(&key, &actual_iv, &input_bytes, false),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Mode".to_string(),
                    reason: format!("Unsupported mode: {}", mode),
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

impl DesDecrypt {
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

    fn pkcs7_unpad(data: &[u8], block_size: usize) -> Result<Vec<u8>, OperationError> {
        if data.is_empty() {
            return Err(OperationError::ProcessingError("Empty input".to_string()));
        }
        let padding_len = data[data.len() - 1] as usize;
        if padding_len == 0 || padding_len > block_size {
            return Err(OperationError::ProcessingError(
                "Invalid padding".to_string(),
            ));
        }
        if data.len() < padding_len {
            return Err(OperationError::ProcessingError(
                "Invalid padding".to_string(),
            ));
        }
        for i in 0..padding_len {
            if data[data.len() - 1 - i] != padding_len as u8 {
                return Err(OperationError::ProcessingError(
                    "Invalid padding".to_string(),
                ));
            }
        }
        Ok(data[..data.len() - padding_len].to_vec())
    }

    fn process_cbc<C>(key: &[u8], iv: &[u8], input: &[u8], encrypt: bool) -> Vec<u8>
    where
        C: cipher::BlockCipher + cipher::KeyInit + cipher::BlockEncrypt + cipher::BlockDecrypt,
    {
        let key_arr = GenericArray::from_slice(key);
        let iv_arr = GenericArray::from_slice(iv);

        let cipher = C::new(&key_arr);
        let mut result = Vec::with_capacity(input.len());
        let mut prev_block = iv_arr.clone();

        for chunk in input.chunks(8) {
            let mut block = GenericArray::clone_from_slice(chunk);
            if encrypt {
                for i in 0..8 {
                    block[i] ^= prev_block[i];
                }
                cipher.encrypt_block(&mut block);
                prev_block.clone_from(&block);
            } else {
                cipher.decrypt_block(&mut block);
                for i in 0..8 {
                    block[i] ^= prev_block[i];
                }
                let chunk_block: GenericArray<u8, C::BlockSize> =
                    GenericArray::clone_from_slice(chunk);
                prev_block = chunk_block;
            }
            result.extend_from_slice(block.as_slice());
        }

        result
    }

    fn process_ecb<C>(key: &[u8], input: &[u8], encrypt: bool) -> Vec<u8>
    where
        C: cipher::BlockCipher + cipher::KeyInit + cipher::BlockEncrypt + cipher::BlockDecrypt,
    {
        let key_arr = GenericArray::from_slice(key);
        let cipher = C::new(&key_arr);
        let mut result = Vec::with_capacity(input.len());

        for chunk in input.chunks(8) {
            let mut block = GenericArray::clone_from_slice(chunk);
            if encrypt {
                cipher.encrypt_block(&mut block);
            } else {
                cipher.decrypt_block(&mut block);
            }
            result.extend_from_slice(block.as_slice());
        }
        result
    }

    fn process_ctr<C>(key: &[u8], iv: &[u8], input: &[u8]) -> Vec<u8>
    where
        C: cipher::BlockCipher + cipher::KeyInit + cipher::BlockEncrypt + cipher::BlockDecrypt,
    {
        let key_arr = GenericArray::from_slice(key);
        let mut counter = GenericArray::clone_from_slice(iv);
        let cipher = C::new(&key_arr);
        let mut result = Vec::with_capacity(input.len());

        for chunk in input.chunks(8) {
            let mut encrypted_counter = counter.clone();
            cipher.encrypt_block(&mut encrypted_counter);

            for i in 0..chunk.len() {
                result.push(chunk[i] ^ encrypted_counter[i]);
            }

            // Increment counter (big endian)
            for i in (0..8).rev() {
                counter[i] = counter[i].wrapping_add(1);
                if counter[i] != 0 {
                    break;
                }
            }
        }
        result
    }

    fn process_ofb<C>(key: &[u8], iv: &[u8], input: &[u8]) -> Vec<u8>
    where
        C: cipher::BlockCipher + cipher::KeyInit + cipher::BlockEncrypt + cipher::BlockDecrypt,
    {
        let key_arr = GenericArray::from_slice(key);
        let mut prev = GenericArray::clone_from_slice(iv);
        let cipher = C::new(&key_arr);
        let mut result = Vec::with_capacity(input.len());

        for chunk in input.chunks(8) {
            cipher.encrypt_block(&mut prev);
            for i in 0..chunk.len() {
                result.push(chunk[i] ^ prev[i]);
            }
        }
        result
    }

    fn process_cfb<C>(key: &[u8], iv: &[u8], input: &[u8], encrypt: bool) -> Vec<u8>
    where
        C: cipher::BlockCipher + cipher::KeyInit + cipher::BlockEncrypt + cipher::BlockDecrypt,
    {
        let key_arr = GenericArray::from_slice(key);
        let mut prev = GenericArray::clone_from_slice(iv);
        let cipher = C::new(&key_arr);
        let mut result = Vec::with_capacity(input.len());

        for chunk in input.chunks(8) {
            let mut encrypted_prev = prev.clone();
            cipher.encrypt_block(&mut encrypted_prev);

            let mut out = Vec::with_capacity(chunk.len());
            for i in 0..chunk.len() {
                out.push(chunk[i] ^ encrypted_prev[i]);
            }
            result.extend_from_slice(&out);

            if encrypt {
                let mut next_prev = GenericArray::default();
                for i in 0..out.len() {
                    next_prev[i] = out[i];
                }
                prev = next_prev;
            } else {
                let mut next_prev = GenericArray::default();
                for i in 0..chunk.len() {
                    next_prev[i] = chunk[i];
                }
                prev = next_prev;
            }
        }
        result
    }
}
