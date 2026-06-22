/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Binary operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Binary operation - converts a binary string back to raw bytes.
pub struct FromBinary;

impl Operation for FromBinary {
    fn name(&self) -> &'static str {
        "From Binary"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a binary string back into its raw form. e.g. '01001000 01101001' becomes 'Hi'"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The delimiter between binary groups (Space, Comma, Semi-colon, Colon, Line feed, CRLF, None)",
                default_value: "Space",
            },
            ArgSchema {
                name: "Byte Length",
                description: "Number of bits per byte (default 8)",
                default_value: "8",
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
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("Space");
        let byte_len = args.get(1).and_then(|v| v.as_usize()).unwrap_or(8);
        let byte_len = if byte_len == 0 { 8 } else { byte_len };

        let clean = if delim_name == "None" {
            input_str.clone()
        } else {
            let delim = match delim_name {
                "Space" => " ",
                "Comma" => ",",
                "Semi-colon" => ";",
                "Colon" => ":",
                "Line feed" => "\n",
                "CRLF" => "\r\n",
                other => other,
            };
            // Remove delimiters to get a flat binary string
            input_str.replace(delim, "")
        };

        // Strip any whitespace that might remain
        let clean: String = clean.chars().filter(|c| *c == '0' || *c == '1').collect();

        if clean.is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        let mut i = 0;
        while i + byte_len <= clean.len() {
            let chunk = &clean[i..i + byte_len];
            let byte = u8::from_str_radix(chunk, 2).map_err(|e| {
                OperationError::InvalidInput(format!("Invalid binary chunk '{}': {}", chunk, e))
            })?;
            result.push(byte);
            i += byte_len;
        }

        Ok(result)
    }
}
