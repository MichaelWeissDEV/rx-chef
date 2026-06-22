/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Shuffle operation.
 * -----------------------------------------------------------------------------
 */

use rand::{seq::SliceRandom, thread_rng};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Shuffle operation
///
/// Randomly reorders input elements (lines by default).
pub struct Shuffle;

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

impl Operation for Shuffle {
    fn name(&self) -> &'static str {
        "Shuffle"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Randomly reorders input elements."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "Line feed, CRLF, Space, Comma, etc.",
            default_value: "Line feed",
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
        let text = String::from_utf8_lossy(&input).into_owned();
        if text.is_empty() {
            return Ok(input);
        }

        let delim_name = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let delim = char_rep(delim_name);

        let mut parts: Vec<String> = if delim.is_empty() {
            text.chars().map(|c| c.to_string()).collect()
        } else {
            text.split(delim.as_str()).map(|s| s.to_string()).collect()
        };

        parts.shuffle(&mut thread_rng());

        let output = parts.join(delim.as_str());
        Ok(output.into_bytes())
    }
}
