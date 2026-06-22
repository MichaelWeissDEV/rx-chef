/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract file paths operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract File Paths operation
pub struct ExtractFilePaths;

impl Operation for ExtractFilePaths {
    fn name(&self) -> &'static str {
        "Extract file paths"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts anything that looks like a Windows or UNIX file path.\n\nNote that if UNIX is selected, there will likely be a lot of false positives."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Windows",
                description: "Include Windows file paths",
                default_value: "true",
            },
            ArgSchema {
                name: "UNIX",
                description: "Include UNIX file paths",
                default_value: "true",
            },
            ArgSchema {
                name: "Display total",
                description: "Display the total number of paths found",
                default_value: "false",
            },
            ArgSchema {
                name: "Sort",
                description: "Sort the results",
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
        let include_win = args.first().and_then(|v| v.as_bool()).unwrap_or(true);
        let include_unix = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let display_total = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let sort = args.get(3).and_then(|v| v.as_bool()).unwrap_or(false);
        let unique = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8_lossy(&input);

        let win_drive = r"[A-Z]:\\";
        let win_name = r"[A-Z\d][A-Z\d\- '_\(\)~]{0,61}";
        let win_ext = r"[A-Z\d]{1,6}";
        let win_path = format!(
            r"{}(?:{}\\?)*{}(?:\.{})?",
            win_drive, win_name, win_name, win_ext
        );
        let unix_path = r"(?:/[A-Z\d.][A-Z\d\-.]{0,61})+".to_string();

        let mut patterns = Vec::new();
        if include_win {
            patterns.push(win_path);
        }
        if include_unix {
            patterns.push(unix_path);
        }

        if patterns.is_empty() {
            return Ok(Vec::new());
        }

        let combined_pattern = patterns.join("|");
        let regex = Regex::new(&format!("(?i){}", combined_pattern))
            .map_err(|e| OperationError::ProcessingError(format!("Invalid regex: {}", e)))?;

        let mut results: Vec<String> = regex
            .find_iter(&input_str)
            .map(|m| m.as_str().to_string())
            .collect();

        if sort {
            results.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        }

        if unique {
            results = results.into_iter().unique().collect();
        }

        let mut output = String::new();
        if display_total {
            output.push_str(&format!("Total found: {}\n\n", results.len()));
        }
        output.push_str(&results.join("\n"));

        Ok(output.into_bytes())
    }
}
