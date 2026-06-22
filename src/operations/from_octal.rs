/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Octal operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Octal operation - converts octal byte string to raw bytes.
///
/// e.g. "110 145 154 154 157" -> "Hello"
pub struct FromOctal;

impl Operation for FromOctal {
    fn name(&self) -> &'static str {
        "From Octal"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts an octal byte string back into its raw value. e.g. '110 145 154 154 157' becomes 'Hello'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description:
                "Delimiter between octal values (Space, Comma, Semi-colon, Colon, Line feed, CRLF)",
            default_value: "Space",
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
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("Space");

        let delim = match delim_name {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            other => other,
        };

        let mut result = Vec::new();
        for token in input_str.split(delim) {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let byte = u8::from_str_radix(token, 8).map_err(|e| {
                OperationError::InvalidInput(format!("Cannot parse '{}' as octal: {}", token, e))
            })?;
            result.push(byte);
        }

        Ok(result)
    }
}
