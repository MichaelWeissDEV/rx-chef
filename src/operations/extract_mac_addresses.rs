/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract MAC addresses operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract MAC addresses operation
pub struct ExtractMACAddresses;

impl Operation for ExtractMACAddresses {
    fn name(&self) -> &'static str {
        "Extract MAC addresses"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts all Media Access Control (MAC) addresses from the input."
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

        let re = Regex::new(r"(?i)[A-F\d]{2}(?:[:-][A-F\d]{2}){5}")
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let mut results: Vec<String> = re
            .find_iter(&input_str)
            .map(|m| m.as_str().to_string())
            .collect();

        if unique {
            results = results.into_iter().unique().collect();
        }

        if sort {
            results.sort_by(|a, b| {
                let a_clean = a.replace(|c: char| !c.is_ascii_hexdigit(), "");
                let b_clean = b.replace(|c: char| !c.is_ascii_hexdigit(), "");
                let a_val = u64::from_str_radix(&a_clean, 16).unwrap_or(0);
                let b_val = u64::from_str_radix(&b_clean, 16).unwrap_or(0);
                a_val.cmp(&b_val)
            });
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
