/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JavaScript Minify operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JavaScript Minify operation
pub struct JavaScriptMinify;

impl Operation for JavaScriptMinify {
    fn name(&self) -> &'static str {
        "JavaScript Minify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Compresses JavaScript code. (Basic implementation using regex)"
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
        let input_str = String::from_utf8(input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid UTF-8: {}", e)))?;

        // Basic minification using regex

        // 1. Remove multi-line comments
        let re_multi = Regex::new(r"(?s)/\*.*?\*/").unwrap();
        let mut minified = re_multi.replace_all(&input_str, "").to_string();

        // 2. Remove single-line comments
        // Need to be careful not to remove comments inside strings, but this is a basic implementation.
        let re_single = Regex::new(r"//.*").unwrap();
        minified = re_single.replace_all(&minified, "").to_string();

        // 3. Remove leading/trailing whitespace from each line and remove empty lines
        let mut lines: Vec<String> = Vec::new();
        for line in minified.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
        }
        minified = lines.join("\n");

        // 4. Remove excessive whitespace around operators (optional, but good for minification)
        // This can be risky without a proper lexer, so let's stick to basic whitespace collapse for now.
        let re_ws = Regex::new(r"[ \t]+").unwrap();
        minified = re_ws.replace_all(&minified, " ").to_string();

        Ok(minified.into_bytes())
    }
}
