/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SQL Beautify operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use regex::{Captures, Regex};
use sqlformat::{format, FormatOptions, Indent, QueryParams};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SQL Beautify operation
pub struct SQLBeautify;

impl Operation for SQLBeautify {
    fn name(&self) -> &'static str {
        "SQL Beautify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Indents and prettifies Structured Query Language (SQL) code."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Indent string",
            description: "String used for indentation (e.g. tab or spaces)",
            default_value: "\\t",
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
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let indent_arg = args.first().and_then(|v| v.as_str()).unwrap_or("\\t");
        let indent_str = match indent_arg {
            "\\t" => "\t",
            "\\n" => "\n",
            "\\r" => "\r",
            _ => indent_arg,
        };

        // Extract and replace bind variables like :Bind1 with __BIND_0__
        let bind_regex = Regex::new(r":\w+").unwrap();
        let mut bind_map = HashMap::new();
        let mut bind_counter = 0;

        let placeholder_input = bind_regex
            .replace_all(&input_str, |caps: &Captures| {
                let placeholder = format!("__BIND_{}__", bind_counter);
                bind_map.insert(placeholder.clone(), caps[0].to_string());
                bind_counter += 1;
                placeholder
            })
            .to_string();

        let mut options = FormatOptions::default();
        options.indent = if indent_str == "\t" {
            Indent::Tabs
        } else {
            Indent::Spaces(indent_str.len() as u8)
        };
        options.uppercase = None;
        options.lines_between_queries = 1;

        let mut formatted = format(&placeholder_input, &QueryParams::None, &options);

        // Replace placeholders back with original bind variables
        let placeholder_regex = Regex::new(r"__BIND_\d+__").unwrap();
        formatted = placeholder_regex
            .replace_all(&formatted, |caps: &Captures| {
                bind_map
                    .get(&caps[0])
                    .cloned()
                    .unwrap_or_else(|| caps[0].to_string())
            })
            .to_string();

        Ok(formatted.into_bytes())
    }
}
