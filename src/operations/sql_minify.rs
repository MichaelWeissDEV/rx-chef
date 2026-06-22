/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SQL Minify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SQL Minify operation
pub struct SQLMinify;

impl Operation for SQLMinify {
    fn name(&self) -> &'static str {
        "SQL Minify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Compresses Structured Query Language (SQL) code."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let minified = minify_sql(&input_str);
        Ok(minified.into_bytes())
    }
}

/// Simple SQL minifier that removes comments and collapses whitespace.
fn minify_sql(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Block comments /* ... */
        if ch == '/' && i + 1 < len && chars[i + 1] == '*' {
            i += 2;
            while i < len {
                if chars[i] == '*' && i + 1 < len && chars[i + 1] == '/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Line comments -- ...
        if ch == '-' && i + 1 < len && chars[i + 1] == '-' {
            i += 2;
            while i < len && chars[i] != '\n' && chars[i] != '\r' {
                i += 1;
            }
            continue;
        }

        // Strings and identifiers ('...', "...", `...`)
        if ch == '\'' || ch == '"' || ch == '`' {
            let quote = ch;
            out.push(ch);
            i += 1;
            while i < len {
                out.push(chars[i]);
                if chars[i] == quote {
                    // Check for escaped quote (SQL uses '' for ')
                    if quote == '\'' && i + 1 < len && chars[i + 1] == '\'' {
                        out.push('\'');
                        i += 2;
                        continue;
                    }
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Whitespace
        if ch.is_whitespace() {
            if !out.is_empty() && !out.ends_with(' ') {
                out.push(' ');
            }
            i += 1;
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }
            continue;
        }

        out.push(ch);
        i += 1;
    }

    out.trim().to_string()
}
