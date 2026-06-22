/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Drop bytes operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Drop bytes operation
///
/// Cuts a slice of the specified number of bytes out of the data.
/// Negative values are allowed.
pub struct DropBytes;

impl Operation for DropBytes {
    fn name(&self) -> &'static str {
        "Drop bytes"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Cuts a slice of the specified number of bytes out of the data. Negative values are allowed."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Start",
                description: "Starting byte position (can be negative)",
                default_value: "0",
            },
            ArgSchema {
                name: "Length",
                description: "Number of bytes to drop (can be negative)",
                default_value: "5",
            },
            ArgSchema {
                name: "Apply to each line",
                description: "Apply drop to each line separately",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let start = args.first().and_then(|a| a.as_i64()).unwrap_or(0) as i64;
        let length = args.get(1).and_then(|a| a.as_i64()).unwrap_or(5) as i64;
        let apply_to_each_line = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        if !apply_to_each_line {
            return drop_bytes_single(&input, start, length);
        }

        // Split input into lines (split by 0x0a = \n)
        let lines: Vec<Vec<u8>> = input.split(|&b| b == 0x0a).map(|s| s.to_vec()).collect();

        let mut output = Vec::new();
        let mut s = start;
        let mut l = length;

        for line in &lines {
            let processed = drop_bytes_single(line, s, l)?;
            output.extend_from_slice(&processed);
            output.push(0x0a); // Add line feed back
            s = start;
            l = length;
        }

        // Remove trailing line feed if present
        if !output.is_empty() && output[output.len() - 1] == 0x0a {
            output.pop();
        }

        Ok(output)
    }
}

fn drop_bytes_single(input: &[u8], start: i64, length: i64) -> Result<Vec<u8>, OperationError> {
    let input_len = input.len() as i64;
    let mut start = start;
    let mut length = length;

    if start < 0 {
        // Take from the end
        start = input_len + start;
    }

    if length < 0 {
        // Flip start point
        start = start + length;
        if start < 0 {
            start = input_len + start;
            length = start - length;
        } else {
            length = -length;
        }
    }

    // Ensure bounds are valid
    if start < 0 {
        start = 0;
    }
    if start > input_len {
        start = input_len;
    }

    let left_len = start as usize;
    let right_start = (start + length) as usize;

    if right_start >= input.len() {
        // Drop everything from start to end
        Ok(input[..left_len].to_vec())
    } else {
        // Drop middle section
        let mut result = Vec::with_capacity(input.len());
        result.extend_from_slice(&input[..left_len]);
        result.extend_from_slice(&input[right_start..]);
        Ok(result)
    }
}
