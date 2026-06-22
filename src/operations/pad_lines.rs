/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Pad lines operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Pad Lines operation
///
/// Adds the specified number of repetitions of a character to the start or
/// end of each line.  This matches JavaScript `padStart(line.length + len)`
/// and `padEnd(line.length + len)` which prepend/append `len` extra chars.
pub struct PadLines;

impl Operation for PadLines {
    fn name(&self) -> &'static str {
        "Pad lines"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Add the specified number of the specified character to the beginning or end of each line."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Position",
                description: "Where to add padding: Start or End",
                default_value: "Start",
            },
            ArgSchema {
                name: "Length",
                description: "Number of padding characters to add",
                default_value: "5",
            },
            ArgSchema {
                name: "Character",
                description: "Character to use for padding",
                default_value: " ",
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
        let position = args.first().and_then(|a| a.as_str()).unwrap_or("Start");

        let len = args.get(1).and_then(|a| a.as_usize()).unwrap_or(5);

        let chr_str = args.get(2).and_then(|a| a.as_str()).unwrap_or(" ");

        if chr_str.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Character".to_string(),
                reason: "Padding character must not be empty".to_string(),
            });
        }

        let pad_char = chr_str.chars().next().unwrap_or(' ');
        let padding: String = std::iter::repeat(pad_char).take(len).collect();

        let text = String::from_utf8_lossy(&input);
        let lines: Vec<&str> = text.split('\n').collect();
        let mut result_lines: Vec<String> = Vec::with_capacity(lines.len());

        for line in &lines {
            let padded = match position {
                "End" => format!("{}{}", line, padding),
                _ => format!("{}{}", padding, line), // "Start" is default
            };
            result_lines.push(padded);
        }

        Ok(result_lines.join("\n").into_bytes())
    }
}
