/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract email addresses operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract email addresses operation
pub struct ExtractEmailAddresses;

impl Operation for ExtractEmailAddresses {
    fn name(&self) -> &'static str {
        "Extract email addresses"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts all email addresses from the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Display total",
                description: "Display total found",
                default_value: "false",
            },
            ArgSchema {
                name: "Sort",
                description: "Sort results",
                default_value: "false",
            },
            ArgSchema {
                name: "Unique",
                description: "Remove duplicate results",
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
        let display_total = args.first().and_then(|v| v.as_bool()).unwrap_or(false);
        let sort = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
        let unique = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

        // Regex from CyberChef
        let re_str = r#"(?i)(?:[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9](?:[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9-]*[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9])?\.)+[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9](?:[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9-]*[\u{00A0}-\u{D7FF}\u{E000}-\u{FFFF}a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\])"#;

        let re = Regex::new(re_str).map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let mut results: Vec<String> = re
            .find_iter(&input_str)
            .map(|m| m.as_str().to_string())
            .collect();

        if unique {
            results = results.into_iter().unique().collect();
        }

        if sort {
            results.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        }

        let total = results.len();
        let output = results.join("\n");

        if display_total {
            Ok(format!("Total found: {}\n\n{}", total, output).into_bytes())
        } else {
            Ok(output.into_bytes())
        }
    }
}
