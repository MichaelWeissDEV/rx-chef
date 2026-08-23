/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ROT13 Brute Force operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ROT13BruteForce;

impl Operation for ROT13BruteForce {
    fn name(&self) -> &'static str {
        "ROT13 Brute Force"
    }
    fn module(&self) -> &'static str {
        "Ciphers"
    }
    fn description(&self) -> &'static str {
        "Tries all meaningful Caesar rotation amounts, with optional character classes, sampling and a known-plaintext filter."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Rotate lower case chars",
                description: "Rotate ASCII lower-case letters",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Rotate upper case chars",
                description: "Rotate ASCII upper-case letters",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Rotate numbers",
                description: "Rotate ASCII decimal digits",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Sample length",
                description: "Maximum number of input bytes to rotate",
                default_value: "100",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Sample offset",
                description: "Byte offset at which the sample begins",
                default_value: "0",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Print amount",
                description: "Prefix each result with its rotation amount",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Crib (known plaintext string)",
                description: "Only retain rotations containing this text, case-insensitively",
                default_value: "",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        S
    }
    fn input_type(&self) -> DataType {
        DataType::Bytes
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let rotate_lower = args.first().and_then(ArgValue::as_bool).unwrap_or(true);
        let rotate_upper = args.get(1).and_then(ArgValue::as_bool).unwrap_or(true);
        let rotate_numbers = args.get(2).and_then(ArgValue::as_bool).unwrap_or(false);
        let sample_len = args.get(3).and_then(ArgValue::as_usize).unwrap_or(100);
        let sample_offset = args.get(4).and_then(ArgValue::as_usize).unwrap_or(0);
        let print_amount = args.get(5).and_then(ArgValue::as_bool).unwrap_or(true);
        let crib = args
            .get(6)
            .and_then(ArgValue::as_str)
            .unwrap_or("")
            .to_lowercase();
        let end = sample_offset.saturating_add(sample_len).min(input.len());
        let sample = input.get(sample_offset..end).unwrap_or(&[]);
        let mut results = Vec::new();

        for amount in 1u8..26 {
            let mut rotated = sample.to_vec();
            for byte in &mut rotated {
                *byte = if rotate_lower && byte.is_ascii_lowercase() {
                    (*byte - b'a' + amount) % 26 + b'a'
                } else if rotate_upper && byte.is_ascii_uppercase() {
                    (*byte - b'A' + amount) % 26 + b'A'
                } else if rotate_numbers && byte.is_ascii_digit() {
                    (*byte - b'0' + amount) % 10 + b'0'
                } else {
                    *byte
                };
            }
            let text = String::from_utf8(rotated).map_err(|_| {
                OperationError::InvalidInput("Rotated sample is not valid UTF-8".into())
            })?;
            if text.to_lowercase().contains(&crib) {
                let escaped = text
                    .chars()
                    .map(|character| {
                        if ('\u{0009}'..='\u{0010}').contains(&character) {
                            char::from_u32(0xe000 + character as u32).unwrap()
                        } else {
                            character
                        }
                    })
                    .collect::<String>();
                results.push(if print_amount {
                    format!("Amount = {:>2}: {escaped}", amount)
                } else {
                    escaped
                });
            }
        }
        Ok(results.join("\n").into_bytes())
    }
}
