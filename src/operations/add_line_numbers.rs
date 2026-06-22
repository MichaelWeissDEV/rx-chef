/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Add line numbers operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Add line numbers operation
///
/// Adds line numbers to the output.
pub struct AddLineNumbers;

impl Operation for AddLineNumbers {
    fn name(&self) -> &'static str {
        "Add line numbers"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Adds line numbers to the output."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Offset",
            description: "Starting line number offset",
            default_value: "0",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let lines: Vec<&str> = input_str.split('\n').collect();
        // Remove trailing empty line if input ends with newline
        let num_lines = lines.len();

        let width = num_lines.to_string().len();
        let offset = if !args.is_empty() {
            args[0]
                .as_str()
                .unwrap_or("0")
                .split('.')
                .next()
                .unwrap_or("0")
                .parse::<isize>()
                .unwrap_or(0)
        } else {
            0
        };

        let mut output = String::new();
        for (n, line) in lines.iter().enumerate() {
            if line.is_empty() && n == num_lines - 1 {
                continue; // Skip trailing empty line from split
            }
            let line_num = (n as isize + 1 + offset).max(1) as usize;
            let line_str = line_num.to_string();
            let padding = " ".repeat(width.saturating_sub(line_str.len()));
            output.push_str(&padding);
            output.push_str(&line_str);
            output.push(' ');
            output.push_str(line);
            output.push('\n');
        }

        // Remove trailing newline if input didn't have one
        let result = if input_str.ends_with('\n') {
            output
        } else {
            output.pop(); // Remove last \n
            output
        };

        Ok(result.into_bytes())
    }
}
