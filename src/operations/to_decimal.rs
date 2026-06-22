/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Decimal operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Decimal operation
///
/// Converts the input data to an ordinal integer array.
/// e.g. `Hello` becomes `72 101 108 108 111`
pub struct ToDecimal;

impl Operation for ToDecimal {
    fn name(&self) -> &'static str {
        "To Decimal"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts the input data to an ordinal integer array. e.g. 'Hello' becomes '72 101 108 108 111'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The delimiter between decimal values",
                default_value: "Space",
            },
            ArgSchema {
                name: "Support signed values",
                description: "Whether to treat bytes as signed (-128 to 127)",
                default_value: "false",
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
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("Space");
        let signed = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

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

        let result: Vec<String> = if signed {
            // Cast each byte as i8 for signed representation
            input.iter().map(|&b| (b as i8).to_string()).collect()
        } else {
            input.iter().map(|&b| b.to_string()).collect()
        };

        Ok(result.join(delim).into_bytes())
    }
}
