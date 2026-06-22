/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Escape Unicode Characters operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, Operation, OperationError};

pub struct EscapeUnicodeCharacters;

impl Operation for EscapeUnicodeCharacters {
    fn name(&self) -> &'static str {
        "Escape Unicode Characters"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts characters to their unicode-escaped notations.<br><br>Supports the prefixes:<ul><li><code>\\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code></code> becomes <code>\\u03C3\\u03BF\\u03C5</code>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Prefix",
                description: "The prefix to use for each escape sequence",
                default_value: "\\u",
            },
            ArgSchema {
                name: "Encode all chars",
                description: "If true, all characters will be escaped. If false, only non-printable ASCII characters will be escaped.",
                default_value: "false",
            },
            ArgSchema {
                name: "Padding",
                description: "The number of hex digits to pad to",
                default_value: "4",
            },
            ArgSchema {
                name: "Uppercase hex",
                description: "Whether to use uppercase hex digits",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let prefix = args.first().and_then(|v| v.as_str()).unwrap_or("\\u");
        let encode_all = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
        let padding = args.get(2).and_then(|v| v.as_usize()).unwrap_or(4);
        let uppercase_hex = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);

        let mut output = String::new();

        for c in input_str.chars() {
            if !encode_all && (c >= ' ' && c <= '~') {
                output.push(c);
                continue;
            }

            let cp = c as u32;
            let hex = if uppercase_hex {
                format!("{:0>width$X}", cp, width = padding)
            } else {
                format!("{:0>width$x}", cp, width = padding)
            };

            output.push_str(prefix);
            output.push_str(&hex);
        }

        Ok(output.into_bytes())
    }
}
