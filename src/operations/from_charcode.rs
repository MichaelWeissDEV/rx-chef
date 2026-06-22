/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Charcode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Charcode operation
pub struct FromCharcode;

impl Operation for FromCharcode {
    fn name(&self) -> &'static str {
        "From Charcode"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts unicode character codes back into text."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The character code delimiter",
                default_value: "Space",
            },
            ArgSchema {
                name: "Base",
                description: "The numerical base of the codes",
                default_value: "16",
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
        let base = args.get(1).and_then(|v| v.as_f64()).unwrap_or(16.0) as u32;

        if base < 2 || base > 36 {
            return Err(OperationError::InvalidArgument {
                name: "Base".to_string(),
                reason: "Base argument must be between 2 and 36".to_string(),
            });
        }

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

        let bites: Vec<String> = if delim.is_empty() {
            if input_str.len() > 17 {
                input_str
                    .as_bytes()
                    .chunks(2)
                    .map(|c| String::from_utf8_lossy(c).to_string())
                    .collect()
            } else {
                vec![input_str]
            }
        } else {
            input_str.split(delim).map(|s| s.to_string()).collect()
        };

        let mut result = Vec::new();
        for b in bites {
            if b.trim().is_empty() {
                continue;
            }
            if let Ok(code) = u32::from_str_radix(b.trim(), base) {
                if let Some(c) = std::char::from_u32(code) {
                    let mut buf = [0u8; 4];
                    result.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                }
            }
        }

        Ok(result)
    }
}
