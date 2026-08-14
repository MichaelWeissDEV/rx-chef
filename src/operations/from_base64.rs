/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base64 operation.
 * -----------------------------------------------------------------------------
 */

use base64::{
    alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
    Engine as _,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base64 operation
pub struct FromBase64;

impl Operation for FromBase64 {
    fn name(&self) -> &'static str {
        "From Base64"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. This operation decodes data from an ASCII Base64 string back into its raw format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base64 alphabet",
                default_value: "A-Za-z0-9+/=",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Remove non-alphabet chars",
                description: "Remove characters not in the alphabet before decoding",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Strict mode",
                description: "Throw an error if the input is not perfectly formatted",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
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

        let alphabet_arg = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("A-Za-z0-9+/=");
        let remove_non_alph = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let strict = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

        let alphabet_str = expand_alphabet(alphabet_arg);

        let clean_input = if remove_non_alph {
            input_str
                .chars()
                .filter(|&c| alphabet_str.contains(c) || c == '=')
                .collect::<String>()
        } else {
            input_str
        };

        if alphabet_str.len() != 64 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!(
                    "Alphabet must be 64 characters long, got {}",
                    alphabet_str.len()
                ),
            });
        }

        let custom_alphabet = alphabet::Alphabet::new(&alphabet_str).map_err(|e| {
            OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!("Invalid alphabet: {}", e),
            }
        })?;

        // In strict mode require canonical padding; otherwise decode forgivingly
        // (accept missing padding and non-zero trailing bits), matching the
        // behaviour of CyberChef's non-strict From Base64.
        let config = if strict {
            GeneralPurposeConfig::new()
                .with_decode_padding_mode(DecodePaddingMode::RequireCanonical)
        } else {
            GeneralPurposeConfig::new()
                .with_decode_padding_mode(DecodePaddingMode::Indifferent)
                .with_decode_allow_trailing_bits(true)
        };
        let engine = GeneralPurpose::new(&custom_alphabet, config);

        engine
            .decode(clean_input.trim())
            .map_err(|e| OperationError::InvalidInput(format!("Base64 decode failed: {}", e)))
    }
}

fn expand_alphabet(alphabet: &str) -> String {
    if alphabet == "A-Za-z0-9+/=" || alphabet == "A-Za-z0-9+-" || alphabet == "A-Za-z0-9-_" {
        // Special common cases
        match alphabet {
            "A-Za-z0-9+/=" => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string()
            }
            "A-Za-z0-9+-" => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-".to_string()
            }
            "A-Za-z0-9-_" => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_".to_string()
            }
            _ => alphabet.to_string(),
        }
    } else {
        // Simple range expansion logic (enough for most CyberChef cases)
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
