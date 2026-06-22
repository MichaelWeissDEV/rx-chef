/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Hex Content operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Hex Content operation - encodes special characters to hex in SNORT pipe notation.
///
/// e.g. `foo=bar` becomes `foo|3d|bar`
pub struct ToHexContent;

impl Operation for ToHexContent {
    fn name(&self) -> &'static str {
        "To Hex Content"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts special characters in a string to hexadecimal using SNORT pipe notation. e.g. 'foo=bar' becomes 'foo|3d|bar'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Convert",
                description:
                    "Only special chars, Only special chars including spaces, or All chars",
                default_value: "Only special chars",
            },
            ArgSchema {
                name: "Print spaces between bytes",
                description: "Add spaces between hex bytes inside pipes",
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
        let convert = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("Only special chars");
        let spaces = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

        if convert == "All chars" {
            let hex_strs: Vec<String> = input.iter().map(|b| format!("{:02x}", b)).collect();
            let joined = if spaces {
                hex_strs.join(" ")
            } else {
                hex_strs.join("")
            };
            return Ok(format!("|{}|", joined).into_bytes());
        }

        let convert_spaces = convert == "Only special chars including spaces";
        let mut output = String::new();
        let mut in_hex = false;

        for &b in &input {
            let is_special = (b == 32 && convert_spaces)
                || (b < 48 && b != 32)
                || (b > 57 && b < 65)
                || (b > 90 && b < 97)
                || b > 122;

            if is_special {
                if !in_hex {
                    output.push('|');
                    in_hex = true;
                } else if spaces {
                    output.push(' ');
                }
                output.push_str(&format!("{:02x}", b));
            } else {
                if in_hex {
                    output.push('|');
                    in_hex = false;
                }
                output.push(b as char);
            }
        }

        if in_hex {
            output.push('|');
        }

        Ok(output.into_bytes())
    }
}
