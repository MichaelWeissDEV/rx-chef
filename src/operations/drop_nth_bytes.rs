/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Drop nth bytes operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Drop nth bytes operation
///
/// Drops every nth byte starting with a given byte.
pub struct DropNthBytes;

impl Operation for DropNthBytes {
    fn name(&self) -> &'static str {
        "Drop nth bytes"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Drops every nth byte starting with a given byte."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Drop every",
                description: "Drop every nth byte",
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
        let n = if args.len() > 0 {
            args[0].as_f64().unwrap_or(4.0)
        } else {
            4.0
        };

        let start = if args.len() > 1 {
            args[1].as_f64().unwrap_or(0.0)
        } else {
            0.0
        };

        let apply_to_each_line = if args.len() > 2 {
            args[2].as_bool().unwrap_or(false)
        } else {
            false
        };

        // Validate n is a positive integer
        if n <= 0.0 || n != n.floor() {
            return Err(OperationError::InvalidArgument {
                name: "Drop every".to_string(),
                reason: "'Drop every' must be a positive integer.".to_string(),
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
        let mut offset = 0;

        for (i, &byte) in input.iter().enumerate() {
            if apply_to_each_line && byte == 0x0a {
                output.push(byte);
                offset = i + 1;
            } else if i - offset < start || (i - (start + offset)) % n != 0 {
                output.push(byte);
            }
        }

        Ok(output)
    }
}
