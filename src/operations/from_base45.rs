/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base45 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base45 operation
pub struct FromBase45;

impl Operation for FromBase45 {
    fn name(&self) -> &'static str {
        "From Base45"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base45 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. The high number base results in shorter strings than with the decimal or hexadecimal system. Base45 is optimized for usage with QR codes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base45 alphabet",
                default_value: "0-9A-Z $%*+-./:",
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
            .unwrap_or("0-9A-Z $%*+-./:");
        let remove_non_alph = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let alphabet_str = expand_base45_alphabet(alphabet_arg);

        let clean_input = if remove_non_alph {
            input_str
                .chars()
                .filter(|&c| alphabet_str.contains(c))
                .collect::<String>()
        } else {
            input_str
        };

        let mut res = Vec::new();
        let chars: Vec<char> = clean_input.chars().collect();

        for chunk in chars.chunks(3) {
            let mut b = 0u32;
            let mut power = 1u32;
            for &c in chunk {
                let idx = alphabet_str.find(c).ok_or_else(|| {
                    OperationError::InvalidInput(format!("Character not in alphabet: '{}'", c))
                })? as u32;
                b += idx * power;
                power *= 45;
            }

            if b > 65535 {
                return Err(OperationError::InvalidInput(format!(
                    "Triplet too large: '{}'",
                    chunk.iter().collect::<String>()
                )));
            }

            if chunk.len() > 2 {
                res.push((b >> 8) as u8);
            }
            res.push((b & 0xff) as u8);
        }

        Ok(res)
    }
}

fn expand_base45_alphabet(alphabet: &str) -> String {
    if alphabet == "0-9A-Z $%*+-./:" {
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:".to_string()
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
