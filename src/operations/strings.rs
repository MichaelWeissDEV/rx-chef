/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Strings operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Strings operation
///
/// Extracts all printable strings from the input, similar to the Unix `strings` command.
pub struct Strings;

impl Operation for Strings {
    fn name(&self) -> &'static str {
        "Strings"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts all strings from the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Encoding",
                description: "Single byte, 16-bit littleendian, 16-bit bigendian, All",
                default_value: "Single byte",
            },
            ArgSchema {
                name: "Minimum length",
                description: "Minimum string length",
                default_value: "4",
            },
            ArgSchema {
                name: "Match",
                description: "Alphanumeric + punctuation (A), All printable chars (A), Null-terminated strings (A), Alphanumeric + punctuation (U), All printable chars (U), Null-terminated strings (U)",
                default_value: "All printable chars (A)",
            },
            ArgSchema {
                name: "Display total",
                description: "Display total count of found strings",
                default_value: "false",
            },
            ArgSchema {
                name: "Sort",
                description: "Sort results case-insensitively",
                default_value: "false",
            },
            ArgSchema {
                name: "Unique",
                description: "Remove duplicate results",
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
        let encoding = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Single byte");
        let min_len = args.get(1).and_then(|a| a.as_usize()).unwrap_or(4);
        let match_type = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("All printable chars (A)");
        let display_total = args.get(3).and_then(|a| a.as_bool()).unwrap_or(false);
        let do_sort = args.get(4).and_then(|a| a.as_bool()).unwrap_or(false);
        let do_unique = args.get(5).and_then(|a| a.as_bool()).unwrap_or(false);

        // Build character class pattern
        let char_class = match match_type {
            "Alphanumeric + punctuation (A)" => {
                "[A-Za-z0-9/\\\\-:.,_$%'\\\"()<>= !\\[\\]{}@]".to_string()
            }
            "All printable chars (A)" | "Null-terminated strings (A)" => r"[\x20-\x7e]".to_string(),
            "Alphanumeric + punctuation (U)" => r"[\p{L}\p{N}\p{P}\p{Z}]".to_string(),
            "All printable chars (U)" | "Null-terminated strings (U)" => {
                r"[\p{L}\p{M}\p{Z}\p{S}\p{N}\p{P}]".to_string()
            }
            // Default to ASCII printable
            _ => r"[\x20-\x7e]".to_string(),
        };

        // For UTF-16 we search the raw bytes via a lossy string; true UTF-16 support
        // would require a separate pass. Here we handle Single byte only for core
        // use; encoding variants use null-byte awareness.
        let null_terminated = match_type.contains("Null-terminated");

        let pattern = match encoding {
            "16-bit littleendian" => format!(r"(?:{}[\x00]){{{},}}", char_class, min_len),
            "16-bit bigendian"    => format!(r"(?:[\x00]{}){{{},}}", char_class, min_len),
            "All"                 => format!(r"(?:[\x00]?{}[\x00]?){{{},}}", char_class, min_len),
            _ /* Single byte */   => {
                if null_terminated {
                    format!(r"{}{{{},}}\x00", char_class, min_len)
                } else {
                    format!(r"{}{{{},}}", char_class, min_len)
                }
            }
        };

        let re = Regex::new(&pattern)
            .map_err(|e| OperationError::ProcessingError(format!("Regex error: {}", e)))?;

        // Work on lossy UTF-8 representation of the raw bytes
        let text = String::from_utf8_lossy(&input);

        let mut results: Vec<String> = re
            .find_iter(&text)
            .map(|m| {
                // Strip trailing null if null-terminated
                let s = m.as_str();
                if null_terminated && s.ends_with('\0') {
                    s.trim_end_matches('\0').to_string()
                } else {
                    s.to_string()
                }
            })
            .collect();

        if do_unique {
            let mut seen: Vec<String> = Vec::new();
            for item in results {
                if !seen.contains(&item) {
                    seen.push(item);
                }
            }
            results = seen;
        }

        if do_sort {
            results.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        }

        let body = results.join("\n");
        let output = if display_total {
            format!("Total found: {}\n\n{}", results.len(), body)
        } else {
            body
        };

        Ok(output.into_bytes())
    }
}
