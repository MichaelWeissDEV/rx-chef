/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SM4 Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use sm4::{
    cipher::{BlockCipherEncrypt, KeyInit},
    Sm4,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SM4 Encrypt operation
///
/// SM4 is a 128-bit block cipher, currently established as a national
/// standard (GB/T 32907-2016) of China. Supports ECB, CBC, CFB, OFB, and CTR
/// modes. PKCS#7 padding is used for ECB and CBC unless the NoPadding variant
/// is selected.
pub struct Sm4Encrypt;

impl Operation for Sm4Encrypt {
    fn name(&self) -> &'static str {
        "SM4 Encrypt"
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
                description: "Encryption key (16 bytes, 128 bits)",
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
        let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let iv_str = args.get(1).and_then(|a| a.as_str()).unwrap_or("");
        let mode = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Hex");

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
            "ECB" => encrypt_ecb(&key, &input_bytes, !no_padding),
            "CBC" => encrypt_cbc(&key, &iv, &input_bytes, !no_padding),
            "CFB" => encrypt_cfb(&key, &iv, &input_bytes),
            "OFB" => encrypt_ofb(&key, &iv, &input_bytes),
            "CTR" => encrypt_ctr(&key, &iv, &input_bytes),
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

/// Apply PKCS#7 padding to reach a multiple of 16 bytes.
fn pkcs7_pad(data: &[u8]) -> Vec<u8> {
    let block_size = 16usize;
    let pad_len = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    padded
}

/// SM4 ECB encryption.
fn encrypt_ecb(key: &[u8], input: &[u8], padding: bool) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    let data = if padding {
        pkcs7_pad(input)
    } else {
        if input.len() % 16 != 0 {
            return Err(OperationError::InvalidInput(
                "Input length must be a multiple of 16 bytes for ECB/NoPadding mode.".to_string(),
            ));
        }
        input.to_vec()
    };

    let mut result = Vec::with_capacity(data.len());
    for chunk in data.chunks(16) {
        let mut block = *<&sm4::cipher::Block<Sm4>>::try_from(chunk).unwrap();
        cipher.encrypt_block(&mut block);
        result.extend_from_slice(block.as_slice());
    }
    Ok(result)
}

/// SM4 CBC encryption.
fn encrypt_cbc(
    key: &[u8],
    iv: &[u8],
    input: &[u8],
    padding: bool,
) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    let data = if padding {
        pkcs7_pad(input)
    } else {
        if input.len() % 16 != 0 {
            return Err(OperationError::InvalidInput(
                "Input length must be a multiple of 16 bytes for CBC/NoPadding mode.".to_string(),
            ));
        }
        input.to_vec()
    };

    let mut result = Vec::with_capacity(data.len());
    let mut prev = *<&sm4::cipher::Block<Sm4>>::try_from(iv).unwrap();

    for chunk in data.chunks(16) {
        let mut block = *<&sm4::cipher::Block<Sm4>>::try_from(chunk).unwrap();
        for i in 0..16 {
            block[i] ^= prev[i];
        }
        cipher.encrypt_block(&mut block);
        prev = block;
        result.extend_from_slice(block.as_slice());
    }
    Ok(result)
}

/// SM4 CFB (full-block feedback) encryption.
fn encrypt_cfb(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
    let cipher = Sm4::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("Invalid key length: {}", e)))?;

    let mut result = Vec::with_capacity(input.len());
    let mut register = *<&sm4::cipher::Block<Sm4>>::try_from(iv).unwrap();

    for chunk in input.chunks(16) {
        let mut encrypted_reg = register;
        cipher.encrypt_block(&mut encrypted_reg);

        let mut block = [0u8; 16];
        for (i, &byte) in chunk.iter().enumerate() {
            block[i] = byte ^ encrypted_reg[i];
        }

        // In full-block CFB, register is updated with ciphertext
        register = *<&sm4::cipher::Block<Sm4>>::try_from(&block[..16]).unwrap();

        result.extend_from_slice(&block[..chunk.len()]);
    }
    Ok(result)
}

/// SM4 OFB encryption.
fn encrypt_ofb(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
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

/// SM4 CTR encryption/decryption (symmetric).
fn encrypt_ctr(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
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
