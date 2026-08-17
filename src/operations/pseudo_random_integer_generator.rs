/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
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
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Min Value",
                description: "Minimum value (inclusive)",
                default_value: "0",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Max Value",
                description: "Maximum value (inclusive)",
                default_value: "99",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between integers",
                default_value: "Space",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output",
                description: "Output format (Raw, Hex, Decimal)",
                default_value: "Decimal",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["Raw", "Hex", "Decimal"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn input_requirement(&self) -> crate::operation::InputRequirement {
        crate::operation::InputRequirement::Ignored
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    /// Generates a fresh random integer on every run.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Random]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
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
