/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Pseudo-Random Integer Generator operation.
 * -----------------------------------------------------------------------------
 */

use rand::Rng;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Pseudo-Random Integer Generator operation
pub struct PseudoRandomIntegerGenerator;

impl Operation for PseudoRandomIntegerGenerator {
    fn name(&self) -> &'static str {
        "Pseudo-Random Integer Generator"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A cryptographically-secure pseudo-random number generator (PRNG). Generates random integers within a specified range. The supported range of integers is from -(2^53 - 1) to (2^53 - 1)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Number of Integers",
                description: "How many integers to generate",
                default_value: "1",
            },
            ArgSchema {
                name: "Min Value",
                description: "Minimum value (inclusive)",
                default_value: "0",
            },
            ArgSchema {
                name: "Max Value",
                description: "Maximum value (inclusive)",
                default_value: "99",
            },
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between integers",
                default_value: "Space",
            },
            ArgSchema {
                name: "Output",
                description: "Output format (Raw, Hex, Decimal)",
                default_value: "Decimal",
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

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let num_ints = args.first().and_then(|a| a.as_usize()).unwrap_or(1);
        let min_val = args.get(1).and_then(|a| a.as_i64()).unwrap_or(0);
        let max_val = args.get(2).and_then(|a| a.as_i64()).unwrap_or(99);
        let delimiter = args.get(3).and_then(|a| a.as_str()).unwrap_or("Space");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Decimal");

        if min_val > max_val {
            return Err(OperationError::InvalidArgument {
                name: "Min Value".to_string(),
                reason: "Min cannot be larger than Max.".to_string(),
            });
        }

        let delim_str = match delimiter {
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            "Tab" => "\t",
            "None" => "",
            _ => " ",
        };

        let mut rng = rand::thread_rng();
        let mut output = Vec::new();

        for _ in 0..num_ints {
            let val: i64 = rng.gen_range(min_val..=max_val);
            let formatted = match output_type {
                "Hex" => format!("{:x}", val),
                "Decimal" => format!("{}", val),
                "Raw" => {
                    if let Some(c) = std::char::from_u32(val as u32) {
                        c.to_string()
                    } else {
                        if (0..=255).contains(&val) {
                            (val as u8 as char).to_string()
                        } else {
                            format!("{}", val)
                        }
                    }
                }
                _ => format!("{}", val),
            };
            output.push(formatted);
        }

        let result = if output_type == "Raw" {
            output.join("")
        } else {
            output.join(delim_str)
        };

        Ok(result.into_bytes())
    }
}
