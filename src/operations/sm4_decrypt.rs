/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SM4 Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use sm4::{
    cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit},
    Sm4,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SM4 Decrypt operation
///
/// SM4 is a 128-bit block cipher, currently established as a national
/// standard (GB/T 32907-2016) of China. Supports ECB, CBC, CFB, OFB, and CTR
/// modes. PKCS#7 padding is removed for ECB and CBC unless the NoPadding
/// variant is selected.
pub struct Sm4Decrypt;

impl Operation for Sm4Decrypt {
    fn name(&self) -> &'static str {
        "SM4 Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "SM4 is a 128-bit block cipher, currently established as a national standard (GB/T 32907-2016) of China. Multiple block cipher modes are supported. When using CBC or ECB mode, the PKCS#7 padding scheme is used."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Decryption key (16 bytes, 128 bits)",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "Initialization Vector (16 bytes for CBC/CFB/OFB/CTR modes)",
                default_value: "",
            },
            ArgSchema {
                name: "Mode",
                description: "Cipher mode (CBC, CFB, OFB, CTR, ECB, CBC/NoPadding, ECB/NoPadding)",
                default_value: "CBC",
            },
            ArgSchema {
                name: "Input",
                description: "Input encoding (Raw, Hex)",
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
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let iv_str = args.get(1).and_then(|a| a.as_str()).unwrap_or("");
        let mode = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Raw");

        let key = parse_hex_bytes(key_str).map_err(|e| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: e,
        })?;
        let iv = if iv_str.is_empty() {
            vec![]
        } else {
            parse_hex_bytes(iv_str).map_err(|e| OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: e,
            })?
        };

        if key.len() != 16 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "Invalid key length: {} bytes. SM4 uses a key length of 16 bytes (128 bits).",
                    key.len()
                ),
            });
        }

        let no_padding = mode.ends_with("/NoPadding");
        let base_mode = if no_padding {
            &mode[..mode.len() - 10]
        } else {
            mode
        };

        if !base_mode.starts_with("ECB") {
            if iv.len() != 16 {
                return Err(OperationError::InvalidArgument {
                    name: "IV".to_string(),
                    reason: format!(
                        "Invalid IV length: {} bytes. SM4 uses an IV length of 16 bytes (128 bits).",
                        iv.len()
                    ),
                });
            }
        }

        // Decode input
        let input_bytes = match input_type {
            "Hex" => parse_hex_bytes(&String::from_utf8_lossy(&input))
                .map_err(|e| OperationError::InvalidInput(e))?,
            _ => input,
        };

        let result = match base_mode {
            "ECB" => decrypt_ecb(&key, &input_bytes, !no_padding),
            "CBC" => decrypt_cbc(&key, &iv, &input_bytes, !no_padding),
            "CFB" => decrypt_cfb(&key, &iv, &input_bytes),
            "OFB" => decrypt_ofb(&key, &iv, &input_bytes),
            "CTR" => decrypt_ctr(&key, &iv, &input_bytes),
            _ => Err(OperationError::InvalidArgument {
                name: "Mode".to_string(),
                reason: format!("Unsupported mode: {}", mode),
            }),
        }?;

        let output = match output_type {
            "Hex" => hex::encode(&result).into_bytes(),
            _ => result,
        };

        Ok(output)
    }
}

/// Parse a hex string (with optional spaces) into bytes.
fn parse_hex_bytes(s: &str) -> Result<Vec<u8>, String> {
    let compact: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    hex::decode(&compact).map_err(|e| format!("hex decode failed: {}", e))
}

/// Remove PKCS#7 padding.
fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, OperationError> {
    let block_size = 16usize;
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

/// SM4 ECB decryption.
fn decrypt_ecb(key: &[u8], input: &[u8], padding: bool) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    if input.len() % 16 != 0 {
        return Err(OperationError::InvalidInput(
            "Input length must be a multiple of 16 bytes for ECB mode.".to_string(),
        ));
    }

    let mut result = Vec::with_capacity(input.len());
    for chunk in input.chunks(16) {
        let mut block = *<&sm4::cipher::Block<Sm4>>::try_from(chunk).unwrap();
        cipher.decrypt_block(&mut block);
        result.extend_from_slice(block.as_slice());
    }

    if padding {
        pkcs7_unpad(&result)
    } else {
        Ok(result)
    }
}

/// SM4 CBC decryption.
fn decrypt_cbc(
    key: &[u8],
    iv: &[u8],
    input: &[u8],
    padding: bool,
) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    if input.len() % 16 != 0 {
        return Err(OperationError::InvalidInput(
            "Input length must be a multiple of 16 bytes for CBC mode.".to_string(),
        ));
    }

    let mut result = Vec::with_capacity(input.len());
    let mut prev = *<&sm4::cipher::Block<Sm4>>::try_from(iv).unwrap();

    for chunk in input.chunks(16) {
        let current_cipher_block = *<&sm4::cipher::Block<Sm4>>::try_from(chunk).unwrap();
        let mut block = current_cipher_block.clone();
        cipher.decrypt_block(&mut block);
        for i in 0..16 {
            block[i] ^= prev[i];
        }
        prev = current_cipher_block;
        result.extend_from_slice(block.as_slice());
    }

    if padding {
        pkcs7_unpad(&result)
    } else {
        Ok(result)
    }
}

/// SM4 CFB (full-block feedback) decryption.
fn decrypt_cfb(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    let mut result = Vec::with_capacity(input.len());
    let mut register = *<&sm4::cipher::Block<Sm4>>::try_from(iv).unwrap();

    for chunk in input.chunks(16) {
        let mut encrypted_reg = register;
        cipher.encrypt_block(&mut encrypted_reg); // CFB uses encryption for decryption too

        let mut block = [0u8; 16];
        for (i, &byte) in chunk.iter().enumerate() {
            block[i] = byte ^ encrypted_reg[i];
        }

        // In CFB decryption, register is updated with ciphertext
        register = *<&sm4::cipher::Block<Sm4>>::try_from(chunk).unwrap();
        if chunk.len() < 16 {
            // Handle partial block if necessary, but CyberChef's implementation usually expects full blocks for CFB/OFB in this way.
            // For simplicity we follow the full block logic.
        }

        result.extend_from_slice(&block[..chunk.len()]);
    }
    Ok(result)
}

/// SM4 OFB decryption (same as encryption).
fn decrypt_ofb(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    let mut result = Vec::with_capacity(input.len());
    let mut register = *<&sm4::cipher::Block<Sm4>>::try_from(iv).unwrap();

    for chunk in input.chunks(16) {
        cipher.encrypt_block(&mut register);
        let keystream = register;

        for (i, &byte) in chunk.iter().enumerate() {
            result.push(byte ^ keystream[i]);
        }
    }
    Ok(result)
}

/// SM4 CTR decryption (same as encryption).
fn decrypt_ctr(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    let mut result = Vec::with_capacity(input.len());
    let mut counter = [0u8; 16];
    counter.copy_from_slice(iv);

    for chunk in input.chunks(16) {
        let mut counter_block = *<&sm4::cipher::Block<Sm4>>::try_from(&counter).unwrap();
        cipher.encrypt_block(&mut counter_block);

        for (i, &byte) in chunk.iter().enumerate() {
            result.push(byte ^ counter_block[i]);
        }

        // Increment counter (big-endian)
        let mut carry: u16 = 1;
        for i in (0..16).rev() {
            let val = counter[i] as u16 + carry;
            counter[i] = val as u8;
            carry = val >> 8;
        }
    }
    Ok(result)
}
