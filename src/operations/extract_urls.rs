/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract URLs operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract URLs operation
pub struct ExtractURLs;

impl Operation for ExtractURLs {
    fn name(&self) -> &'static str {
        "Extract URLs"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts Uniform Resource Locators (URLs) from the input. The protocol (http, ftp etc.) is required otherwise there will be far too many false positives."
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

        // protocol = "[A-Z]+://",
        // hostname = "[-\\w]+(?:\\.\\w[-\\w]*)+",
        // port = ":\\d+",
        // path = "/[^.!,?\"<>\\[\\]{}\\s\\x7F-\\xFF]*" + "(?:[.!,?]+[^.!,?\"<>\\[\\]{}\\s\\x7F-\\xFF]+)*";
        let pattern = r"(?i)[A-Z]+://[-\w]+(?:\.\w[-\w]*)+(?::\d+)?(?:/[^.!,?<>\[\]{}\s\x7F-\uFFFF]*(?:[.!,?]+[^.!,?<>\[\]{}\s\x7F-\uFFFF]+)*)?";

        let re = Regex::new(pattern).map_err(|e| OperationError::ProcessingError(e.to_string()))?;

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
