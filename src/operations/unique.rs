/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Unique operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Unique operation
///
/// Removes duplicate strings from the input, optionally showing the occurrence count.
pub struct Unique;

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

impl Operation for Unique {
    fn name(&self) -> &'static str {
        "Unique"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Removes duplicate strings from the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Line feed, CRLF, Space, Comma, etc.",
                default_value: "Line feed",
            },
            ArgSchema {
                name: "Display count",
                description: "Prefix each unique item with its occurrence count",
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
        let text = String::from_utf8_lossy(&input).into_owned();

        let delim_name = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let display_count = args.get(1).and_then(|a| a.as_bool()).unwrap_or(false);

        let delim = char_rep(delim_name);

        let parts: Vec<&str> = if delim.is_empty() {
            vec![text.as_str()]
        } else {
            text.split(delim.as_str()).collect()
        };

        if display_count {
            // Count occurrences preserving first-seen order
            let mut counts: HashMap<&str, usize> = HashMap::new();
            let mut order: Vec<&str> = Vec::new();
            for item in &parts {
                if let Some(c) = counts.get_mut(item) {
                    *c += 1;
                } else {
                    counts.insert(item, 1);
                    order.push(item);
                }
            }
            let result: Vec<String> = order
                .iter()
                .map(|k| format!("{} {}", counts[k], k))
                .collect();
            let output = result.join(delim.as_str());
            Ok(output.into_bytes())
        } else {
            // Remove duplicates, preserve order
            let mut seen: Vec<&str> = Vec::new();
            for item in &parts {
                if !seen.contains(item) {
                    seen.push(item);
                }
            }
            let output = seen.join(delim.as_str());
            Ok(output.into_bytes())
        }
    }
}
