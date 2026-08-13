/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AES Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use generic_array::GenericArray;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AES Decrypt operation
///
/// Advanced Encryption Standard (AES) is a U.S. Federal Information Processing
/// Standard (FIPS). It was selected after a 5-year process where 15 competing
/// designs were evaluated.
pub struct AesDecrypt;

impl Operation for AesDecrypt {
    fn name(&self) -> &'static str {
        "AES Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Advanced Encryption Standard (AES) is a U.S. Federal Information Processing Standard (FIPS). It was selected after a 5-year process where 15 competing designs were evaluated."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Decryption key (16, 24, or 32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (16 bytes, optional, defaults to null)",
                default_value: "",
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode (CBC, CFB, OFB, CTR, GCM, ECB)",
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
            ArgSchema {
                name: "GCM Tag",
                description: "GCM authentication tag (optional)",
                default_value: "",
            },
            ArgSchema {
                name: "Additional Authenticated Data",
                description: "AAD for GCM mode (optional)",
                default_value: "",
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
        let mode = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Raw");
        let gcm_tag = Self::parse_arg_bytes(args.get(5))?;
        let aad = Self::parse_arg_bytes(args.get(6))?;

        // Validate key length
        if ![16, 24, 32].contains(&key.len()) {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes. Valid lengths are 16, 24, or 32 bytes.",
                    key.len()
                ),
            });
        }

        // Parse input
        let input_bytes = if input_type == "Hex" {
            hex::decode(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        let no_padding = mode.ends_with("/NoPadding");
        let mode = mode.split('/').next().unwrap_or("CBC");

        // Handle NoPadding modes - check input length
        if no_padding && input_bytes.len() % 16 != 0 {
            return Err(OperationError::InvalidArgument {
                name: "Input".to_string(),
                reason: "Input length must be a multiple of 16 bytes for NoPadding modes."
                    .to_string(),
            });
        }

        // For GCM, check if we have a tag
        if mode == "GCM" && gcm_tag.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "GCM Tag".to_string(),
                reason: "GCM mode requires an authentication tag".to_string(),
            });
        }

        let result = match mode {
            "CBC" => self.decrypt_cbc(&key, &iv, &input_bytes, no_padding),
            "CFB" => self.decrypt_stream(&key, &iv, &input_bytes, StreamMode::Cfb),
            "OFB" => self.decrypt_stream(&key, &iv, &input_bytes, StreamMode::Ofb),
            "CTR" => self.decrypt_stream(&key, &iv, &input_bytes, StreamMode::Ctr),
            "GCM" => self.decrypt_gcm(&key, &iv, &gcm_tag, &aad, &input_bytes),
            "ECB" => self.decrypt_ecb(&key, &input_bytes, no_padding),
            _ => Err(OperationError::InvalidArgument {
                name: "Mode".to_string(),
                reason: format!("Unsupported mode: {}", mode),
            }),
        }?;

        let output_bytes = if output_type == "Hex" {
            hex::encode(result).into_bytes()
        } else {
            result
        };

        Ok(output_bytes)
    }
}

impl AesDecrypt {
    /// Parse an ArgValue as bytes (supports Hex and UTF8)
    fn parse_arg_bytes(arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else if let Some(stripped) = s.strip_prefix("0x") {
                    hex::decode(stripped).map_err(|e| OperationError::InvalidArgument {
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

    /// PKCS#7 padding removal
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
        // Verify padding
        for i in 0..padding_len {
            if data[data.len() - 1 - i] != padding_len as u8 {
                return Err(OperationError::ProcessingError(
                    "Invalid padding".to_string(),
                ));
            }
        }
        Ok(data[..data.len() - padding_len].to_vec())
    }

    /// AES-CBC decryption using the aes crate
    fn decrypt_cbc(
        &self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
        no_padding: bool,
    ) -> Result<Vec<u8>, OperationError> {
        let iv = if iv.is_empty() {
            vec![0u8; 16]
        } else if iv.len() != 16 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: "IV must be 16 bytes".to_string(),
            });
        } else {
            iv.to_vec()
        };

        if input.is_empty() {
            return Err(OperationError::ProcessingError("Empty input".to_string()));
        }

        // Input must be multiple of block size
        if !input.len().is_multiple_of(16) {
            return Err(OperationError::ProcessingError(
                "Input must be multiple of block size".to_string(),
            ));
        }

        // Use the aes crate with the block cipher traits
        use aes::{Aes128, Aes192, Aes256};

        let cipher_output = match key.len() {
            16 => Self::process_cbc::<Aes128>(key, &iv, input, false),
            24 => Self::process_cbc::<Aes192>(key, &iv, input, false),
            32 => Self::process_cbc::<Aes256>(key, &iv, input, false),
            _ => {
                return Err(OperationError::ProcessingError(
                    "Invalid key length".to_string(),
                ))
            }
        };

        let result = if no_padding {
            cipher_output
        } else {
            Self::pkcs7_unpad(&cipher_output, 16)?
        };

        Ok(result)
    }

    fn decrypt_ecb(
        &self,
        key: &[u8],
        input: &[u8],
        no_padding: bool,
    ) -> Result<Vec<u8>, OperationError> {
        use aes::{Aes128, Aes192, Aes256};

        if input.is_empty() || !input.len().is_multiple_of(16) {
            return Err(OperationError::InvalidInput(
                "ECB input must be a non-empty multiple of 16 bytes".to_string(),
            ));
        }
        let output = match key.len() {
            16 => Self::process_ecb::<Aes128>(key, input),
            24 => Self::process_ecb::<Aes192>(key, input),
            32 => Self::process_ecb::<Aes256>(key, input),
            _ => unreachable!("key length was validated"),
        };
        if no_padding {
            Ok(output)
        } else {
            Self::pkcs7_unpad(&output, 16)
        }
    }

    fn process_ecb<C>(key: &[u8], input: &[u8]) -> Vec<u8>
    where
        C: cipher::BlockCipher
            + cipher::BlockSizeUser<BlockSize = cipher::consts::U16>
            + cipher::KeyInit
            + cipher::BlockDecrypt,
    {
        let cipher = C::new(GenericArray::from_slice(key));
        let mut output = input.to_vec();
        for chunk in output.chunks_exact_mut(16) {
            cipher.decrypt_block(GenericArray::from_mut_slice(chunk));
        }
        output
    }

    fn decrypt_stream(
        &self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
        mode: StreamMode,
    ) -> Result<Vec<u8>, OperationError> {
        use aes::{Aes128, Aes192, Aes256};

        let iv = normalise_iv(iv)?;
        Ok(match key.len() {
            16 => Self::process_stream::<Aes128>(key, &iv, input, mode),
            24 => Self::process_stream::<Aes192>(key, &iv, input, mode),
            32 => Self::process_stream::<Aes256>(key, &iv, input, mode),
            _ => unreachable!("key length was validated"),
        })
    }

    fn process_stream<C>(key: &[u8], iv: &[u8; 16], input: &[u8], mode: StreamMode) -> Vec<u8>
    where
        C: cipher::BlockCipher
            + cipher::BlockSizeUser<BlockSize = cipher::consts::U16>
            + cipher::KeyInit
            + cipher::BlockEncrypt,
    {
        let cipher = C::new(GenericArray::from_slice(key));
        let mut state = *iv;
        let mut output = Vec::with_capacity(input.len());
        for chunk in input.chunks(16) {
            let mut keystream = GenericArray::clone_from_slice(&state);
            cipher.encrypt_block(&mut keystream);
            output.extend(
                chunk
                    .iter()
                    .zip(keystream.iter())
                    .map(|(byte, mask)| byte ^ mask),
            );
            match mode {
                StreamMode::Cfb => {
                    if chunk.len() == 16 {
                        state.copy_from_slice(chunk);
                    }
                }
                StreamMode::Ofb => state.copy_from_slice(&keystream),
                StreamMode::Ctr => increment_counter(&mut state),
            }
        }
        output
    }

    /// Process CBC mode encryption/decryption
    fn process_cbc<C>(key: &[u8], iv: &[u8], input: &[u8], encrypt: bool) -> Vec<u8>
    where
        C: cipher::BlockCipher + cipher::KeyInit + cipher::BlockEncrypt + cipher::BlockDecrypt,
    {
        let key_arr = GenericArray::from_slice(key);
        let iv_arr = GenericArray::from_slice(iv);

        // Create a new cipher instance
        let cipher = C::new(&key_arr);

        let mut result = Vec::with_capacity(input.len());
        let mut prev_block = iv_arr.clone();

        for chunk in input.chunks(16) {
            let mut block = GenericArray::clone_from_slice(chunk);
            if encrypt {
                // XOR with previous block (or IV for first block)
                for i in 0..16 {
                    block[i] ^= prev_block[i];
                }
                // Encrypt
                cipher.encrypt_block(&mut block);
                // Update previous block
                prev_block.clone_from(&block);
            } else {
                // Decrypt first
                cipher.decrypt_block(&mut block);
                // XOR with previous block (or IV for first block)
                for i in 0..16 {
                    block[i] ^= prev_block[i];
                }
                // Update previous block
                let chunk_block: GenericArray<u8, C::BlockSize> =
                    GenericArray::clone_from_slice(chunk);
                prev_block = chunk_block;
            }
            result.extend_from_slice(block.as_slice());
        }

        result
    }

    /// AES-GCM decryption
    fn decrypt_gcm(
        &self,
        key: &[u8],
        iv: &[u8],
        tag: &[u8],
        aad: &[u8],
        input: &[u8],
    ) -> Result<Vec<u8>, OperationError> {
        use aes_gcm::aead::{Aead, KeyInit, Payload};
        use aes_gcm::{Aes128Gcm, Aes256Gcm, Nonce};

        if iv.len() != 12 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: "GCM requires a 12-byte IV".to_string(),
            });
        }
        if tag.len() != 16 {
            return Err(OperationError::InvalidArgument {
                name: "GCM Tag".to_string(),
                reason: "GCM tag must be 16 bytes".to_string(),
            });
        }
        let mut ciphertext = input.to_vec();
        ciphertext.extend_from_slice(tag);
        let payload = Payload {
            msg: &ciphertext,
            aad,
        };
        match key.len() {
            16 => Aes128Gcm::new_from_slice(key)
                .unwrap()
                .decrypt(Nonce::from_slice(iv), payload),
            32 => Aes256Gcm::new_from_slice(key)
                .unwrap()
                .decrypt(Nonce::from_slice(iv), payload),
            24 => {
                return Err(OperationError::InvalidArgument {
                    name: "Key".to_string(),
                    reason: "AES-192-GCM is not supported; use a 16- or 32-byte key".to_string(),
                })
            }
            _ => unreachable!("key length was validated"),
        }
        .map_err(|_| OperationError::ProcessingError("GCM authentication failed".to_string()))
    }
}

#[derive(Clone, Copy)]
enum StreamMode {
    Cfb,
    Ofb,
    Ctr,
}

fn normalise_iv(iv: &[u8]) -> Result<[u8; 16], OperationError> {
    if iv.is_empty() {
        return Ok([0; 16]);
    }
    iv.try_into().map_err(|_| OperationError::InvalidArgument {
        name: "IV".to_string(),
        reason: "IV must be 16 bytes".to_string(),
    })
}

fn increment_counter(counter: &mut [u8; 16]) {
    for byte in counter.iter_mut().rev() {
        let (value, overflow) = byte.overflowing_add(1);
        *byte = value;
        if !overflow {
            break;
        }
    }
}
