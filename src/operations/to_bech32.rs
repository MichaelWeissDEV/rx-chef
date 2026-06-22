/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Bech32 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Bech32 operation
pub struct ToBech32;

const CHARSET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const BECH32_CONST: u32 = 1;
const BECH32M_CONST: u32 = 0x2bc830a3;
const GENERATOR: [u32; 5] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];

fn polymod(values: &[u8]) -> u32 {
    let mut chk: u32 = 1;
    for &v in values {
        let top = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ (v as u32);
        for i in 0..5 {
            if ((top >> i) & 1) == 1 {
                chk ^= GENERATOR[i];
            }
        }
    }
    chk
}

fn hrp_expand(hrp: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(hrp.len() * 2 + 1);
    for c in hrp.chars() {
        result.push((c as u32 >> 5) as u8);
    }
    result.push(0);
    for c in hrp.chars() {
        result.push((c as u32 & 31) as u8);
    }
    result
}

fn create_checksum(hrp: &str, data: &[u8], encoding: &str) -> Vec<u8> {
    let constant = if encoding == "Bech32m" {
        BECH32M_CONST
    } else {
        BECH32_CONST
    };
    let mut values = hrp_expand(hrp);
    values.extend_from_slice(data);
    values.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
    let mod_res = polymod(&values) ^ constant;
    let mut result = Vec::with_capacity(6);
    for i in 0..6 {
        result.push(((mod_res >> (5 * (5 - i))) & 31) as u8);
    }
    result
}

fn to_words(data: &[u8]) -> Vec<u8> {
    let mut value: u32 = 0;
    let mut bits: u32 = 0;
    let mut result = Vec::new();

    for &b in data {
        value = (value << 8) | (b as u32);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            result.push(((value >> bits) & 31) as u8);
        }
    }

    if bits > 0 {
        result.push(((value << (5 - bits)) & 31) as u8);
    }

    result
}

impl Operation for ToBech32 {
    fn name(&self) -> &'static str {
        "To Bech32"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Bech32 is an encoding scheme primarily used for Bitcoin SegWit addresses (BIP-0173). It uses a 32-character alphabet that excludes easily confused characters (1, b, i, o) and includes a checksum for error detection.<br><br>Bech32m (BIP-0350) is an updated version that fixes a weakness in the original Bech32 checksum and is used for Bitcoin Taproot addresses.<br><br>The Human-Readable Part (HRP) identifies the network or purpose (e.g., 'bc' for Bitcoin mainnet, 'tb' for testnet, 'age' for AGE encryption keys).<br><br>Maximum output length is 90 characters as per specification."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Human-Readable Part (HRP)",
                description: "Human-Readable Part (HRP)",
                default_value: "bc",
            },
            ArgSchema {
                name: "Encoding",
                description: "Encoding",
                default_value: "Bech32",
            },
            ArgSchema {
                name: "Input Format",
                description: "Input Format",
                default_value: "Raw bytes",
            },
            ArgSchema {
                name: "Mode",
                description: "Mode",
                default_value: "Generic",
            },
            ArgSchema {
                name: "Witness Version",
                description: "Witness Version (0-16)",
                default_value: "0",
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
        let hrp = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("bc")
            .to_lowercase();
        let encoding = args.get(1).and_then(|a| a.as_str()).unwrap_or("Bech32");
        let input_format = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw bytes");
        let mode = args.get(3).and_then(|a| a.as_str()).unwrap_or("Generic");
        let witness_version = args.get(4).and_then(|a| a.as_f64()).unwrap_or(0.0) as u8;

        if hrp.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "HRP".to_string(),
                reason: "HRP cannot be empty".to_string(),
            });
        }

        let input_bytes = if input_format == "Hex" {
            let hex_str = String::from_utf8_lossy(&input).replace(char::is_whitespace, "");
            hex::decode(hex_str)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let words = if mode == "Bitcoin SegWit" {
            if witness_version > 16 {
                return Err(OperationError::InvalidArgument {
                    name: "Witness Version".to_string(),
                    reason: "Must be 0-16".to_string(),
                });
            }
            let mut w = vec![witness_version];
            w.extend(to_words(&input_bytes));
            w
        } else {
            to_words(&input_bytes)
        };

        let checksum = create_checksum(&hrp, &words, encoding);
        let mut result = hrp;
        result.push('1');
        for &w in words.iter().chain(checksum.iter()) {
            result.push(CHARSET[w as usize] as char);
        }

        if result.len() > 90 {
            return Err(OperationError::ProcessingError(
                "Encoded string exceeds maximum length of 90 characters".to_string(),
            ));
        }

        Ok(result.into_bytes())
    }
}
