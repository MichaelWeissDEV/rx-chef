/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base62 operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use num_traits::Zero;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base62 operation
pub struct FromBase62;

impl Operation for FromBase62 {
    fn name(&self) -> &'static str {
        "From Base62"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base62 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. The high number base results in shorter strings than with the decimal or hexadecimal system."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Alphabet",
            description: "The Base62 alphabet",
            default_value: "0-9A-Za-z",
        }];
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

        let alphabet_arg = args.first().and_then(|v| v.as_str()).unwrap_or("0-9A-Za-z");
        let alphabet_str = expand_base_alphabet(alphabet_arg);

        if alphabet_str.len() != 62 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!(
                    "Alphabet must be 62 characters long, got {}",
                    alphabet_str.len()
                ),
            });
        }

        let mut res = BigUint::zero();
        let base = BigUint::from(62u32);

        for c in input_str.chars() {
            if let Some(idx) = alphabet_str.find(c) {
                res = res * &base + BigUint::from(idx as u32);
            }
        }

        Ok(res.to_bytes_be())
    }
}

fn expand_base_alphabet(alphabet: &str) -> String {
    if alphabet == "0-9A-Za-z" {
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string()
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
