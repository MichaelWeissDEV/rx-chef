/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JavaScript Minify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JavaScript Minify operation
pub struct JavaScriptMinify;

impl Operation for JavaScriptMinify {
    fn name(&self) -> &'static str {
        "JavaScript Minify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Safely reduces JavaScript source by removing lexical comments, blank lines, and redundant horizontal whitespace while preserving quoted strings, template literals, escapes, and comment-like text inside them."
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
        let input_str = String::from_utf8(input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;

        let without_comments = strip_comments(&input_str);
        let mut lines = Vec::new();
        for line in without_comments.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                lines.push(collapse_horizontal_whitespace(trimmed));
            }
        }
        Ok(lines.join("\n").into_bytes())
    }
}

fn strip_comments(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    let mut quote = None;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(delimiter) = quote {
            output.push(byte);
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            index += 1;
            continue;
        }
        if matches!(byte, b'\'' | b'"' | b'`') {
            quote = Some(byte);
            output.push(byte);
            index += 1;
        } else if byte == b'/'
            && !matches!(bytes.get(index + 1), Some(b'/') | Some(b'*'))
            && slash_starts_regex(&output)
        {
            output.push(byte);
            index += 1;
            let mut escaped = false;
            let mut character_class = false;
            while index < bytes.len() {
                let current = bytes[index];
                output.push(current);
                index += 1;
                if escaped {
                    escaped = false;
                } else if current == b'\\' {
                    escaped = true;
                } else if current == b'[' {
                    character_class = true;
                } else if current == b']' {
                    character_class = false;
                } else if current == b'/' && !character_class {
                    while index < bytes.len() && bytes[index].is_ascii_alphabetic() {
                        output.push(bytes[index]);
                        index += 1;
                    }
                    break;
                }
            }
        } else if byte == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
        } else if byte == b'/' && bytes.get(index + 1) == Some(&b'*') {
            index += 2;
            while index + 1 < bytes.len() && !(bytes[index] == b'*' && bytes[index + 1] == b'/') {
                if bytes[index] == b'\n' {
                    output.push(b'\n');
                }
                index += 1;
            }
            index = (index + 2).min(bytes.len());
        } else {
            output.push(byte);
            index += 1;
        }
    }
    String::from_utf8(output).expect("source started as UTF-8")
}

fn slash_starts_regex(output: &[u8]) -> bool {
    match output.iter().rev().find(|byte| !byte.is_ascii_whitespace()) {
        None => true,
        Some(byte) => b"=([{,:;!&|?+-*%^~<>".contains(byte),
    }
}

fn collapse_horizontal_whitespace(line: &str) -> String {
    let mut output = String::with_capacity(line.len());
    let mut whitespace = false;
    let mut quote = None;
    let mut escaped = false;
    for character in line.chars() {
        if let Some(delimiter) = quote {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == delimiter {
                quote = None;
            }
        } else if matches!(character, '\'' | '"' | '`') {
            if whitespace && !output.is_empty() {
                output.push(' ');
            }
            whitespace = false;
            quote = Some(character);
            output.push(character);
        } else if character == ' ' || character == '\t' {
            whitespace = true;
        } else {
            if whitespace && !output.is_empty() {
                output.push(' ');
            }
            whitespace = false;
            output.push(character);
        }
    }
    output
}
