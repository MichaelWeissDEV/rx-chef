/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JSON Beautify operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JSON Beautify operation
pub struct JSONBeautify;

impl Operation for JSONBeautify {
    fn name(&self) -> &'static str {
        "JSON Beautify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Indents and pretty prints JavaScript Object Notation (JSON) code."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Indent string",
                description: "String used for indentation (e.g. tab or spaces)",
                default_value: "    ",
            },
            ArgSchema {
                name: "Sort Object Keys",
                description: "Sort keys in JSON objects alphabetically",
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
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let indent_str = args
            .first()
            .and_then(|a| a.as_str())
            .unwrap_or("    ")
            .to_string();

        let sort_keys = args.get(1).and_then(|a| a.as_bool()).unwrap_or(false);

        let mut json: Value = serde_json::from_str(&input_str).map_err(|e| {
            OperationError::InvalidInput(format!("Unable to parse input as JSON.\n{}", e))
        })?;

        if sort_keys {
            json = sort_json_keys(json);
        }

        let result = format_json_with_indent(&json, &indent_str);
        Ok(result.into_bytes())
    }
}

/// Recursively sort keys in JSON objects
fn sort_json_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted: Vec<(String, Value)> = map.into_iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));
            let new_map: serde_json::Map<String, Value> = sorted
                .into_iter()
                .map(|(k, v)| (k, sort_json_keys(v)))
                .collect();
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_json_keys).collect()),
        other => other,
    }
}

/// Format JSON using a custom indent string.
/// serde_json::to_string_pretty uses 2-space indent; we post-process to substitute.
fn format_json_with_indent(value: &Value, indent: &str) -> String {
    // serde_json pretty uses 2 spaces per level
    let two_space = serde_json::to_string_pretty(value).unwrap_or_default();
    if indent == "  " {
        return two_space;
    }
    replace_indent(&two_space, "  ", indent)
}

/// Replace every leading run of `old_indent` with `new_indent` in each line.
fn replace_indent(s: &str, old_indent: &str, new_indent: &str) -> String {
    let old_len = old_indent.len();
    if old_len == 0 {
        return s.to_string();
    }
    let old_bytes = old_indent.as_bytes();
    s.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut count = 0usize;
            while bytes.len() >= (count + 1) * old_len
                && &bytes[count * old_len..(count + 1) * old_len] == old_bytes
            {
                count += 1;
            }
            if count == 0 {
                line.to_string()
            } else {
                format!("{}{}", new_indent.repeat(count), &line[count * old_len..])
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
