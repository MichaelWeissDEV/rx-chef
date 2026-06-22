/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Caret/M-decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Caret/M-decode operation
pub struct CaretMdecode;

impl Operation for CaretMdecode {
    fn name(&self) -> &'static str {
        "Caret/M-decode"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Decodes caret or M-encoded strings, i.e. ^M turns into a newline, M-^] turns into 0x9d. Sources such as `cat -v`.\n\nPlease be aware that when using `cat -v` ^_ (caret-underscore) will not be encoded, but represents a valid encoding (namely that of 0x1f)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let mut bytes = Vec::new();
        let mut prev = "";

        for c in input_str.chars() {
            let char_code = c as u32;

            match prev {
                "M-^" => {
                    if (64..=95).contains(&char_code) {
                        bytes.push((char_code + 64) as u8);
                    } else if char_code == 63 {
                        bytes.push(255);
                    } else {
                        bytes.extend_from_slice(&[77, 45, 94, char_code as u8]);
                    }
                    prev = "";
                }
                "M-" => {
                    if c == '^' {
                        prev = "M-^";
                    } else if (32..=126).contains(&char_code) {
                        bytes.push((char_code + 128) as u8);
                        prev = "";
                    } else {
                        bytes.extend_from_slice(&[77, 45, char_code as u8]);
                        prev = "";
                    }
                }
                "M" => {
                    if c == '-' {
                        prev = "M-";
                    } else {
                        bytes.extend_from_slice(&[77, char_code as u8]);
                        prev = "";
                    }
                }
                "^" => {
                    if (64..=126).contains(&char_code) {
                        bytes.push((char_code - 64) as u8);
                    } else if char_code == 63 {
                        bytes.push(127);
                    } else {
                        bytes.extend_from_slice(&[94, char_code as u8]);
                    }
                    prev = "";
                }
                _ => {
                    if c == 'M' {
                        prev = "M";
                    } else if c == '^' {
                        prev = "^";
                    } else {
                        bytes.push(char_code as u8);
                    }
                }
            }
        }

        // CyberChef implementation does not seem to handle trailing 'M' or '^'
        // But for completeness we should probably add them if we wanted to be robust.
        // Given the instructions, we stick to the JS logic.

        Ok(bytes)
    }
}
