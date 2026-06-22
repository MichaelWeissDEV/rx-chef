/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RC4 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RC4 stream cipher operation.
///
/// RC4 (also known as ARC4) is a widely-used stream cipher designed by Ron Rivest.
/// This implementation applies KSA (Key Scheduling Algorithm) and PRGA (Pseudo-Random
/// Generation Algorithm) directly.
pub struct RC4;

/// Perform RC4 key setup and generate keystream XOR'd with plaintext.
fn rc4_crypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    // KSA
    let mut s: [u8; 256] = [0u8; 256];
    for i in 0..256usize {
        s[i] = i as u8;
    }
    let mut j: usize = 0;
    for i in 0..256usize {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }

    // PRGA
    let mut output = Vec::with_capacity(data.len());
    let mut i: usize = 0;
    let mut j: usize = 0;
    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        output.push(byte ^ k);
    }
    output
}

impl Operation for RC4 {
    fn name(&self) -> &'static str {
        "RC4"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "RC4 (also known as ARC4) is a widely-used stream cipher designed by Ron Rivest. It is used in popular protocols such as SSL and WEP. Although remarkable for its simplicity and speed, the algorithm's history doesn't inspire confidence in its security."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Passphrase/key as UTF-8 string or hex (prefix 0x for hex)",
                default_value: "",
            },
            ArgSchema {
                name: "Input format",
                description: "Input encoding: Raw or Hex",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Output format",
                description: "Output encoding: Raw or Hex",
                default_value: "Hex",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let input_fmt = args.get(1).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_fmt = args.get(2).and_then(|a| a.as_str()).unwrap_or("Hex");

        if key_str.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Key must not be empty".to_string(),
            });
        }

        let key_bytes: Vec<u8> = if key_str.starts_with("0x") {
            hex::decode(&key_str[2..]).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: e.to_string(),
            })?
        } else {
            key_str.as_bytes().to_vec()
        };

        let data: Vec<u8> = match input_fmt {
            "Hex" => {
                hex::decode(&input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
            }
            _ => input,
        };

        let result = rc4_crypt(&key_bytes, &data);

        match output_fmt {
            "Hex" => Ok(hex::encode(&result).into_bytes()),
            _ => Ok(result),
        }
    }
}
