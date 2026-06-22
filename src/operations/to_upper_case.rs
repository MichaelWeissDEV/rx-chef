/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Upper case operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Converts the input string to upper case.
///
/// The "Scope" argument controls whether all characters, only the first letter
/// of each word, the first letter of each sentence, or the first letter of each
/// paragraph is capitalised.
pub struct ToUpperCase;

/// Apply regex-driven capitalisation: capture group 1 is the letter to upper-case.
fn capitalise_by_regex(s: &str, pattern: &str) -> Result<String, OperationError> {
    let re = Regex::new(pattern).map_err(|e| OperationError::ProcessingError(e.to_string()))?;
    let matches: Vec<(usize, usize, String)> = re
        .captures_iter(s)
        .filter_map(|caps| {
            let cap = caps.get(1)?;
            Some((cap.start(), cap.end(), cap.as_str().to_uppercase()))
        })
        .collect();
    let mut result = s.as_bytes().to_vec();
    for (start, end, replacement) in matches.into_iter().rev() {
        result.splice(start..end, replacement.into_bytes());
    }
    Ok(String::from_utf8_lossy(&result).into_owned())
}

impl Operation for ToUpperCase {
    fn name(&self) -> &'static str {
        "To Upper case"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts the input string to upper case, optionally limiting scope to only \
         the first character in each word, sentence or paragraph."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Scope",
            description: "All, Word, Sentence, or Paragraph",
            default_value: "All",
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
        let scope = args.first().and_then(|a| a.as_str()).unwrap_or("All");
        let s = String::from_utf8_lossy(&input).into_owned();

        let result = match scope {
            "All" => s.to_uppercase(),
            "Word" => {
                // Capitalise the first word-char after every word boundary.
                let re = Regex::new(r"\b(\w)")
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                let matches: Vec<(usize, usize, String)> = re
                    .captures_iter(&s)
                    .filter_map(|caps| {
                        let cap = caps.get(1)?;
                        Some((cap.start(), cap.end(), cap.as_str().to_uppercase()))
                    })
                    .collect();
                let mut result = s.into_bytes();
                for (start, end, replacement) in matches.into_iter().rev() {
                    result.splice(start..end, replacement.into_bytes());
                }
                String::from_utf8_lossy(&result).into_owned()
            }
            "Sentence" => {
                // Capitalise the first word-char after "." or at the very start.
                // (?m) makes ^ match at every line start as in the JS original.
                capitalise_by_regex(&s, r"(?m)(?:\.|^)\s*(\b\w)")?
            }
            "Paragraph" => {
                // Capitalise the first word-char after a newline or at the start.
                capitalise_by_regex(&s, r"(?m)(?:\n|^)\s*(\b\w)")?
            }
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Scope".to_string(),
                    reason: format!("unrecognised scope: {}", other),
                })
            }
        };

        Ok(result.into_bytes())
    }
}
