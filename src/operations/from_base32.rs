/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base32 operation.
 * -----------------------------------------------------------------------------
 */

use data_encoding::Specification;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base32 operation
pub struct FromBase32;

impl Operation for FromBase32 {
    fn name(&self) -> &'static str {
        "From Base32"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base32 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. It uses a smaller set of characters than Base64, usually the uppercase alphabet and the numbers 2 to 7."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base32 alphabet",
                default_value: "A-Z2-7",
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

        let alphabet_arg = args.first().and_then(|v| v.as_str()).unwrap_or("A-Z2-7");
        let remove_non_alph = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let alphabet_str = expand_base32_alphabet(alphabet_arg);

        let clean_input = if remove_non_alph {
            input_str
                .chars()
                .filter(|&c| alphabet_str.contains(c) || c == '=')
                .collect::<String>()
        } else {
            input_str
        };

        let mut spec = Specification::new();
        spec.symbols = alphabet_str;
        spec.padding = Some('=');
        let encoding = spec
            .encoding()
            .map_err(|e| OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!("Invalid alphabet: {}", e),
            })?;

        encoding
            .decode(clean_input.trim().as_bytes())
            .map_err(|e| OperationError::InvalidInput(format!("Base32 decode failed: {}", e)))
    }
}

fn expand_base32_alphabet(alphabet: &str) -> String {
    if alphabet == "A-Z2-7" {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".to_string()
    } else if alphabet == "0-9A-V" {
        "0123456789ABCDEFGHIJKLMNOPQRSTUV".to_string()
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
                if chars[i] != '=' {
                    result.push(chars[i]);
                }
                i += 1;
            }
        }
        result
    }
}
