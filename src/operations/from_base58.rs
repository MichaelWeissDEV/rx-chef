/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base58 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base58 operation
pub struct FromBase58;

impl Operation for FromBase58 {
    fn name(&self) -> &'static str {
        "From Base58"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base58 (similar to Base64) is a notation for encoding arbitrary byte data. It differs from Base64 by removing easily misread characters (i.e. l, I, 0 and O) to improve human readability. This operation decodes data from an ASCII string back into its raw form."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base58 alphabet",
                default_value: "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
            },
            ArgSchema {
                name: "Remove non-alphabet chars",
                description: "Remove characters not in the alphabet before decoding",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let alphabet_arg = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
        let remove_non_alph = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let alphabet_str = expand_base58_alphabet(alphabet_arg);

        if alphabet_str.len() != 58 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!(
                    "Alphabet must be 58 characters long, got {}",
                    alphabet_str.len()
                ),
            });
        }

        let clean_input = if remove_non_alph {
            input_str
                .chars()
                .filter(|&c| alphabet_str.contains(c))
                .collect::<String>()
        } else {
            input_str
        };

        let alphabet_bytes: Vec<u8> = alphabet_str.as_bytes().to_vec();
        let alphabet_bytes_len = alphabet_bytes.len();
        let alphabet_arr: [u8; 58] =
            alphabet_bytes
                .try_into()
                .map_err(|_| OperationError::InvalidArgument {
                    name: "Alphabet".to_string(),
                    reason: format!("Alphabet must be 58 bytes long, got {}", alphabet_bytes_len),
                })?;

        let alpha =
            bs58::Alphabet::new(&alphabet_arr).map_err(|_| OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: "Invalid alphabet for Base58".to_string(),
            })?;

        bs58::decode(clean_input.trim())
            .with_alphabet(&alpha)
            .into_vec()
            .map_err(|e| OperationError::InvalidInput(format!("Base58 decode failed: {}", e)))
    }
}

fn expand_base58_alphabet(alphabet: &str) -> String {
    if alphabet == "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz" {
        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string()
    } else if alphabet == "rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz" {
        "rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz".to_string()
    } else {
        // Simple range expansion logic
        let mut result = String::new();
        let chars: Vec<char> = alphabet.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if i + 2 < chars.len() && chars[i + 1] == '-' {
                let start = chars[i] as u32;
                let end = chars[i + 2] as u32;
                for code in start..=end {
                    result.push(std::char::from_u32(code).unwrap());
                }
                i += 3;
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
    }
}
