/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Hexdump operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Hexdump operation
///
/// Creates a hexdump of the input data, displaying both the hexadecimal values
/// of each byte and an ASCII representation alongside.
pub struct ToHexdump;

impl Operation for ToHexdump {
    fn name(&self) -> &'static str {
        "To Hexdump"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Creates a hexdump of the input data, displaying both the hexadecimal values of each byte and an ASCII representation alongside."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Width",
                description: "Number of bytes per row (must be >= 1)",
                default_value: "16",
            },
            ArgSchema {
                name: "Upper case hex",
                description: "Display hex bytes in upper case",
                default_value: "false",
            },
            ArgSchema {
                name: "Include final length",
                description: "Append the total byte count as a final line",
                default_value: "false",
            },
            ArgSchema {
                name: "UNIX format",
                description: "Use UNIX printable character subset for ASCII column",
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
        let width = args.first().and_then(|v| v.as_usize()).unwrap_or(16);
        let upper_case = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
        let include_final_length = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let unix_format = args.get(3).and_then(|v| v.as_bool()).unwrap_or(false);

        if width < 1 {
            return Err(OperationError::InvalidArgument {
                name: "Width".to_string(),
                reason: "Width must be a positive integer".to_string(),
            });
        }

        let mut lines: Vec<String> = Vec::new();

        let chunks: Vec<&[u8]> = input.chunks(width).collect();
        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let offset = chunk_idx * width;
            let mut line_no = format!("{:08x}", offset);

            // Build hex column: each byte as 2 hex digits, padded to width*(2+1) chars
            let hex_parts: Vec<String> = chunk.iter().map(|&b| format!("{:02x}", b)).collect();
            let mut hex_str = hex_parts.join(" ");
            // Pad to full line width: each byte takes 3 chars (2 hex + 1 space) except last
            let full_width = width * 3 - 1;
            while hex_str.len() < full_width {
                hex_str.push(' ');
            }

            // Build ASCII column: printable chars pass through, others become '.'
            let ascii_str: String = chunk
                .iter()
                .map(|&b| printable_char(b, unix_format))
                .collect();

            if upper_case {
                hex_str = hex_str.to_uppercase();
                line_no = line_no.to_uppercase();
            }

            lines.push(format!("{}  {} |{}|", line_no, hex_str, ascii_str));
        }

        if include_final_length && !input.is_empty() {
            let final_offset = format!("{:08x}", input.len());
            let formatted = if upper_case {
                final_offset.to_uppercase()
            } else {
                final_offset
            };
            lines.push(formatted);
        }

        Ok(lines.join("\n").into_bytes())
    }
}

/// Returns the printable ASCII character for hex dumps.
///
/// Matches JS `Utils.printable`: for standard mode, printable is 0x20-0x7E.
/// For UNIX format, only 0x21-0x7E (excludes space).
fn printable_char(b: u8, unix_format: bool) -> char {
    if unix_format {
        if b > 0x20 && b <= 0x7E {
            b as char
        } else {
            '.'
        }
    } else if (0x20..=0x7E).contains(&b) {
        b as char
    } else {
        '.'
    }
}
