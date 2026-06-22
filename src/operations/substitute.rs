/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Substitute operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Substitute operation
pub struct Substitute;

impl Operation for Substitute {
    fn name(&self) -> &'static str {
        "Substitute"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "A substitution cipher allowing you to specify bytes to replace with other byte values. This can be used to create Caesar ciphers but is more powerful as any byte value can be substituted, not just letters, and the substitution values need not be in order.<br><br>Enter the bytes you want to replace in the Plaintext field and the bytes to replace them with in the Ciphertext field.<br><br>Non-printable bytes can be specified using string escape notation. For example, a line feed character can be written as either <code>\\n</code> or <code>\\x0a</code>.<br><br>Byte ranges can be specified using a hyphen. For example, the sequence <code>0123456789</code> can be written as <code>0-9</code>.<br><br>Note that blackslash characters are used to escape special characters, so will need to be escaped themselves if you want to use them on their own (e.g.<code>\\\\</code>)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Plaintext",
                description: "The bytes you want to replace",
                default_value: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            },
            ArgSchema {
                name: "Ciphertext",
                description: "The bytes to replace them with",
                default_value: "XYZABCDEFGHIJKLMNOPQRSTUVW",
            },
            ArgSchema {
                name: "Ignore case",
                description: "If true, the case of the input character is preserved.",
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
        let plaintext_raw = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let ciphertext_raw = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let ignore_case = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

        let plaintext_unescaped = unescape(plaintext_raw);
        let ciphertext_unescaped = unescape(ciphertext_raw);

        let plaintext = expand_alph_range(&plaintext_unescaped);
        let ciphertext = expand_alph_range(&ciphertext_unescaped);

        let mut output = String::new();
        if plaintext.len() != ciphertext.len() {
            output.push_str("Warning: Plaintext and Ciphertext lengths differ\n\n");
        }

        let mut dict = HashMap::new();
        let len = std::cmp::min(plaintext.len(), ciphertext.len());
        for i in 0..len {
            dict.insert(plaintext[i], ciphertext[i]);
        }

        let input_str = String::from_utf8_lossy(&input);
        for c in input_str.chars() {
            output.push(self.cipher_single_char(c, &dict, ignore_case));
        }

        Ok(output.into_bytes())
    }
}

impl Substitute {
    fn cipher_single_char(&self, c: char, dict: &HashMap<char, char>, ignore_case: bool) -> char {
        if !ignore_case {
            return *dict.get(&c).unwrap_or(&c);
        }

        let is_upper = c.is_uppercase();

        if let Some(&replacement) = dict.get(&c) {
            return if is_upper {
                replacement.to_uppercase().next().unwrap_or(replacement)
            } else {
                replacement.to_lowercase().next().unwrap_or(replacement)
            };
        }

        // Check for the other case
        if is_upper {
            let lower = c.to_lowercase().next().unwrap_or(c);
            if let Some(&replacement) = dict.get(&lower) {
                return replacement.to_uppercase().next().unwrap_or(replacement);
            }
        } else {
            let upper = c.to_uppercase().next().unwrap_or(c);
            if let Some(&replacement) = dict.get(&upper) {
                return replacement.to_lowercase().next().unwrap_or(replacement);
            }
        }

        c
    }
}

fn unescape(s: &str) -> String {
    let re = Regex::new(r"\\([abfnrtv'\\&quot;]|[0-3][0-7]{2}|[0-7]{1,2}|x[\da-fA-F]{2}|u[\da-fA-F]{4}|u\{[\da-fA-F]{1,6}\})").unwrap();

    re.replace_all(s, |caps: &regex::Captures| {
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
    })
    .into_owned()
}

fn expand_alph_range(alph_str: &str) -> Vec<char> {
    let chars: Vec<char> = alph_str.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '-' && chars[i] != '\\' {
            let start = chars[i] as u32;
            let end = chars[i + 2] as u32;
            for j in start..=end {
                if let Some(c) = std::char::from_u32(j) {
                    result.push(c);
                }
            }
            i += 3;
        } else if i + 1 < chars.len() && chars[i] == '\\' && chars[i + 1] == '-' {
            result.push('-');
            i += 2;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
