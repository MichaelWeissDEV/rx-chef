/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Strip HTML tags operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Strip HTML Tags operation
///
/// Removes all HTML tags from the input. Optionally removes leading whitespace and
/// excess blank lines.
pub struct StripHTMLTags;

impl Operation for StripHTMLTags {
    fn name(&self) -> &'static str {
        "Strip HTML tags"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Removes all HTML tags from the input. Optionally removes indentation and excess line breaks."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Remove indentation",
                description: "Remove leading whitespace from lines",
                default_value: "true",
            },
            ArgSchema {
                name: "Remove excess line breaks",
                description: "Collapse multiple blank lines",
                default_value: "true",
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
        let remove_indentation = args
            .get(0)
            .and_then(|a| a.as_str())
            .map(|s| s.to_lowercase() != "false")
            .unwrap_or(true);
        let remove_line_breaks = args
            .get(1)
            .and_then(|a| a.as_str())
            .map(|s| s.to_lowercase() != "false")
            .unwrap_or(true);

        let text = String::from_utf8_lossy(&input);

        let tag_re =
            Regex::new(r"<[^>]*>").map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        let mut result = tag_re.replace_all(&text, "").into_owned();

        if remove_indentation {
            let indent_re = Regex::new(r"\n[ \t\x0c]+")
                .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
            result = indent_re.replace_all(&result, "\n").into_owned();
        }

        if remove_line_breaks {
            // Remove leading blank line
            let leading_re = Regex::new(r"^\s*\n")
                .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
            result = leading_re.replace(&result, "").into_owned();
            // Collapse multiple blank lines
            let multi_re = Regex::new(r"(\n\s*){2,}")
                .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
            result = multi_re.replace_all(&result, "\n").into_owned();
        }

        Ok(result.into_bytes())
    }
}
