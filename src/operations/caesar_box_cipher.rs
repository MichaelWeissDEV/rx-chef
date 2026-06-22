/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Caesar Box Cipher operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Caesar Box Cipher operation
pub struct CaesarBoxCipher;

impl Operation for CaesarBoxCipher {
    fn name(&self) -> &'static str {
        "Caesar Box Cipher"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Caesar Box is a transposition cipher used in the Roman Empire, in which letters of the message are written in rows in a square (or a rectangle) and then, read by column."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Box Height",
            description: "Number of rows in the transposition box",
            default_value: "1",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let table_height = if let Some(arg) = args.first() {
            arg.as_f64().unwrap_or(1.0) as usize
        } else {
            1
        };

        if table_height == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Box Height".to_string(),
                reason: "Box Height must be greater than 0".to_string(),
            });
        }

        // Remove spaces
        input_str = input_str.replace(' ', "");

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let table_width = (input_str.chars().count() + table_height - 1) / table_height;

        // Pad with null bytes if needed
        let total_size = table_height * table_width;
        let mut chars: Vec<char> = input_str.chars().collect();
        while chars.len() < total_size {
            chars.push('\0');
        }

        let mut result = String::new();
        for i in 0..table_height {
            for j in (i..chars.len()).step_by(table_height) {
                if chars[j] != '\0' {
                    result.push(chars[j]);
                }
            }
        }

        Ok(result.into_bytes())
    }
}
