/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Reverse operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Reverses the input string by byte, character, or line.
pub struct Reverse;

impl Operation for Reverse {
    fn name(&self) -> &'static str {
        "Reverse"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Reverses the input string. The 'By' argument controls whether the reversal \
         operates on individual bytes, Unicode characters, or whole lines."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "By",
            description: "Byte, Character, or Line",
            default_value: "Character",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let by = args.first().and_then(|a| a.as_str()).unwrap_or("Character");

        match by {
            "Byte" => {
                let mut out = input;
                out.reverse();
                Ok(out)
            }
            "Character" => {
                // Decode as UTF-8 (lossy), reverse the Unicode scalar sequence, re-encode.
                let s = String::from_utf8_lossy(&input);
                let reversed: String = s.chars().rev().collect();
                Ok(reversed.into_bytes())
            }
            "Line" => {
                // Split on 0x0a, reverse the list of lines, rejoin with 0x0a.
                // The JS implementation preserves the original byte length exactly.
                let input_len = input.len();
                let mut lines: Vec<Vec<u8>> = Vec::new();
                let mut current: Vec<u8> = Vec::new();
                for &byte in &input {
                    if byte == b'\n' {
                        lines.push(current);
                        current = Vec::new();
                    } else {
                        current.push(byte);
                    }
                }
                lines.push(current);
                lines.reverse();

                let mut result: Vec<u8> = Vec::with_capacity(input_len);
                for line in lines {
                    result.extend_from_slice(&line);
                    result.push(b'\n');
                }
                // Trim to original length (the JS does result.slice(0, input.length)).
                result.truncate(input_len);
                Ok(result)
            }
            other => Err(OperationError::InvalidArgument {
                name: "By".to_string(),
                reason: format!("unrecognised mode: {}", other),
            }),
        }
    }
}
