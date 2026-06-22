/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Offset checker operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Offset Checker operation
///
/// Compares multiple inputs separated by a delimiter and identifies characters
/// that appear at the same position in all samples.  Matching characters are
/// wrapped in [brackets]; non-matching characters are left plain.
pub struct OffsetChecker;

impl Operation for OffsetChecker {
    fn name(&self) -> &'static str {
        "Offset checker"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Compares multiple inputs (separated by the specified delimiter) and marks characters which appear at the same position in all samples. Matching characters are shown in [brackets]."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Sample delimiter",
            description: "String used to separate samples in the input",
            default_value: "\\n\\n",
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
        let raw = String::from_utf8_lossy(&input);

        let delim = args.first().and_then(|a| a.as_str()).unwrap_or("\\n\\n");

        // Unescape common escape sequences in the delimiter
        let delim_resolved = delim
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t");

        let samples: Vec<&str> = raw.split(delim_resolved.as_str()).collect();

        if samples.len() < 2 {
            return Err(OperationError::InvalidInput(
                "Not enough samples. Please ensure the delimiter separates at least two samples."
                    .to_string(),
            ));
        }

        // Convert samples to Vec<char> for indexing
        let char_samples: Vec<Vec<char>> = samples.iter().map(|s| s.chars().collect()).collect();
        let first_len = char_samples[0].len();

        // Build output for each sample
        let mut outputs: Vec<String> = vec![String::new(); samples.len()];
        let mut in_match = false;

        for i in 0..first_len {
            let ch = char_samples[0][i];
            // Check if all samples have the same char at position i
            let all_match = char_samples[1..].iter().all(|s| s.get(i) == Some(&ch));

            for s in 0..samples.len() {
                if let Some(&sc) = char_samples[s].get(i) {
                    if all_match && !in_match {
                        outputs[s].push('[');
                        outputs[s].push(sc);
                        // Close bracket if this is also the end of the sample
                        if char_samples[s].len() == i + 1 {
                            outputs[s].push(']');
                        }
                    } else if !all_match && in_match {
                        outputs[s].push(']');
                        outputs[s].push(sc);
                    } else {
                        outputs[s].push(sc);
                        if in_match && char_samples[s].len() == i + 1 {
                            outputs[s].push(']');
                        }
                    }

                    // Append any remaining chars beyond first_len for last pass
                    if i == first_len - 1 {
                        if in_match && all_match {
                            // bracket already closed above or will be below
                        }
                        let remainder: String = char_samples[s][i + 1..].iter().collect();
                        outputs[s].push_str(&remainder);
                    }
                } else {
                    // This sample is shorter than the first
                    if in_match {
                        outputs[s].push(']');
                    }
                }
            }

            if all_match {
                in_match = true;
            } else {
                in_match = false;
            }
        }

        // Close any open match spans
        if in_match {
            for s in 0..samples.len() {
                // Check if we already appended the closing bracket
                if !outputs[s].ends_with(']') {
                    outputs[s].push(']');
                }
            }
        }

        let result = outputs.join(&delim_resolved);
        Ok(result.into_bytes())
    }
}
