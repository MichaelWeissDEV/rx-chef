/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From BCD operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From BCD operation
pub struct FromBCD;

impl Operation for FromBCD {
    fn name(&self) -> &'static str {
        "From BCD"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Binary-Coded Decimal (BCD) is a class of binary encodings of decimal numbers where each decimal digit is represented by a fixed number of bits, usually four or eight. Special bit patterns are sometimes used for a sign."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Scheme",
                description: "The BCD encoding scheme",
                default_value: "8 4 2 1",
            },
            ArgSchema {
                name: "Packed",
                description: "Whether the BCD is packed (two digits per byte)",
                default_value: "true",
            },
            ArgSchema {
                name: "Signed",
                description: "Whether the BCD is signed (trailing sign nibble)",
                default_value: "false",
            },
            ArgSchema {
                name: "Input format",
                description: "The format of the input data",
                default_value: "Nibbles",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let scheme = args.first().and_then(|v| v.as_str()).unwrap_or("8 4 2 1");
        let packed = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let signed = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let input_format = args.get(3).and_then(|v| v.as_str()).unwrap_or("Nibbles");

        let encoding = match scheme {
            "8 4 2 1" => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            "7 4 2 1" => &[0, 1, 2, 3, 4, 5, 6, 8, 9, 10],
            "4 2 2 1" => &[0, 1, 4, 5, 8, 9, 12, 13, 14, 15],
            "2 4 2 1" => &[0, 1, 2, 3, 4, 11, 12, 13, 14, 15],
            "8 4 -2 -1" => &[0, 7, 6, 5, 4, 11, 10, 9, 8, 15],
            "Excess-3" => &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            "IBM 8 4 2 1" => &[10, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Scheme".to_string(),
                    reason: format!("Unknown scheme: {}", scheme),
                })
            }
        };

        let mut nibbles = Vec::new();
        match input_format {
            "Nibbles" | "Bytes" => {
                let clean_input = input_str.replace(|c: char| c.is_whitespace(), "");
                for i in (0..clean_input.len()).step_by(4) {
                    if i + 4 <= clean_input.len() {
                        let n = u8::from_str_radix(&clean_input[i..i + 4], 2).map_err(|_| {
                            OperationError::InvalidInput("Invalid bit string".to_string())
                        })?;
                        nibbles.push(n);
                    }
                }
            }
            "Raw" => {
                for b in input_str.as_bytes() {
                    nibbles.push(b >> 4);
                    nibbles.push(b & 15);
                }
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Input format".to_string(),
                    reason: format!("Unknown input format: {}", input_format),
                })
            }
        }

        if !packed {
            // Discard every other nibble (the high one)
            let mut new_nibbles = Vec::new();
            for (i, &n) in nibbles.iter().enumerate() {
                if i % 2 != 0 {
                    new_nibbles.push(n);
                }
            }
            nibbles = new_nibbles;
        }

        let mut output = String::new();
        if signed && !nibbles.is_empty() {
            let sign = nibbles.pop().unwrap();
            if sign == 13 || sign == 11 {
                output.push('-');
            }
        }

        for n in nibbles {
            if let Some(pos) = encoding.iter().position(|&x| x == n) {
                output.push_str(&pos.to_string());
            } else {
                return Err(OperationError::InvalidInput(format!(
                    "Value {:04b} is not in the encoding scheme",
                    n
                )));
            }
        }

        Ok(output.into_bytes())
    }
}
