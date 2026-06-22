/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Octal operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Octal operation - converts raw bytes to an octal string.
///
/// e.g. "Hello" -> "110 145 154 154 157"
pub struct ToOctal;

impl Operation for ToOctal {
    fn name(&self) -> &'static str {
        "To Octal"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts the input string to octal bytes separated by the specified delimiter. e.g. 'Hello' becomes '110 145 154 154 157'."
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
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
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

        let result: Vec<String> = input.iter().map(|&b| format!("{:o}", b)).collect();
        Ok(result.join(delim).into_bytes())
    }
}
