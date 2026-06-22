/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CSS Minify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CSS Minify operation
///
/// Compresses Cascading Style Sheets (CSS) code by removing unnecessary
/// whitespace and optionally stripping comments.
pub struct CssMinify;

impl Operation for CssMinify {
    fn name(&self) -> &'static str {
        "CSS Minify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Compresses Cascading Style Sheets (CSS) code by removing unnecessary whitespace \
        and optionally stripping block comments."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Preserve comments",
            description: "Keep CSS block comments in output",
            default_value: "false",
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
        let preserve_comments = args.first().and_then(|a| a.as_bool()).unwrap_or(false);

        let result = minify_css(&text, preserve_comments);
        Ok(result.into_bytes())
    }
}

/// Minify CSS by collapsing whitespace and optionally stripping comments.
fn minify_css(input: &str, preserve_comments: bool) -> String {
    let mut out = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Handle block comments
        if ch == '/' && i + 1 < len && chars[i + 1] == '*' {
            i += 2;
            let mut comment = String::from("/*");
            while i < len {
                comment.push(chars[i]);
                if chars[i] == '*' && i + 1 < len && chars[i + 1] == '/' {
                    comment.push('/');
                    i += 2;
                    break;
                }
                i += 1;
            }
            if preserve_comments {
                out.push_str(&comment);
            }
            continue;
        }

        // Handle strings in CSS (e.g. content:"...") - preserve them verbatim
        if ch == '"' || ch == '\'' {
            let quote = ch;
            out.push(ch);
            i += 1;
            while i < len {
                out.push(chars[i]);
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                if chars[i] == '\\' && i + 1 < len {
                    i += 1;
                    out.push(chars[i]);
                }
                i += 1;
            }
            continue;
        }

        // Collapse whitespace
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            // Emit a single space only if needed (not after { } ; or at start)
            if !out.is_empty() {
                let last = out.chars().last().unwrap_or(' ');
                if last != '{' && last != '}' && last != ';' && last != ':' && last != ',' {
                    // Peek ahead to see if next non-whitespace is a structural char
                    let mut j = i + 1;
                    while j < len
                        && (chars[j] == ' '
                            || chars[j] == '\t'
                            || chars[j] == '\n'
                            || chars[j] == '\r')
                    {
                        j += 1;
                    }
                    let next = if j < len { chars[j] } else { '\0' };
                    if next != '{' && next != '}' && next != ';' && next != ':' && next != ',' {
                        out.push(' ');
                    }
                }
            }
            i += 1;
            // skip remaining whitespace
            while i < len
                && (chars[i] == ' ' || chars[i] == '\t' || chars[i] == '\n' || chars[i] == '\r')
            {
                i += 1;
            }
            continue;
        }

        out.push(ch);
        i += 1;
    }

    out.trim().to_string()
}
