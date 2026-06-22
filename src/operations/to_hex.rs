/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Hex operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Hex operation
///
/// Converts the input string to hexadecimal bytes separated by the specified delimiter.
/// e.g. `Hello` becomes `48 65 6c 6c 6f`
pub struct ToHex;

impl Operation for ToHex {
    fn name(&self) -> &'static str {
        "To Hex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts the input string to hexadecimal bytes separated by the specified delimiter."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between hex bytes (Space, Comma, Semi-colon, Colon, Line feed, CRLF, None, 0x with comma, \\x)",
                default_value: "Space",
            },
            ArgSchema {
                name: "Bytes per line",
                description: "Number of bytes per output line (0 = no wrapping)",
                default_value: "0",
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
        let line_size = args.get(1).and_then(|v| v.as_usize()).unwrap_or(0);

        // Special delimiter: "0x with comma" prepends "0x" before each byte, separated by ","
        let (prefix, sep, trailing_sep) = if delim_name == "0x with comma" {
            ("0x", ",", false)
        } else {
            let sep = match delim_name {
                "Space" => " ",
                "Comma" => ",",
                "Semi-colon" => ";",
                "Colon" => ":",
                "Line feed" => "\n",
                "CRLF" => "\r\n",
                "None" => "",
                r"\x" => "\\x",
                other => other,
            };
            // For \x the prefix is also \x and sep is empty between bytes
            if delim_name == r"\x" {
                ("\\x", "", false)
            } else {
                ("", sep, false)
            }
        };

        let _ = trailing_sep; // suppress unused warning

        let mut output = String::new();
        let total = input.len();

        for (i, &b) in input.iter().enumerate() {
            // Write prefix
            output.push_str(prefix);
            // Write hex byte
            output.push_str(&format!("{:02x}", b));

            let is_last = i + 1 == total;
            let line_break = line_size > 0 && (i + 1) % line_size == 0 && !is_last;

            if line_break {
                // "0x with comma" keeps the comma before the newline (comma is a byte separator)
                if delim_name == "0x with comma" {
                    output.push(',');
                }
                output.push('\n');
            } else if !is_last && !sep.is_empty() {
                output.push_str(sep);
            }
        }

        Ok(output.into_bytes())
    }
}
