/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RC4 Drop operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RC4 Drop stream cipher operation.
///
/// RC4-drop defends against the Fluhrer-Mantin-Shamir attack by discarding
/// the first N dwords (N*4 bytes) of the keystream before encrypting.
pub struct RC4Drop;

/// Perform RC4-drop with the specified number of bytes to discard.
fn rc4_drop_crypt(key: &[u8], data: &[u8], drop_bytes: usize) -> Vec<u8> {
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

    // PRGA - discard first drop_bytes bytes
    let mut i: usize = 0;
    let mut j: usize = 0;
    for _ in 0..drop_bytes {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
    }

    // Encrypt data
    let mut output = Vec::with_capacity(data.len());
    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        output.push(byte ^ k);
    }
    output
}

impl Operation for RC4Drop {
    fn name(&self) -> &'static str {
        "RC4 Drop"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "It was discovered that the first few bytes of the RC4 keystream are strongly non-random and leak information about the key. We can defend against this attack by discarding the initial portion of the keystream. This modified algorithm is traditionally called RC4-drop."
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
            ArgSchema {
                name: "Number of dwords to drop",
                description:
                    "Number of 4-byte dwords to discard from keystream start (default: 192)",
                default_value: "192",
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
        let drop_dwords = args
            .get(3)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.get(3)
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(192usize);

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

        // CryptoJS RC4Drop counts dwords (4 bytes each)
        let drop_bytes = drop_dwords * 4;
        let result = rc4_drop_crypt(&key_bytes, &data, drop_bytes);

        match output_fmt {
            "Hex" => Ok(hex::encode(&result).into_bytes()),
            _ => Ok(result),
        }
    }
}
