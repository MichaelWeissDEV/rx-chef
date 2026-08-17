/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
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
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (16 bytes, optional, defaults to null)",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode (CBC, CFB, OFB, CTR, GCM, ECB)",
                default_value: "CBC",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["CBC", "CFB", "OFB", "CTR", "GCM", "ECB"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Input",
                description: "Input encoding (Raw, Hex)",
                default_value: "Raw",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["Raw", "Hex"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output",
                description: "Output encoding (Hex, Raw)",
                default_value: "Hex",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["Hex", "Raw"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Additional Authenticated Data",
                description: "AAD for GCM mode (optional)",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
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

    /// Conforms to the published specification; not yet compared against CyberChef.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Compatible
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
            "CFB" => self.encrypt_stream(&key, &iv, &input_bytes, StreamMode::Cfb),
            "OFB" => self.encrypt_stream(&key, &iv, &input_bytes, StreamMode::Ofb),
            "CTR" => self.encrypt_stream(&key, &iv, &input_bytes, StreamMode::Ctr),
            "GCM" => self.encrypt_gcm(&key, &iv, &aad, &input_bytes),
            "ECB" => self.encrypt_ecb(&key, &input_bytes, no_padding),
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

    fn encrypt_ecb(
        &self,
        key: &[u8],
        input: &[u8],
        no_padding: bool,
    ) -> Result<Vec<u8>, OperationError> {
        use aes::{Aes128, Aes192, Aes256};

        let input = if no_padding {
            input.to_vec()
        } else {
            Self::pkcs7_pad(input, 16)
        };
        Ok(match key.len() {
            16 => Self::process_ecb::<Aes128>(key, &input),
            24 => Self::process_ecb::<Aes192>(key, &input),
            32 => Self::process_ecb::<Aes256>(key, &input),
            _ => unreachable!("key length was validated"),
        })
    }

    fn process_ecb<C>(key: &[u8], input: &[u8]) -> Vec<u8>
    where
        C: cipher::BlockCipher
            + cipher::BlockSizeUser<BlockSize = cipher::consts::U16>
            + cipher::KeyInit
            + cipher::BlockEncrypt,
    {
        let cipher = C::new(GenericArray::from_slice(key));
        let mut output = input.to_vec();
        for chunk in output.chunks_exact_mut(16) {
            cipher.encrypt_block(GenericArray::from_mut_slice(chunk));
        }
        output
    }

    fn encrypt_stream(
        &self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
        mode: StreamMode,
    ) -> Result<Vec<u8>, OperationError> {
        use aes::{Aes128, Aes192, Aes256};

        let iv = normalise_iv(iv)?;
        Ok(match key.len() {
            16 => Self::process_stream::<Aes128>(key, &iv, input, mode, true),
            24 => Self::process_stream::<Aes192>(key, &iv, input, mode, true),
            32 => Self::process_stream::<Aes256>(key, &iv, input, mode, true),
            _ => unreachable!("key length was validated"),
        })
    }

    fn process_stream<C>(
        key: &[u8],
        iv: &[u8; 16],
        input: &[u8],
        mode: StreamMode,
        encrypt: bool,
    ) -> Vec<u8>
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
            let transformed = chunk
                .iter()
                .zip(keystream.iter())
                .map(|(byte, mask)| byte ^ mask)
                .collect::<Vec<_>>();
            output.extend_from_slice(&transformed);
            match mode {
                StreamMode::Cfb => {
                    if chunk.len() == 16 {
                        state.copy_from_slice(if encrypt { &transformed } else { chunk });
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
        use aes_gcm::aead::{Aead, KeyInit, Payload};
        use aes_gcm::{Aes128Gcm, Aes256Gcm, Nonce};

        if iv.len() != 12 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: "GCM requires a 12-byte IV".to_string(),
            });
        }
        let payload = Payload { msg: input, aad };
        match key.len() {
            16 => Aes128Gcm::new_from_slice(key)
                .unwrap()
                .encrypt(Nonce::from_slice(iv), payload),
            32 => Aes256Gcm::new_from_slice(key)
                .unwrap()
                .encrypt(Nonce::from_slice(iv), payload),
            24 => {
                return Err(OperationError::InvalidArgument {
                    name: "Key".to_string(),
                    reason: "AES-192-GCM is not supported; use a 16- or 32-byte key".to_string(),
                })
            }
            _ => unreachable!("key length was validated"),
        }
        .map_err(|_| OperationError::ProcessingError("GCM encryption failed".to_string()))
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
