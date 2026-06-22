/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Take nth bytes operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Take nth bytes operation
///
/// Takes every nth byte starting with a given byte.
pub struct TakeNthBytes;

impl Operation for TakeNthBytes {
    fn name(&self) -> &'static str {
        "Take nth bytes"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Takes every nth byte starting with a given byte."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Take every",
                description: "Take every nth byte",
                default_value: "4",
            },
            ArgSchema {
                name: "Starting at",
                description: "Starting byte offset",
                default_value: "0",
            },
            ArgSchema {
                name: "Apply to each line",
                description: "If true, apply to each line separately",
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
        let n = args.first().and_then(|a| a.as_f64()).unwrap_or(4.0);
        let start = args.get(1).and_then(|a| a.as_f64()).unwrap_or(0.0);
        let each_line = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        // Validate n is a positive integer
        if n <= 0.0 || n != n.floor() {
            return Err(OperationError::InvalidArgument {
                name: "Take every".to_string(),
                reason: "'Take every' must be a positive integer.".to_string(),
            });
        }

        // Validate start is a non-negative integer
        if start < 0.0 || start != start.floor() {
            return Err(OperationError::InvalidArgument {
                name: "Starting at".to_string(),
                reason: "'Starting at' must be a positive or zero integer.".to_string(),
            });
        }

        let n = n as usize;
        let start = start as usize;

        let mut output = Vec::new();
        let mut offset: usize = 0;

        for (i, &byte) in input.iter().enumerate() {
            if each_line && byte == 0x0a {
                output.push(0x0a);
                offset = i + 1;
            } else if i >= offset + start && (i - (start + offset)) % n == 0 {
                output.push(byte);
            }
        }

        Ok(output)
    }
}
