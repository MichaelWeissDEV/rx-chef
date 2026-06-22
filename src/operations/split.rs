/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Split operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Split operation
///
/// Splits a string into sections around a given delimiter, then rejoins
/// them with a different delimiter.
pub struct Split;

fn char_rep(s: &str) -> String {
    match s {
        "Line feed" => "\n".to_string(),
        "CRLF" => "\r\n".to_string(),
        "Space" => " ".to_string(),
        "Comma" => ",".to_string(),
        "Semi-colon" => ";".to_string(),
        "Colon" => ":".to_string(),
        "Tab" => "\t".to_string(),
        "Forward slash" => "/".to_string(),
        "Backslash" => "\\".to_string(),
        "None" => "".to_string(),
        other => other
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t"),
    }
}

impl Operation for Split {
    fn name(&self) -> &'static str {
        "Split"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Splits a string into sections around a given delimiter."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Split delimiter",
                description: "Delimiter to split on (e.g. Comma, Line feed, or literal string)",
                default_value: "Comma",
            },
            ArgSchema {
                name: "Join delimiter",
                description: "Delimiter to rejoin with (e.g. Line feed, CRLF)",
                default_value: "Line feed",
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
        let text = String::from_utf8_lossy(&input).into_owned();

        let split_delim_raw = args.first().and_then(|a| a.as_str()).unwrap_or("Comma");
        let join_delim_raw = args.get(1).and_then(|a| a.as_str()).unwrap_or("Line feed");

        let split_delim = char_rep(split_delim_raw);
        let join_delim = char_rep(join_delim_raw);

        let _sections: Vec<&str> = if split_delim.is_empty() {
            text.chars()
                .map(|_| "")
                .collect::<Vec<_>>()
                // can't really split on empty in a meaningful way; return as-is
                // fall back to single element
                .into_iter()
                .take(0)
                .collect()
        } else {
            text.split(split_delim.as_str()).collect()
        };

        // Re-do properly: if split delim is empty, we can't split, just return input
        let sections: Vec<&str> = if split_delim.is_empty() {
            vec![text.as_str()]
        } else {
            text.split(split_delim.as_str()).collect()
        };

        let output = sections.join(join_delim.as_str());
        Ok(output.into_bytes())
    }
}
