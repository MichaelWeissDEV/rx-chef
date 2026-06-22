/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Take bytes operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Take bytes operation
///
/// Takes a slice of the specified number of bytes from the data.
/// Negative values are allowed.
pub struct TakeBytes;

impl Operation for TakeBytes {
    fn name(&self) -> &'static str {
        "Take bytes"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Takes a slice of the specified number of bytes from the data. Negative values are allowed."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Start",
                description: "Starting byte position (negative counts from end)",
                default_value: "0",
            },
            ArgSchema {
                name: "Length",
                description: "Number of bytes to take (negative reverses direction)",
                default_value: "5",
            },
            ArgSchema {
                name: "Apply to each line",
                description: "If true, apply operation to each line separately",
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
        let start = args.first().and_then(|a| a.as_i64()).unwrap_or(0);
        let length = args.get(1).and_then(|a| a.as_i64()).unwrap_or(5);
        let apply_to_each_line = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        if !apply_to_each_line {
            let result = take_slice(&input, start, length);
            Ok(result)
        } else {
            // Split on 0x0a, process each line, rejoin with 0x0a
            let lines = split_lines(&input);
            let mut output: Vec<u8> = Vec::new();
            for (idx, line) in lines.iter().enumerate() {
                let sliced = take_slice(line, start, length);
                output.extend_from_slice(&sliced);
                if idx < lines.len() - 1 {
                    output.push(0x0a);
                }
            }
            Ok(output)
        }
    }
}

/// Split bytes on 0x0a newlines, returning a Vec of lines (without the newline chars).
fn split_lines(data: &[u8]) -> Vec<Vec<u8>> {
    let mut lines: Vec<Vec<u8>> = Vec::new();
    let mut current: Vec<u8> = Vec::new();
    for &byte in data {
        if byte == 0x0a {
            lines.push(current.clone());
            current.clear();
        } else {
            current.push(byte);
        }
    }
    lines.push(current);
    lines
}

/// Take a slice from data starting at `start` with `length` bytes.
/// Mirrors the JS behavior with negative start/length.
fn take_slice(data: &[u8], mut start: i64, mut length: i64) -> Vec<u8> {
    let data_len = data.len() as i64;

    if start < 0 {
        start = data_len + start;
    }

    if length < 0 {
        start += length;
        if start < 0 {
            start = data_len + start;
            length = start - length;
        } else {
            length = -length;
        }
    }

    // Clamp to valid range
    let s = start.max(0).min(data_len) as usize;
    let end = (start + length).max(0).min(data_len) as usize;

    if s >= end {
        return Vec::new();
    }

    data[s..end].to_vec()
}
