/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Float operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Float operation - converts raw bytes to IEEE 754 floating-point representation.
pub struct ToFloat;

impl Operation for ToFloat {
    fn name(&self) -> &'static str {
        "To Float"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Convert to IEEE754 Floating Point Numbers."
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
                description: "Delimiter to join float values (Space, Comma, etc.)",
                default_value: "Space",
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
        let byte_size: usize = if is_double { 8 } else { 4 };

        let delim = match delim_name {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            other => other,
        };

        if input.len() % byte_size != 0 {
            return Err(OperationError::InvalidInput(format!(
                "Input length {} is not a multiple of {}",
                input.len(),
                byte_size
            )));
        }

        let mut output = Vec::new();
        for chunk in input.chunks(byte_size) {
            let f_str = if is_double {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(chunk);
                let f = if is_le {
                    f64::from_le_bytes(arr)
                } else {
                    f64::from_be_bytes(arr)
                };
                format!("{}", f)
            } else {
                let mut arr = [0u8; 4];
                arr.copy_from_slice(chunk);
                let f = if is_le {
                    f32::from_le_bytes(arr)
                } else {
                    f32::from_be_bytes(arr)
                };
                format!("{}", f)
            };
            output.push(f_str);
        }

        Ok(output.join(delim).into_bytes())
    }
}
