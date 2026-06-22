/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CMAC operation.
 * -----------------------------------------------------------------------------
 */

use aes::{Aes128, Aes192, Aes256};
use base64::{engine::general_purpose::STANDARD, Engine};
use cipher::{BlockEncrypt, KeyInit};
use generic_array::GenericArray;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CMAC operation
///
/// CMAC (Cipher-based Message Authentication Code) is a block-cipher based
/// message authentication code algorithm.
///
/// RFC4493 defines AES-CMAC that uses AES encryption with a 128-bit key.
/// This implementation supports AES with 128, 192, or 256-bit keys.
pub struct Cmac;

impl Operation for Cmac {
    fn name(&self) -> &'static str {
        "CMAC"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "CMAC is a block-cipher based message authentication code algorithm. \
         AES-CMAC uses AES encryption with a 128, 192, or 256-bit key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Encryption key (Hex, UTF8, Latin1, or Base64)",
                default_value: "",
            },
            ArgSchema {
                name: "Encryption algorithm",
                description: "Encryption algorithm (AES)",
                default_value: "AES",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Binary
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_bytes = parse_key_arg(args.first())?;
        let algo = args.get(1).and_then(|a| a.as_str()).unwrap_or("AES");

        if algo != "AES" {
            return Err(OperationError::InvalidArgument {
                name: "Encryption algorithm".to_string(),
                reason: format!(
                    "Unsupported encryption algorithm: {}. Only AES is supported.",
                    algo
                ),
            });
        }

        if key_bytes.len() != 16 && key_bytes.len() != 24 && key_bytes.len() != 32 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!(
                    "The key for AES must be either 16, 24, or 32 bytes (currently {} bytes)",
                    key_bytes.len()
                ),
            });
        }

        let cmac = compute_aes_cmac(&key_bytes, &input)?;
        Ok(hex::encode(cmac).into_bytes())
    }
}

/// Parse key argument (supports Hex, UTF8, Latin1, Base64, and raw bytes)
fn parse_key_arg(arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
    match arg {
        Some(ArgValue::Str(s)) => {
            if s.is_empty() {
                Ok(vec![])
            } else if s.starts_with("0x") || is_hex_string(s) {
                let hex_str = if s.starts_with("0x") { &s[2..] } else { s };
                hex::decode(hex_str).map_err(|e| OperationError::InvalidArgument {
                    name: "Key".to_string(),
                    reason: format!("Invalid hex: {}", e),
                })
            } else if let Ok(decoded) = STANDARD.decode(s) {
                Ok(decoded)
            } else {
                Ok(s.as_bytes().to_vec())
            }
        }
        Some(ArgValue::Bytes(b)) => Ok(b.clone()),
        _ => Ok(vec![]),
    }
}

/// Check if a string is a valid hex string
fn is_hex_string(s: &str) -> bool {
    if s.is_empty() || s.len() % 2 != 0 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Compute AES-CMAC
fn compute_aes_cmac(key: &[u8], input: &[u8]) -> Result<Vec<u8>, OperationError> {
    const BLOCK_SIZE: usize = 16;
    const RB: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x87];

    // L = AES(0^b)
    let l = encrypt_block(key, &vec![0u8; BLOCK_SIZE])?;

    // K1 = left_shift(L)
    let k1 = left_shift(&l, BLOCK_SIZE);
    let k1 = if (l[0] & 0x80) != 0 {
        xor(&k1, &RB, BLOCK_SIZE)
    } else {
        k1
    };

    // K2 = left_shift(K1)
    let k2 = left_shift(&k1, BLOCK_SIZE);
    let k2 = if (k1[0] & 0x80) != 0 {
        xor(&k2, &RB, BLOCK_SIZE)
    } else {
        k2
    };

    let n = (input.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;

    let (last_block, _) = if n == 0 {
        // For empty input, last block = K2 XOR 0x80
        let mut data = k2.clone();
        data[0] ^= 0x80;
        (data, false)
    } else {
        let last_pos = (n - 1) * BLOCK_SIZE;
        let last_input = &input[last_pos..];

        if last_input.len() == BLOCK_SIZE {
            (xor(last_input, &k1, BLOCK_SIZE), true)
        } else {
            let mut data = vec![0u8; BLOCK_SIZE];
            data[0..last_input.len()].copy_from_slice(last_input);
            data[last_input.len()] = 0x80;
            (xor(&data, &k2, BLOCK_SIZE), false)
        }
    };

    // X = 0^b
    let mut x = vec![0u8; BLOCK_SIZE];
    let mut y = vec![0u8; BLOCK_SIZE];

    // Process blocks 0 to n-2
    for i in 0..n.saturating_sub(1) {
        let pos = i * BLOCK_SIZE;
        let block = &input[pos..pos + BLOCK_SIZE];
        xor_block(block, &x, &mut y);
        x = encrypt_block(key, &y)?;
    }

    // Y = last block XOR X (always XOR, even for n=0)
    xor_block(&last_block, &x, &mut y);

    // CMAC = encrypt(Y)
    encrypt_block(key, &y)
}

/// Encrypt a single block using AES
fn encrypt_block(key: &[u8], block: &[u8]) -> Result<Vec<u8>, OperationError> {
    let block_size = 16;
    let mut result = vec![0u8; block_size];

    match key.len() {
        16 => {
            let key_arr = GenericArray::from_slice(key);
            let cipher = Aes128::new(key_arr);
            let mut block_copy = GenericArray::clone_from_slice(block);
            cipher.encrypt_block(&mut block_copy);
            result.copy_from_slice(block_copy.as_slice());
        }
        24 => {
            let key_arr = GenericArray::from_slice(key);
            let cipher = Aes192::new(key_arr);
            let mut block_copy = GenericArray::clone_from_slice(block);
            cipher.encrypt_block(&mut block_copy);
            result.copy_from_slice(block_copy.as_slice());
        }
        32 => {
            let key_arr = GenericArray::from_slice(key);
            let cipher = Aes256::new(key_arr);
            let mut block_copy = GenericArray::clone_from_slice(block);
            cipher.encrypt_block(&mut block_copy);
            result.copy_from_slice(block_copy.as_slice());
        }
        _ => {
            return Err(OperationError::ProcessingError(
                "Invalid key length for AES".into(),
            ))
        }
    }

    Ok(result)
}

/// Left shift a byte array by 1 bit
fn left_shift(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut result = vec![0u8; block_size];
    let mut carry = 0u8;

    for i in (0..block_size).rev() {
        result[i] = (data[i] << 1) | carry;
        carry = data[i] >> 7;
    }

    result
}

/// XOR two byte arrays
fn xor(a: &[u8], b: &[u8], block_size: usize) -> Vec<u8> {
    let mut result = vec![0u8; block_size];
    for i in 0..block_size {
        result[i] = a[i] ^ b[i];
    }
    result
}

/// XOR block with accumulator
fn xor_block(a: &[u8], acc: &[u8], out: &mut [u8]) {
    for (i, &val) in a.iter().enumerate() {
        out[i] = val ^ acc[i];
    }
}
