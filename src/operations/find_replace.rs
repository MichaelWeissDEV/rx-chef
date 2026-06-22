/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Find / Replace operation.
 * -----------------------------------------------------------------------------
 */

use regex::RegexBuilder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Find / Replace operation
pub struct FindReplace;

impl Operation for FindReplace {
    fn name(&self) -> &'static str {
        "Find / Replace"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Replaces all occurrences of the first string with the second. Supports regex, simple string, and extended string modes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Find",
                description: "The string or regex to find",
                default_value: "",
            },
            ArgSchema {
                name: "Find type",
                description: "Regex, Extended (\\n, \\t, \\x...), or Simple string",
                default_value: "Simple string",
            },
            ArgSchema {
                name: "Replace",
                description: "The replacement string",
                default_value: "",
            },
            ArgSchema {
                name: "Global match",
                description: "Replace all occurrences",
                default_value: "true",
            },
            ArgSchema {
                name: "Case insensitive",
                description: "Ignore case when matching",
                default_value: "false",
            },
            ArgSchema {
                name: "Multiline matching",
                description: "^ and $ match start/end of lines",
                default_value: "true",
            },
            ArgSchema {
                name: "Dot matches all",
                description: "Dot also matches newline",
                default_value: "false",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let find = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let find_type = args
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("Simple string");
        let replace = args.get(2).and_then(|v| v.as_str()).unwrap_or("");
        let global = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);
        let case_insensitive = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);
        let multiline = args.get(5).and_then(|v| v.as_bool()).unwrap_or(true);
        let dot_all = args.get(6).and_then(|v| v.as_bool()).unwrap_or(false);

        if find.is_empty() {
            return Ok(input_str.into_bytes());
        }

        let pattern = match find_type {
            "Regex" => find.to_string(),
            "Extended (\\n, \\t, \\x...)" => {
                let resolved = parse_escaped_chars(find);
                regex::escape(&resolved)
            }
            _ => regex::escape(find),
        };

        let re = RegexBuilder::new(&pattern)
            .case_insensitive(case_insensitive)
            .multi_line(multiline)
            .dot_matches_new_line(dot_all)
            .build()
            .map_err(|e| OperationError::InvalidArgument {
                name: "Find".to_string(),
                reason: format!("Invalid pattern: {}", e),
            })?;

        let result = if global {
            re.replace_all(&input_str, replace).into_owned()
        } else {
            re.replace(&input_str, replace).into_owned()
        };

        Ok(result.into_bytes())
    }
}

/// Parse CyberChef-style extended escape sequences.
fn parse_escaped_chars(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('b') => result.push('\x08'),
                Some('f') => result.push('\x0C'),
                Some('x') => {
                    let h1 = chars.next().unwrap_or('0');
                    let h2 = chars.next().unwrap_or('0');
                    let hex_str = format!("{}{}", h1, h2);
                    if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                        result.push(byte as char);
                    } else {
                        result.push('\\');
                        result.push('x');
                        result.push(h1);
                        result.push(h2);
                    }
                }
                Some('\\') => result.push('\\'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}
