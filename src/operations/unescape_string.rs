/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Unescape string operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Unescape string operation
pub struct UnescapeString;

impl Operation for UnescapeString {
    fn name(&self) -> &'static str {
        "Unescape string"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Unescapes characters in a string that have been escaped. For example, <code>Don\\'t stop me now</code> becomes <code>Don't stop me now</code>.<br><br>Supports the following escape sequences:<ul><li><code>\\n</code> (Line feed/newline)</li><li><code>\\r</code> (Carriage return)</li><li><code>\\t</code> (Horizontal tab)</li><li><code>\\b</code> (Backspace)</li><li><code>\\f</code> (Form feed)</li><li><code>\\nnn</code> (Octal, where n is 0-7)</li><li><code>\\xnn</code> (Hex, where n is 0-f)</li><li><code>\\\\</code> (Backslash)</li><li><code>\\'</code> (Single quote)</li><li><code>\\&quot;</code> (Double quote)</li><li><code>\\unnnn</code> (Unicode character)</li><li><code>\\u{nnnnnn}</code> (Unicode code point)</li></ul>"
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
        let re = Regex::new(r#"\\([abfnrtv'\\""]|[0-3][0-7]{2}|[0-7]{1,2}|x[\da-fA-F]{2}|u[\da-fA-F]{4}|u\{[\da-fA-F]{1,6}\})"#).unwrap();

        let result = re.replace_all(&input_str, |caps: &regex::Captures| {
            let a = &caps[1];
            match a.chars().next().unwrap() {
                '\\' => "\\".to_string(),
                '0'..='7' => {
                    if let Ok(val) = u32::from_str_radix(a, 8) {
                        std::char::from_u32(val).unwrap_or('\u{FFFD}').to_string()
                    } else {
                        caps[0].to_string()
                    }
                }
                'a' => "\x07".to_string(),
                'b' => "\x08".to_string(),
                't' => "\t".to_string(),
                'n' => "\n".to_string(),
                'v' => "\x0b".to_string(),
                'f' => "\x0c".to_string(),
                'r' => "\r".to_string(),
                '\'' => "'".to_string(),
                '"' => "\"".to_string(),
                'x' => {
                    if let Ok(val) = u32::from_str_radix(&a[1..], 16) {
                        std::char::from_u32(val).unwrap_or('\u{FFFD}').to_string()
                    } else {
                        caps[0].to_string()
                    }
                }
                'u' => {
                    if a.starts_with("u{") {
                        if let Ok(val) = u32::from_str_radix(&a[2..a.len() - 1], 16) {
                            std::char::from_u32(val).unwrap_or('\u{FFFD}').to_string()
                        } else {
                            caps[0].to_string()
                        }
                    } else {
                        if let Ok(val) = u32::from_str_radix(&a[1..], 16) {
                            std::char::from_u32(val).unwrap_or('\u{FFFD}').to_string()
                        } else {
                            caps[0].to_string()
                        }
                    }
                }
                _ => caps[0].to_string(),
            }
        });

        Ok(result.into_owned().into_bytes())
    }
}
