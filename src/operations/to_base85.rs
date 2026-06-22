/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base85 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToBase85;

impl Operation for ToBase85 {
    fn name(&self) -> &'static str {
        "To Base85"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Base85 (also called Ascii85) is a notation for encoding arbitrary byte data. It is usually more efficient than Base64."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base85 alphabet",
                default_value: "!-u",
            },
            ArgSchema {
                name: "Include delimiter",
                description: "Adds a <~ and ~> delimiter to the start and end of the data.",
                default_value: "false",
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
        let alphabet_arg = args.first().and_then(|v| v.as_str()).unwrap_or("!-u");
        let include_delim = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

        let alphabet = expand_alphabet(alphabet_arg);
        let alphabet_chars: Vec<char> = alphabet.chars().collect();
        if alphabet_chars.len() != 85 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!(
                    "Alphabet must be of length 85, got {}",
                    alphabet_chars.len()
                ),
            });
        }

        let is_standard = alphabet == "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstu";

        let mut result = String::new();
        if input.is_empty() {
            return Ok(Vec::new());
        }

        for i in (0..input.len()).step_by(4) {
            let block = ((input[i] as u32) << 24)
                | ((input.get(i + 1).cloned().unwrap_or(0) as u32) << 16)
                | ((input.get(i + 2).cloned().unwrap_or(0) as u32) << 8)
                | (input.get(i + 3).cloned().unwrap_or(0) as u32);

            if is_standard && block == 0 && i + 4 <= input.len() {
                result.push('z');
            } else {
                let mut temp_block = block;
                let mut digits = Vec::with_capacity(5);
                for _ in 0..5 {
                    digits.push((temp_block % 85) as usize);
                    temp_block /= 85;
                }
                digits.reverse();

                let num_chars = if i + 4 > input.len() {
                    (input.len() - i) + 1
                } else {
                    5
                };

                for &digit in &digits[..num_chars] {
                    result.push(alphabet_chars[digit]);
                }
            }
        }

        let final_result = if include_delim {
            format!("<~{}~>", result)
        } else {
            result
        };

        Ok(final_result.into_bytes())
    }
}

fn expand_alphabet(alphabet: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = alphabet.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i < chars.len() - 2 && chars[i + 1] == '-' && (i == 0 || chars[i - 1] != '\\') {
            let start = chars[i] as u32;
            let end = chars[i + 2] as u32;
            for code in start..=end {
                if let Some(c) = std::char::from_u32(code) {
                    result.push(c);
                }
            }
            i += 3;
        } else if i < chars.len() - 1 && chars[i] == '\\' && chars[i + 1] == '-' {
            result.push('-');
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
