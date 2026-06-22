/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the DES Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{Block, BlockEncrypt, KeyInit};
use des::Des;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// DES Encrypt operation
pub struct DesEncrypt;

impl Operation for DesEncrypt {
    fn name(&self) -> &'static str {
        "DES Encrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "DES is a previously dominant algorithm for encryption, and was published as an official U.S. Federal Information Processing Standard (FIPS). It is now considered to be insecure due to its small key size.<br><br><b>Key:</b> DES uses a key length of 8 bytes (64 bits).<br><br>You can generate a password-based key using one of the KDF operations.<br><br><b>IV:</b> The Initialization Vector should be 8 bytes long. If not entered, it will default to 8 null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, PKCS#7 padding will be used."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Encryption key (8 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (8 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode (CBC, CFB, OFB, CTR, ECB)",
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
        let key = Self::parse_arg_bytes(args.first())?;
        let iv = Self::parse_arg_bytes(args.get(1))?;
        let mode = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Hex");

        if key.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes\n\nDES uses a key length of 8 bytes (64 bits).",
                    key.len()
                ),
            });
        }

        if mode != "ECB" && !iv.is_empty() && iv.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!("Invalid IV length: {} bytes\n\nDES uses an IV length of 8 bytes (64 bits).\nMake sure you have specified the type correctly (e.g. Hex vs UTF8).", iv.len()),
            });
        }

        let input_bytes = if input_type == "Hex" {
            let s = String::from_utf8_lossy(&input);
            hex::decode(s.as_ref().trim())
                .map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        let iv = if iv.is_empty() { vec![0u8; 8] } else { iv };

        let padded_input = Self::pkcs7_pad(&input_bytes);
        let mode_lower = mode.to_lowercase();

        let result = match mode_lower.as_str() {
            "ecb" => self.encrypt_ecb(&key, &padded_input),
            "cbc" => self.encrypt_cbc(&key, &iv, &padded_input),
            "cfb" => self.encrypt_cfb(&key, &iv, &input_bytes),
            "ofb" => self.encrypt_ofb(&key, &iv, &input_bytes),
            "ctr" => self.encrypt_ctr(&key, &iv, &input_bytes),
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

impl DesEncrypt {
    fn parse_arg_bytes(arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else {
                    let s_trimmed = s.trim_start_matches("0x");
                    if s_trimmed.chars().all(|c| c.is_ascii_hexdigit())
                        && !s_trimmed.is_empty()
                        && s_trimmed.len().is_multiple_of(2)
                    {
                        match hex::decode(s_trimmed) {
                            Ok(decoded) => Ok(decoded),
                            Err(_) => Ok(s.as_bytes().to_vec()),
                        }
                    } else {
                        match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s)
                        {
                            Ok(decoded) => Ok(decoded),
                            Err(_) => Ok(s.as_bytes().to_vec()),
                        }
                    }
                }
            }
            Some(ArgValue::Bytes(b)) => Ok(b.clone()),
            _ => Ok(vec![]),
        }
    }

    fn pkcs7_pad(data: &[u8]) -> Vec<u8> {
        let block_size = 8;
        let padding_len = block_size - (data.len() % block_size);
        let mut padded = data.to_vec();
        padded.extend(vec![padding_len as u8; padding_len]);
        padded
    }

    fn encrypt_ecb(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let cipher = Des::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Invalid key length".to_string(),
        })?;

        let mut result = Vec::with_capacity(input.len());
        for chunk in input.chunks(8) {
            let mut block = Block::<Des>::default();
            block.clone_from_slice(chunk);
            cipher.encrypt_block(&mut block);
            result.extend_from_slice(block.as_slice());
        }

        Ok(result)
    }

    fn encrypt_cbc(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let cipher = Des::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Invalid key length".to_string(),
        })?;

        let mut prev_block = Block::<Des>::default();
        prev_block.clone_from_slice(iv);
        let mut result = Vec::with_capacity(input.len());

        for chunk in input.chunks(8) {
            let mut block = Block::<Des>::default();
            block.clone_from_slice(chunk);

            for i in 0..8 {
                block[i] ^= prev_block[i];
            }

            cipher.encrypt_block(&mut block);
            prev_block.clone_from_slice(&block);
            result.extend_from_slice(block.as_slice());
        }

        Ok(result)
    }

    fn encrypt_cfb(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let cipher = Des::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Invalid key length".to_string(),
        })?;

        let mut result = Vec::with_capacity(input.len());
        let mut register = Block::<Des>::default();
        register.clone_from_slice(iv);

        for chunk in input.chunks(8) {
            let mut encrypted_reg = register.clone();
            cipher.encrypt_block(&mut encrypted_reg);

            let mut block = vec![0u8; chunk.len()];
            for i in 0..chunk.len() {
                block[i] = chunk[i] ^ encrypted_reg[i];
            }

            let chunk_len = chunk.len();
            for i in chunk_len..8 {
                register[i - chunk_len] = register[i];
            }
            for (i, &v) in block.iter().enumerate() {
                register[8 - chunk_len + i] = v;
            }

            result.extend_from_slice(&block);
        }

        Ok(result)
    }

    fn encrypt_ofb(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let cipher = Des::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Invalid key length".to_string(),
        })?;

        let mut result = Vec::with_capacity(input.len());
        let mut register = Block::<Des>::default();
        register.clone_from_slice(iv);

        for chunk in input.chunks(8) {
            let mut encrypted_reg = register.clone();
            cipher.encrypt_block(&mut encrypted_reg);
            register = encrypted_reg;

            let mut block = vec![0u8; chunk.len()];
            for i in 0..chunk.len() {
                block[i] = chunk[i] ^ encrypted_reg[i];
            }

            result.extend_from_slice(&block);
        }

        Ok(result)
    }

    fn encrypt_ctr(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        let cipher = Des::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Invalid key length".to_string(),
        })?;

        let mut result = Vec::with_capacity(input.len());
        let mut counter = Block::<Des>::default();
        counter.clone_from_slice(iv);

        for chunk in input.chunks(8) {
            let mut counter_block = counter.clone();
            cipher.encrypt_block(&mut counter_block);

            let mut block = vec![0u8; chunk.len()];
            for i in 0..chunk.len() {
                block[i] = chunk[i] ^ counter_block[i];
            }

            let mut carry = 1u64;
            for i in (0..8).rev() {
                let val = counter[i] as u64 + carry;
                counter[i] = val as u8;
                carry = val >> 8;
            }

            result.extend_from_slice(&block);
        }

        Ok(result)
    }
}
