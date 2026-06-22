/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ROT13 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ROT13;

impl Operation for ROT13 {
    fn name(&self) -> &'static str {
        "ROT13"
    }
    fn module(&self) -> &'static str {
        "Ciphers"
    }
    fn description(&self) -> &'static str {
        "Rotates each letter by 13 positions in the alphabet. ROT13 is a simple Caesar cipher."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Rotate lower case chars",
                description: "Apply rotation to a-z",
                default_value: "true",
            },
            ArgSchema {
                name: "Rotate upper case chars",
                description: "Apply rotation to A-Z",
                default_value: "true",
            },
            ArgSchema {
                name: "Rotate digits",
                description: "Apply ROT5 to digits 0-9",
                default_value: "false",
            },
            ArgSchema {
                name: "Amount",
                description: "Amount to rotate (default 13)",
                default_value: "13",
            },
        ];
        S
    }
    fn input_type(&self) -> DataType {
        DataType::String
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let lower = args
            .get(0)
            .and_then(|a| a.as_str())
            .map(|s| s != "false")
            .unwrap_or(true);
        let upper = args
            .get(1)
            .and_then(|a| a.as_str())
            .map(|s| s != "false")
            .unwrap_or(true);
        let digits = args
            .get(2)
            .and_then(|a| a.as_str())
            .map(|s| s == "true")
            .unwrap_or(false);
        let amount = args
            .get(3)
            .and_then(|a| a.as_str())
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(13);

        let text = String::from_utf8_lossy(&input);
        let result: String = text
            .chars()
            .map(|c| {
                if lower && c.is_ascii_lowercase() {
                    (((c as u8 - b'a' + amount) % 26) + b'a') as char
                } else if upper && c.is_ascii_uppercase() {
                    (((c as u8 - b'A' + amount) % 26) + b'A') as char
                } else if digits && c.is_ascii_digit() {
                    (((c as u8 - b'0' + (amount % 10)) % 10) + b'0') as char
                } else {
                    c
                }
            })
            .collect();
        Ok(result.into_bytes())
    }
}
