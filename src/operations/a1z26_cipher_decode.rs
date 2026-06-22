/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the A1Z26 Cipher Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// A1Z26 Cipher Decode operation
///
/// Converts alphabet order numbers into their corresponding alphabet character.
/// e.g. `1` becomes `a` and `2` becomes `b`.
pub struct A1Z26CipherDecode;

impl Operation for A1Z26CipherDecode {
    fn name(&self) -> &'static str {
        "A1Z26 Cipher Decode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Converts alphabet order numbers into their corresponding alphabet character.<br><br>e.g. <code>1</code> becomes <code>a</code> and <code>2</code> becomes <code>b</code>."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between numbers",
                default_value: "Space",
            },
            ArgSchema {
                name: "Check format",
                description: "Automatically detect delimiter from input format",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        // Determine delimiter from args or auto-detect
        let delim = if !args.is_empty() {
            args[0].as_str().unwrap_or("Space")
        } else {
            "Space"
        };

        let delimiter_char = match delim {
            "Space" => ' ',
            "Comma" => ',',
            "Semi-colon" => ';',
            "Colon" => ':',
            "Line feed" => '\n',
            "CRLF" => '\r',
            _ => ' ',
        };

        // Split input by delimiter
        let mut latin1 = String::new();
        for part in input_str.split(delimiter_char) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let num: usize = part.parse().map_err(|_| {
                OperationError::InvalidInput(
                    "Error: all numbers must be between 1 and 26.".to_string(),
                )
            })?;

            if !(1..=26).contains(&num) {
                return Err(OperationError::InvalidInput(
                    "Error: all numbers must be between 1 and 26.".to_string(),
                ));
            }

            // Convert number to character (1 -> 'a', 2 -> 'b', etc.)
            let chr = (num as u8 + 96) as char;
            latin1.push(chr);
        }

        Ok(latin1.into_bytes())
    }
}
