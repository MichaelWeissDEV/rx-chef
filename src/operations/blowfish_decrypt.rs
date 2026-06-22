/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Blowfish Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use blowfish::Blowfish;
use byteorder::BE;
use cipher::{Block, BlockDecrypt, BlockEncrypt, KeyInit};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Blowfish Decrypt operation
///
/// Blowfish is a symmetric-key block cipher designed in 1993 by Bruce Schneier
/// and included in a large number of cipher suites and encryption products.
/// AES now receives more attention.
pub struct BlowfishDecrypt;

impl Operation for BlowfishDecrypt {
    fn name(&self) -> &'static str {
        "Blowfish Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Blowfish is a symmetric-key block cipher designed in 1993 by Bruce Schneier and included in a large number of cipher suites and encryption products. AES now receives more attention."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Decryption key (4-56 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (8 bytes for non-ECB modes, optional)",
                default_value: "",
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode (CBC, CFB, OFB, CTR, ECB)",
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
        let key = Self::parse_arg_bytes(args.first(), args.first())?;
        let iv = Self::parse_arg_bytes(args.get(1), args.get(1))?;
        let mode = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Raw");

        // Validate key length (Blowfish: 4-56 bytes)
        if key.len() < 4 || key.len() > 56 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes. Blowfish's key length needs to be between 4 and 56 bytes (32-448 bits).",
                    key.len()
                ),
            });
        }

        // Validate IV length for non-ECB modes
        if mode != "ECB" && !iv.is_empty() && iv.len() != 8 {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!("Invalid IV length: {} bytes. Expected 8 bytes.", iv.len()),
            });
        }

        // Parse input
        let input_bytes = if input_type == "Hex" {
            hex::decode(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        // Default IV to 8 null bytes if empty
        let iv = if iv.is_empty() { vec![0u8; 8] } else { iv };

        let mode_lower = mode.to_lowercase();

        let result = match mode_lower.as_str() {
            "ecb" => self.decrypt_ecb(&key, &input_bytes),
            "cbc" => self.decrypt_cbc(&key, &iv, &input_bytes),
            "cfb" => self.decrypt_cfb(&key, &iv, &input_bytes),
            "ofb" => self.decrypt_ofb(&key, &iv, &input_bytes),
            "ctr" => self.decrypt_ctr(&key, &iv, &input_bytes),
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

impl BlowfishDecrypt {
    /// Parse an ArgValue as bytes (supports Hex, UTF8, Latin1, Base64)
    fn parse_arg_bytes(
        arg: Option<&ArgValue>,
        _arg2: Option<&ArgValue>,
    ) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else {
                    // Check if it's hex (starts with 0x or contains only hex chars)
                    let s_trimmed = s.trim_start_matches("0x");
                    if s_trimmed.chars().all(|c| c.is_ascii_hexdigit()) && !s_trimmed.is_empty() {
                        // Try hex decode, but fall back to raw bytes if invalid (e.g., odd length)
                        match hex::decode(s_trimmed) {
                            Ok(decoded) => Ok(decoded),
                            Err(_) => Ok(s.as_bytes().to_vec()),
                        }
                    } else {
                        // Try base64 first
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

    /// PKCS#7 padding removal
    fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, OperationError> {
        if data.is_empty() {
            return Err(OperationError::ProcessingError(
                "Empty input data".to_string(),
            ));
        }
        let padding_len = data[data.len() - 1] as usize;
        if padding_len == 0 || padding_len > 8 {
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

    /// Decrypt data in ECB mode
    fn decrypt_ecb(&self, key: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        if input.len() % 8 != 0 {
            return Err(OperationError::InvalidInput(
                "Input length must be a multiple of 8 bytes for ECB mode".to_string(),
            ));
        }

        let cipher: Blowfish<BE> =
            Blowfish::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Invalid key length".to_string(),
            })?;

        let mut result = Vec::with_capacity(input.len());
        for chunk in input.chunks(8) {
            let mut block = Block::<Blowfish<BE>>::default();
            block.clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);
            result.extend_from_slice(block.as_slice());
        }

        Ok(result)
    }

    /// Decrypt data in CBC mode
    fn decrypt_cbc(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        if input.len() % 8 != 0 {
            return Err(OperationError::InvalidInput(
                "Input length must be a multiple of 8 bytes for CBC mode".to_string(),
            ));
        }

        let cipher: Blowfish<BE> =
            Blowfish::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Invalid key length".to_string(),
            })?;

        let mut prev_block = Block::<Blowfish<BE>>::default();
        prev_block.clone_from_slice(iv);
        let mut result = Vec::with_capacity(input.len());

        for chunk in input.chunks(8) {
            let mut block = Block::<Blowfish<BE>>::default();
            block.clone_from_slice(chunk);

            cipher.decrypt_block(&mut block);

            // XOR with previous block
            for i in 0..8 {
                block[i] ^= prev_block[i];
            }

            prev_block.clone_from_slice(chunk);
            result.extend_from_slice(block.as_slice());
        }

        Ok(Self::pkcs7_unpad(&result)?)
    }

    /// Decrypt data in CFB mode
    fn decrypt_cfb(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        // CFB mode doesn't require padding
        let cipher: Blowfish<BE> =
            Blowfish::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Invalid key length".to_string(),
            })?;

        let mut result = Vec::with_capacity(input.len());
        let mut register = Block::<Blowfish<BE>>::default();
        register.clone_from_slice(iv);

        for chunk in input.chunks(8) {
            // Encrypt the register (full block)
            let mut encrypted_reg = register.clone();
            cipher.encrypt_block(&mut encrypted_reg);

            // XOR with ciphertext to get plaintext
            let mut block = Block::<Blowfish<BE>>::default();
            block[..chunk.len()].clone_from_slice(chunk);
            for i in 0..chunk.len() {
                block[i] ^= encrypted_reg[i];
            }

            // Update register with ciphertext (shift and append)
            let chunk_len = chunk.len();
            for i in chunk_len..8 {
                register[i - chunk_len] = register[i];
            }
            for (i, &v) in chunk.iter().enumerate() {
                register[8 - chunk_len + i] = v;
            }

            result.extend_from_slice(block.as_slice());
        }

        Ok(result)
    }

    /// Decrypt data in OFB mode
    fn decrypt_ofb(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        // OFB mode doesn't require padding (same as encryption)
        let cipher: Blowfish<BE> =
            Blowfish::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Invalid key length".to_string(),
            })?;

        let mut result = Vec::with_capacity(input.len());
        let mut register = Block::<Blowfish<BE>>::default();
        register.clone_from_slice(iv);

        for chunk in input.chunks(8) {
            // Encrypt the register (full block)
            let mut encrypted_reg = register.clone();
            cipher.encrypt_block(&mut encrypted_reg);
            register = encrypted_reg;

            // XOR with ciphertext to get plaintext
            let mut block = Block::<Blowfish<BE>>::default();
            block[..chunk.len()].clone_from_slice(chunk);
            for i in 0..chunk.len() {
                block[i] ^= encrypted_reg[i];
            }

            result.extend_from_slice(block.as_slice());
        }

        Ok(result)
    }

    /// Decrypt data in CTR mode
    fn decrypt_ctr(&self, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
        // CTR mode doesn't require padding (same as encryption)
        let cipher: Blowfish<BE> =
            Blowfish::new_from_slice(key).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Invalid key length".to_string(),
            })?;

        let mut result = Vec::with_capacity(input.len());
        let mut counter = Block::<Blowfish<BE>>::default();
        counter.clone_from_slice(iv);

        for chunk in input.chunks(8) {
            // Encrypt the counter (full block)
            let mut counter_block = counter.clone();
            cipher.encrypt_block(&mut counter_block);

            // XOR with ciphertext to get plaintext
            let mut block = Block::<Blowfish<BE>>::default();
            block[..chunk.len()].clone_from_slice(chunk);
            for i in 0..chunk.len() {
                block[i] ^= counter_block[i];
            }

            // Increment counter
            let mut carry = 1u64;
            for i in (0..8).rev() {
                let val = counter[i] as u64 + carry;
                counter[i] = val as u8;
                carry = val >> 8;
            }

            result.extend_from_slice(block.as_slice());
        }

        Ok(result)
    }
}
