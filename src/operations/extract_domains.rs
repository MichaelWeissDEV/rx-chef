/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract domains operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract domains operation
pub struct ExtractDomains;

impl Operation for ExtractDomains {
    fn name(&self) -> &'static str {
        "Extract domains"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts fully qualified domain names.\nNote that this will not include paths. Use Extract URLs to find entire URLs."
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
            ArgSchema {
                name: "Underscore (DMARC, DKIM, etc)",
                description: "Allow underscores in domain labels",
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
        let dmarc = args.get(3).and_then(|v| v.as_bool()).unwrap_or(false);

        // Regex from CyberChef (simplified to remove lookahead)
        let re_str = if dmarc {
            r"(?i)\b(?:(?:xn--|)[a-z0-9_]+(?:-[a-z0-9_]+)*\.)+[a-z]{2,63}\b"
        } else {
            r"(?i)\b(?:(?:xn--|)[a-z0-9]+(?:-[a-z0-9]+)*\.)+[a-z]{2,63}\b"
        };

        let re = Regex::new(re_str).map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let mut results: Vec<String> = re
            .find_iter(&input_str)
            .map(|m| m.as_str().to_string())
            .filter(|s| {
                // Manually enforce the 63 character limit per label
                s.split('.').all(|label| label.len() <= 63)
            })
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
