/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract hashes operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract Hashes operation
pub struct ExtractHashes;

impl Operation for ExtractHashes {
    fn name(&self) -> &'static str {
        "Extract hashes"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts potential hashes based on hash character length"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Hash character length",
                description: "The length of the hash in characters",
                default_value: "40",
            },
            ArgSchema {
                name: "All hashes",
                description: "Extract all common hash lengths",
                default_value: "false",
            },
            ArgSchema {
                name: "Display Total",
                description: "Display the total number of hashes found",
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
        let hash_length = args.first().and_then(|v| v.as_usize()).unwrap_or(40);
        let search_all = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
        let display_total = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8_lossy(&input);

        let char_lengths: Vec<usize> = if search_all {
            vec![1, 2, 4, 8, 16, 32, 40, 48, 56, 64, 80, 96, 128, 256]
        } else {
            vec![hash_length]
        };

        let mut all_results = Vec::new();

        for len in char_lengths {
            let pattern = format!(r"(?:\b|^)[a-f0-9]{{{}}}(?:\b|$)", len);
            let regex = Regex::new(&pattern)
                .map_err(|e| OperationError::ProcessingError(format!("Invalid regex: {}", e)))?;

            for m in regex.find_iter(&input_str) {
                all_results.push(m.as_str().to_string());
            }
        }

        let mut output = String::new();
        if display_total {
            output.push_str(&format!("Total Results: {}\n\n", all_results.len()));
        }
        output.push_str(&all_results.join("\n"));

        Ok(output.into_bytes())
    }
}
