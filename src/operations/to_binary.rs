/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Binary operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Binary operation - converts raw bytes to a binary string.
pub struct ToBinary;

impl Operation for ToBinary {
    fn name(&self) -> &'static str {
        "To Binary"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Displays the input data as a binary string. e.g. 'Hi' becomes '01001000 01101001'"
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
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("Space");
        let byte_len = args.get(1).and_then(|v| v.as_usize()).unwrap_or(8);
        let byte_len = if byte_len == 0 { 8 } else { byte_len };

        let delim = match delim_name {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            "None" => "",
            other => other,
        };

        let binary_strings: Vec<String> = input
            .iter()
            .map(|&b| format!("{:0>width$b}", b, width = byte_len))
            .collect();

        Ok(binary_strings.join(delim).into_bytes())
    }
}
