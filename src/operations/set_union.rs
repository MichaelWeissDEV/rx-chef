/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Set Union operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::LinkedList;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Set Union operation
///
/// Calculates the union of two sets, preserving insertion order and deduplicating.
pub struct SetUnion;

fn unescape_str(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\0", "\0")
}

impl Operation for SetUnion {
    fn name(&self) -> &'static str {
        "Set Union"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates the union of two sets."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample delimiter",
                description: "Delimiter separating the two input sets",
                default_value: "\\n\\n",
            },
            ArgSchema {
                name: "Item delimiter",
                description: "Delimiter separating items within each set",
                default_value: ",",
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

        let sample_delim = args.first().and_then(|a| a.as_str()).unwrap_or("\\n\\n");
        let item_delim = args.get(1).and_then(|a| a.as_str()).unwrap_or(",");

        let sample_delim = unescape_str(sample_delim);
        let item_delim_str = unescape_str(item_delim);

        let parts: Vec<&str> = text.splitn(2, sample_delim.as_str()).collect();
        if parts.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of sets, perhaps you need to modify the sample delimiter or add more samples?".to_string(),
            ));
        }

        // Use a LinkedList as a seen-checker via Vec + dedup logic
        let mut seen: Vec<String> = Vec::new();
        let mut ordered: LinkedList<String> = LinkedList::new();

        let add_unique = |item: &str, seen: &mut Vec<String>, ordered: &mut LinkedList<String>| {
            if !seen.contains(&item.to_string()) {
                seen.push(item.to_string());
                ordered.push_back(item.to_string());
            }
        };

        for item in parts[0].split(item_delim_str.as_str()) {
            add_unique(item, &mut seen, &mut ordered);
        }
        for item in parts[1].split(item_delim_str.as_str()) {
            add_unique(item, &mut seen, &mut ordered);
        }

        let result_vec: Vec<String> = ordered.into_iter().collect();
        let output = result_vec.join(item_delim_str.as_str());
        Ok(output.into_bytes())
    }
}
