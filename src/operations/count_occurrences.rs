/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Count occurrences operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Count occurrences operation
///
/// Counts the number of times the provided string occurs in the input.
pub struct CountOccurrences;

impl Operation for CountOccurrences {
    fn name(&self) -> &'static str {
        "Count occurrences"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Counts the number of times the provided string occurs in the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Search string",
            description: "The string to search for",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let search_str = args.first().and_then(|a| a.as_str()).unwrap_or("");

        if search_str.is_empty() {
            return Ok(b"0".to_vec());
        }

        let input_str = String::from_utf8_lossy(&input);
        let count = count_occurrences(&input_str, search_str);

        Ok(count.to_string().into_bytes())
    }
}

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut idx = 0;

    while let Some(pos) = haystack[idx..].find(needle) {
        count += 1;
        idx += pos + needle.len();
    }

    count
}
