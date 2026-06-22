/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Float operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Float operation - converts IEEE 754 float strings to raw bytes.
pub struct FromFloat;

impl Operation for FromFloat {
    fn name(&self) -> &'static str {
        "From Float"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Convert from IEEE754 Floating Point Numbers to raw bytes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Endianness",
                description: "Big Endian or Little Endian",
                default_value: "Big Endian",
            },
            ArgSchema {
                name: "Size",
                description: "Float (4 bytes) or Double (8 bytes)",
                default_value: "Float (4 bytes)",
            },
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter separating float values (Space, Comma, etc.)",
                default_value: "Space",
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

        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let endianness = args
            .first()
            .and_then(|v| v.as_str())
            .unwrap_or("Big Endian");
        let size = args
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("Float (4 bytes)");
        let delim_name = args.get(2).and_then(|v| v.as_str()).unwrap_or("Space");

        let is_double = size == "Double (8 bytes)";
        let is_le = endianness == "Little Endian";

        let delim = match delim_name {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            other => other,
        };

        let mut output = Vec::new();
        for token in input_str.split(delim) {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let f: f64 = token.parse::<f64>().map_err(|e| {
                OperationError::InvalidInput(format!("Cannot parse '{}' as float: {}", token, e))
            })?;

            if is_double {
                let bytes = if is_le {
                    f.to_le_bytes().to_vec()
                } else {
                    f.to_be_bytes().to_vec()
                };
                output.extend(bytes);
            } else {
                let f32_val = f as f32;
                let bytes = if is_le {
                    f32_val.to_le_bytes().to_vec()
                } else {
                    f32_val.to_be_bytes().to_vec()
                };
                output.extend(bytes);
            }
        }

        Ok(output)
    }
}
