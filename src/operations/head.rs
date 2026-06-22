/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Head operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Head operation
///
/// Like the UNIX head utility. Gets the first n lines (or records split by delimiter).
/// A negative value returns all but the last n lines.
pub struct Head;

fn char_rep(delim: &str) -> String {
    match delim {
        "Line feed" => "\n".to_string(),
        "CRLF" => "\r\n".to_string(),
        "Forward slash" => "/".to_string(),
        "Backslash" => "\\".to_string(),
        "Comma" => ",".to_string(),
        "Semi-colon" => ";".to_string(),
        "Colon" => ":".to_string(),
        "Tab" => "\t".to_string(),
        "Space" => " ".to_string(),
        other => other.to_string(),
    }
}

impl Operation for Head {
    fn name(&self) -> &'static str {
        "Head"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Like the UNIX head utility. Gets the first n lines. You can select all but the last n lines by entering a negative value for n. The delimiter can be changed so that instead of lines, fields are selected instead."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Record delimiter: Line feed, CRLF, Comma, etc.",
                default_value: "Line feed",
            },
            ArgSchema {
                name: "Number",
                description: "Number of lines to take. Negative value = all but last n.",
                default_value: "10",
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
        let delim_name = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let number: i64 = args.get(1).and_then(|a| a.as_i64()).unwrap_or(10);

        let delim = char_rep(delim_name);
        let text = String::from_utf8_lossy(&input);
        let parts: Vec<&str> = text.split(delim.as_str()).collect();
        let len = parts.len() as i64;

        let keep_count = if number < 0 {
            // All but the last |number| items
            let n = len + number;
            if n < 0 {
                0
            } else {
                n as usize
            }
        } else {
            let n = number.min(len);
            n as usize
        };

        let result = parts[..keep_count].join(delim.as_str());
        Ok(result.into_bytes())
    }
}
