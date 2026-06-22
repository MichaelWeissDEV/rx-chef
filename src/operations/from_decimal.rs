/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Decimal operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Decimal operation
pub struct FromDecimal;

impl Operation for FromDecimal {
    fn name(&self) -> &'static str {
        "From Decimal"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts the data from an ordinal integer array back into its raw form."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The character code delimiter",
                default_value: "Space",
            },
            ArgSchema {
                name: "Support signed values",
                description: "Support negative values (signed 8-bit integers)",
                default_value: "false",
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
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let delim_arg = args.first().and_then(|v| v.as_str()).unwrap_or("Space");
        let signed = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

        let delim = match delim_arg {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            "None" => "",
            _ => delim_arg,
        };

        let bites: Vec<&str> = if delim.is_empty() {
            vec![&input_str]
        } else {
            input_str.split(delim).collect()
        };

        let mut result = Vec::new();
        for b in bites {
            let b = b.trim();
            if b.is_empty() {
                continue;
            }

            if signed {
                if let Ok(val) = b.parse::<i16>() {
                    let byte = if val < 0 {
                        (256 + val) as u8
                    } else {
                        val as u8
                    };
                    result.push(byte);
                }
            } else {
                if let Ok(val) = b.parse::<u8>() {
                    result.push(val);
                }
            }
        }

        Ok(result)
    }
}
