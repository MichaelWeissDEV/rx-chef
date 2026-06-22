/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CSS Beautify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CSS Beautify operation
///
/// Indents and prettifies Cascading Style Sheets (CSS) code using pure string
/// manipulation. Handles basic CSS structure including rules, selectors, and
/// block comments. Not a full CSS parser.
pub struct CssBeautify;

impl Operation for CssBeautify {
    fn name(&self) -> &'static str {
        "CSS Beautify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Indents and prettifies Cascading Style Sheets (CSS) code."
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

        let result = beautify_css(&text, indent_str);
        Ok(result.into_bytes())
    }
}

/// Beautify CSS by tracking brace depth and inserting newlines/indentation.
fn beautify_css(input: &str, indent: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    let mut depth: usize = 0;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    fn add_indent(out: &mut String, depth: usize, indent: &str) {
        for _ in 0..depth {
            out.push_str(indent);
        }
    }

    while i < len {
        // Skip leading whitespace
        while i < len && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= len {
            break;
        }

        // Check for block comment
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            add_indent(&mut out, depth, indent);
            out.push_str("/*");
            i += 2;
            while i < len {
                if chars[i] == '*' && i + 1 < len && chars[i + 1] == '/' {
                    out.push_str("*/\n");
                    i += 2;
                    break;
                }
                out.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // Read a token up to structural chars: '{', '}', ';', ':', '/' (if comment)
        let mut token = String::new();
        let mut in_str = None;
        let mut structural_char = None;

        while i < len {
            let ch = chars[i];

            // Check for comment starting inside a token
            if ch == '/' && i + 1 < len && chars[i + 1] == '*' && in_str.is_none() {
                break;
            }

            if in_str.is_none() && (ch == '\'' || ch == '"') {
                in_str = Some(ch);
                token.push(ch);
                i += 1;
                continue;
            }

            if let Some(quote) = in_str {
                token.push(ch);
                if ch == quote {
                    // Check if escaped
                    let mut backslashes = 0;
                    let mut k = token.len() - 2;
                    while k > 0 && token.as_bytes()[k] == b'\\' {
                        backslashes += 1;
                        k -= 1;
                    }
                    if backslashes % 2 == 0 {
                        in_str = None;
                    }
                }
                i += 1;
                continue;
            }

            if ch == '{' || ch == '}' || ch == ';' {
                structural_char = Some(ch);
                i += 1;
                break;
            }

            if ch == ':' {
                // Peek ahead to see if there is a '{' before any ';' or '}'
                let mut is_selector = false;
                let mut j = i + 1;
                while j < len {
                    let next_ch = chars[j];
                    if next_ch == '{' {
                        is_selector = true;
                        break;
                    }
                    if next_ch == ';' || next_ch == '}' {
                        break;
                    }
                    j += 1;
                }
                if is_selector {
                    token.push(ch);
                    i += 1;
                    continue;
                } else {
                    structural_char = Some(ch);
                    i += 1;
                    break;
                }
            }

            token.push(ch);
            i += 1;
        }

        let trimmed_token = token.trim();

        if !trimmed_token.is_empty() {
            if out.ends_with('\n') || out.is_empty() {
                add_indent(&mut out, depth, indent);
            }
            out.push_str(trimmed_token);
        }

        if let Some(ch) = structural_char {
            match ch {
                '{' => {
                    // Space before brace
                    if !out.is_empty() && !out.ends_with('\n') && !out.ends_with(' ') {
                        out.push(' ');
                    }
                    out.push_str(" {\n");
                    depth += 1;
                }
                '}' => {
                    if !out.ends_with('\n') {
                        out.push('\n');
                    }
                    if depth > 0 {
                        depth -= 1;
                    }
                    add_indent(&mut out, depth, indent);
                    out.push_str("}\n");
                }
                ':' => {
                    out.push_str(": ");
                }
                ';' => {
                    out.push_str(";\n");
                }
                _ => {}
            }
        }
    }

    out.trim_end().to_string()
}
