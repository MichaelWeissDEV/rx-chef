/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AES Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use generic_array::GenericArray;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AES Encrypt operation
///
/// Advanced Encryption Standard (AES) is a U.S. Federal Information Processing
/// Standard (FIPS). It was selected after a 5-year process where 15 competing
/// designs were evaluated.
pub struct AesEncrypt;

impl Operation for AesEncrypt {
    fn name(&self) -> &'static str {
        "AES Encrypt"
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
                description: "Encryption key (16, 24, or 32 bytes)",
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
                description: "Input encoding (Raw, Hex)",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output",
                description: "Output encoding (Hex, Raw)",
                default_value: "Hex",
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
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Hex");
        let aad = Self::parse_arg_bytes(args.get(5))?;

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

        let result = match mode {
            "CBC" => self.encrypt_cbc(&key, &iv, &input_bytes, no_padding),
            "CFB" => Err(OperationError::ProcessingError(
                "CFB mode not yet implemented".to_string(),
            )),
            "OFB" => Err(OperationError::ProcessingError(
                "OFB mode not yet implemented".to_string(),
            )),
            "CTR" => Err(OperationError::ProcessingError(
                "CTR mode not yet implemented".to_string(),
            )),
            "GCM" => self.encrypt_gcm(&key, &iv, &aad, &input_bytes),
            "ECB" => Err(OperationError::ProcessingError(
                "ECB mode not yet implemented".to_string(),
            )),
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

impl AesEncrypt {
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

    /// PKCS#7 padding
    fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
        let padding_len = block_size - (data.len() % block_size);
        let mut padded = data.to_vec();
        padded.extend(vec![padding_len as u8; padding_len]);
        padded
    }

    /// AES-CBC encryption using the aes crate
    fn encrypt_cbc(
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

        let cipher_input = if no_padding {
            input.to_vec()
        } else {
            Self::pkcs7_pad(input, 16)
        };

        // Use the aes crate with the block cipher traits
        use aes::{Aes128, Aes192, Aes256};

        let result = match key.len() {
            16 => Self::process_cbc::<Aes128>(&key, &iv, &cipher_input, true),
            24 => Self::process_cbc::<Aes192>(&key, &iv, &cipher_input, true),
            32 => Self::process_cbc::<Aes256>(&key, &iv, &cipher_input, true),
            _ => {
                return Err(OperationError::ProcessingError(
                    "Invalid key length".to_string(),
                ))
            }
        };

        Ok(result)
    }

    /// Process CBC mode encryption/decryption
    fn process_cbc<C>(key: &[u8], iv: &[u8], input: &[u8], encrypt: bool) -> Vec<u8>
    where
        C: cipher::BlockCipher
            + cipher::BlockSizeUser
            + cipher::KeyInit
            + cipher::BlockEncrypt
            + cipher::BlockDecrypt,
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

    /// AES-GCM encryption
    fn encrypt_gcm(
        &self,
        key: &[u8],
        iv: &[u8],
        aad: &[u8],
        input: &[u8],
    ) -> Result<Vec<u8>, OperationError> {
        // GCM mode not supported by the aes crate directly
        // Use aad parameter and IV for future implementation
        let _ = (key, iv, aad, input);
        Err(OperationError::ProcessingError(
            "GCM mode not yet implemented".to_string(),
        ))
    }
}
