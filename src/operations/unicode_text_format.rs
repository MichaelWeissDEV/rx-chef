/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Unicode Text Format operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Unicode Text Format operation
pub struct UnicodeTextFormat;

impl Operation for UnicodeTextFormat {
    fn name(&self) -> &'static str {
        "Unicode Text Format"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Adds Unicode combining characters to change formatting of plaintext."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Underline",
                description: "Underline",
                default_value: "false",
            },
            ArgSchema {
                name: "Strikethrough",
                description: "Strikethrough",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let underline = args.first().and_then(|a| a.as_bool()).unwrap_or(false);
        let strikethrough = args.get(1).and_then(|a| a.as_bool()).unwrap_or(false);

        if !underline && !strikethrough {
            return Ok(input);
        }

        let input_str = String::from_utf8_lossy(&input);
        let mut result = String::with_capacity(input.len() * 2);

        for c in input_str.chars() {
            result.push(c);
            if strikethrough {
                result.push('\u{0336}');
            }
            if underline {
                result.push('\u{0332}');
            }
        }

        Ok(result.into_bytes())
    }
}
