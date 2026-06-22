/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Unescape Unicode Characters operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Unescape Unicode Characters operation
pub struct UnescapeUnicodeCharacters;

impl Operation for UnescapeUnicodeCharacters {
    fn name(&self) -> &'static str {
        "Unescape Unicode Characters"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts unicode-escaped character notation back into raw characters.<br><br>Supports the prefixes:<ul><li><code>\\u</code></li><li><code>%u</code></li><li><code>U+</code></li></ul>e.g. <code>\\u03c3\\u03bf\\u03c5</code> becomes <code></code>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Prefix",
            description: "The prefix used for the unicode escape sequence",
            default_value: "\\u",
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
        let prefix = args.first().and_then(|a| a.as_str()).unwrap_or("\\u");
        let input_str = String::from_utf8_lossy(&input);

        let regex_prefix = match prefix {
            "\\u" => r"\\u",
            "%u" => "%u",
            "U+" => r"U\+",
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Prefix".to_string(),
                    reason: "Invalid prefix".to_string(),
                })
            }
        };

        let pattern = format!("{}([a-fA-F0-9]{{4}})", regex_prefix);
        let re = Regex::new(&pattern).unwrap();

        let mut output = String::new();
        let mut last_index = 0;

        for cap in re.captures_iter(&input_str) {
            let m = cap.get(0).unwrap();
            output.push_str(&input_str[last_index..m.start()]);

            if let Ok(val) = u32::from_str_radix(&cap[1], 16) {
                if let Some(c) = std::char::from_u32(val) {
                    output.push(c);
                } else {
                    output.push_str(m.as_str());
                }
            } else {
                output.push_str(m.as_str());
            }
            last_index = m.end();
        }
        output.push_str(&input_str[last_index..]);

        Ok(output.into_bytes())
    }
}
