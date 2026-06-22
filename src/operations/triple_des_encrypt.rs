/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Triple DES Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use block_padding::Pkcs7;
use cipher::{BlockEncrypt, BlockEncryptMut, KeyInit, KeyIvInit};
use des::TdesEde3;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

type TdesEde3CbcEnc = cbc::Encryptor<TdesEde3>;

/// Triple DES Encrypt operation
///
/// Encrypts data using Triple DES (3DES/TdesEde3) with CBC or ECB mode.
/// Key must be 16 or 24 bytes; a 16-byte key is extended to 24 by appending
/// the first 8 bytes (two-key 3DES).
pub struct TripleDESEncrypt;

fn decode_encoding(s: &str, enc: &str) -> Result<Vec<u8>, OperationError> {
    match enc {
        "Hex" => hex::decode(s).map_err(|e| OperationError::InvalidArgument {
            name: "encoding".to_string(),
            reason: format!("hex decode failed: {}", e),
        }),
        "Base64" => {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(|e| OperationError::InvalidArgument {
                    name: "encoding".to_string(),
                    reason: format!("base64 decode failed: {}", e),
                })
        }
        _ => Ok(s.as_bytes().to_vec()),
    }
}

fn extend_key_to_24(key: Vec<u8>) -> Result<Vec<u8>, OperationError> {
    match key.len() {
        24 => Ok(key),
        16 => {
            let mut extended = key.clone();
            extended.extend_from_slice(&key[..8]);
            Ok(extended)
        }
        n => Err(OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: format!(
                "Invalid key length: {} bytes. Triple DES requires 16 or 24 bytes.",
                n
            ),
        }),
    }
}

fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let pad_len = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    padded
}

impl Operation for TripleDESEncrypt {
    fn name(&self) -> &'static str {
        "Triple DES Encrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Encrypts data using Triple DES (3DES). \
         Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. \
         Modes supported: CBC, ECB. Input/output can be Raw or Hex."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Encryption key (16 or 24 bytes). Encoding: Hex, UTF8, Latin1, Base64",
                default_value: "",
            },
            ArgSchema {
                name: "Key encoding",
                description: "Encoding of the key: Hex, UTF8, Latin1, Base64",
                default_value: "Hex",
            },
            ArgSchema {
                name: "IV",
                description:
                    "Initialization vector (8 bytes for CBC). Encoding: Hex, UTF8, Latin1, Base64",
                default_value: "",
            },
            ArgSchema {
                name: "IV encoding",
                description: "Encoding of the IV: Hex, UTF8, Latin1, Base64",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode: CBC, ECB",
                default_value: "CBC",
            },
            ArgSchema {
                name: "Input",
                description: "Input encoding: Raw, Hex",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output",
                description: "Output encoding: Hex, Raw",
                default_value: "Hex",
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
        let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let key_enc = args.get(1).and_then(|a| a.as_str()).unwrap_or("Hex");
        let iv_str = args.get(2).and_then(|a| a.as_str()).unwrap_or("");
        let iv_enc = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let mode = args.get(4).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_enc = args.get(5).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_enc = args.get(6).and_then(|a| a.as_str()).unwrap_or("Hex");

        let raw_key = decode_encoding(key_str, key_enc)?;
        let iv = decode_encoding(iv_str, iv_enc)?;
        let key = extend_key_to_24(raw_key)?;

        let plaintext = if input_enc == "Hex" {
            hex::decode(String::from_utf8_lossy(&input).trim()).map_err(|e| {
                OperationError::InvalidInput(format!("Hex decode of input failed: {}", e))
            })?
        } else {
            input
        };

        if plaintext.is_empty() {
            return Ok(Vec::new());
        }

        let ciphertext: Vec<u8> = match mode {
            "CBC" => {
                if iv.len() != 8 {
                    return Err(OperationError::InvalidArgument {
                        name: "IV".to_string(),
                        reason: format!("CBC mode requires 8-byte IV, got {} bytes", iv.len()),
                    });
                }
                let key_arr: [u8; 24] = key.as_slice().try_into().map_err(|_| {
                    OperationError::ProcessingError("Key conversion failed".to_string())
                })?;
                let iv_arr: [u8; 8] = iv.as_slice().try_into().map_err(|_| {
                    OperationError::ProcessingError("IV conversion failed".to_string())
                })?;
                let enc = TdesEde3CbcEnc::new(&key_arr.into(), &iv_arr.into());
                enc.encrypt_padded_vec_mut::<Pkcs7>(&plaintext)
            }
            "ECB" => {
                let key_arr: [u8; 24] = key.as_slice().try_into().map_err(|_| {
                    OperationError::ProcessingError("Key conversion failed".to_string())
                })?;
                let cipher = TdesEde3::new(&key_arr.into());
                let mut buf = pkcs7_pad(&plaintext, 8);
                for chunk in buf.chunks_exact_mut(8) {
                    let block = generic_array::GenericArray::from_mut_slice(chunk);
                    cipher.encrypt_block(block);
                }
                buf
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Mode".to_string(),
                    reason: format!("Unsupported mode: {}. Use CBC or ECB.", mode),
                });
            }
        };

        if output_enc == "Hex" {
            Ok(hex::encode(&ciphertext).into_bytes())
        } else {
            Ok(ciphertext)
        }
    }
}
