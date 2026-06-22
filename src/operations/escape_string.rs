/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Escape string operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, Operation, OperationError};

pub struct EscapeString;

impl Operation for EscapeString {
    fn name(&self) -> &'static str {
        "Escape string"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Escapes special characters in a string so that they do not cause conflicts. For example, <code>Don't stop me now</code> becomes <code>Don\\'t stop me now</code>.<br><br>Supports the following escape sequences:<ul><li><code>\\n</code> (Line feed/newline)</li><li><code>\\r</code> (Carriage return)</li><li><code>\\t</code> (Horizontal tab)</li><li><code>\\b</code> (Backspace)</li><li><code>\\f</code> (Form feed)</li><li><code>\\xnn</code> (Hex, where n is 0-f)</li><li><code>\\\\</code> (Backslash)</li><li><code>\\'</code> (Single quote)</li><li><code>\\&quot;</code> (Double quote)</li><li><code>\\unnnn</code> (Unicode character)</li><li><code>\\u{nnnnnn}</code> (Unicode code point)</li></ul>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Escape level",
                description: "The level of escaping to perform",
                default_value: "Special chars",
            },
            ArgSchema {
                name: "Escape quote",
                description: "Which type of quote to escape",
                default_value: "Single",
            },
            ArgSchema {
                name: "JSON compatible",
                description: "Whether to ensure the output is JSON compatible",
                default_value: "false",
            },
            ArgSchema {
                name: "ES6 compatible",
                description: "Whether to use ES6 unicode escape sequences (\\u{...})",
                default_value: "true",
            },
            ArgSchema {
                name: "Uppercase hex",
                description: "Whether to use uppercase hex digits",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let level = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("Special chars");
        let quotes = args.get(1).and_then(|v| v.as_str()).unwrap_or("Single");
        let json = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let es6 = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);
        let uppercase_hex = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);

        let quote_to_escape = match quotes {
            "Single" => '\'',
            "Double" => '"',
            "Backtick" => '`',
            _ => '\'',
        };

        let mut output = String::new();

        for c in input_str.chars() {
            if level == "Everything" {
                output.push_str(&escape_char(c, json, es6, uppercase_hex));
                continue;
            }

            if c == '\\' {
                output.push_str("\\\\");
                continue;
            }

            if c == quote_to_escape {
                output.push('\\');
                output.push(c);
                continue;
            }

            if level == "Minimal" {
                output.push(c);
                continue;
            }

            // Special chars
            match c {
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                '\u{0008}' => output.push_str("\\b"),
                '\u{000c}' => output.push_str("\\f"),
                _ => {
                    if c.is_control() || !c.is_ascii() {
                        output.push_str(&escape_char(c, json, es6, uppercase_hex));
                    } else {
                        output.push(c);
                    }
                }
            }
        }

        Ok(output.into_bytes())
    }
}

fn escape_char(c: char, json: bool, es6: bool, uppercase: bool) -> String {
    let cp = c as u32;
    if json {
        if cp <= 0xFFFF {
            if uppercase {
                format!("\\u{:04X}", cp)
            } else {
                format!("\\u{:04x}", cp)
            }
        } else {
            // JSON doesn't support \u{...}, it uses surrogate pairs
            let cp = cp - 0x10000;
            let high = (cp >> 10) + 0xD800;
            let low = (cp & 0x3FF) + 0xDC00;
            if uppercase {
                format!("\\u{:04X}\\u{:04X}", high, low)
            } else {
                format!("\\u{:04x}\\u{:04x}", high, low)
            }
        }
    } else if es6 && cp > 0xFF {
        if uppercase {
            format!("\\u{{{:X}}}", cp)
        } else {
            format!("\\u{{{:x}}}", cp)
        }
    } else if cp <= 0xFF {
        if uppercase {
            format!("\\x{:02X}", cp)
        } else {
            format!("\\x{:02x}", cp)
        }
    } else {
        if uppercase {
            format!("\\u{:04X}", cp)
        } else {
            format!("\\u{:04x}", cp)
        }
    }
}
