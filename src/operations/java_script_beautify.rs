/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JavaScript Beautify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JavaScript Beautify operation
pub struct JavaScriptBeautify;

impl Operation for JavaScriptBeautify {
    fn name(&self) -> &'static str {
        "JavaScript Beautify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Parses and pretty prints valid JavaScript code. Note: This implementation uses heuristic-based formatting as a full JS parser is not available."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Indent string",
            description: "String to use for each level of indentation",
            default_value: "    ",
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
        let text = String::from_utf8_lossy(&input);
        let indent_str = args.first().and_then(|a| a.as_str()).unwrap_or("    ");

        let result = beautify_js(&text, indent_str);
        Ok(result.into_bytes())
    }
}

fn beautify_js(input: &str, indent: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    let mut depth: usize = 0;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        match ch {
            '{' => {
                out.push('{');
                out.push('\n');
                depth += 1;
                for _ in 0..depth {
                    out.push_str(indent);
                }
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                }
                if !out.ends_with('\n') {
                    out.push('\n');
                }
                // Trim trailing spaces from previous line's indent
                let trimmed = out.trim_end_matches(' ');
                let tlen = trimmed.len();
                out.truncate(tlen);
                if !out.ends_with('\n') {
                    out.push('\n');
                }

                for _ in 0..depth {
                    out.push_str(indent);
                }
                out.push('}');
                out.push('\n');
                for _ in 0..depth {
                    out.push_str(indent);
                }
            }
            ';' => {
                out.push(';');
                out.push('\n');
                for _ in 0..depth {
                    out.push_str(indent);
                }
            }
            '\n' | '\r' => {
                // Ignore existing newlines to reformat
            }
            _ => {
                out.push(ch);
            }
        }
        i += 1;
    }

    out.trim().to_string()
}
