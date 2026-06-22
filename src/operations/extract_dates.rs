/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract dates operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract dates operation
pub struct ExtractDates;

impl Operation for ExtractDates {
    fn name(&self) -> &'static str {
        "Extract dates"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts dates in the following formats\nyyyy-mm-dd\ndd/mm/yyyy\nmm/dd/yyyy\nDividers can be any of /, -, . or space"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Display total",
            description: "Display total found",
            default_value: "false",
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
        let input_str = String::from_utf8_lossy(&input);
        let display_total = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        let date1 = r"(?:19|20)\d\d[- /.](?:0[1-9]|1[012])[- /.](?:0[1-9]|[12][0-9]|3[01])"; // yyyy-mm-dd
        let date2 = r"(?:0[1-9]|[12][0-9]|3[01])[- /.](?:0[1-9]|1[012])[- /.](?:19|20)\d\d"; // dd/mm/yyyy
        let date3 = r"(?:0[1-9]|1[012])[- /.](?:0[1-9]|[12][0-9]|3[01])[- /.](?:19|20)\d\d"; // mm/dd/yyyy

        let re_str = format!(r"(?i){}|{}|{}", date1, date2, date3);

        let re = Regex::new(&re_str).map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let results: Vec<String> = re
            .find_iter(&input_str)
            .map(|m| m.as_str().to_string())
            .collect();

        let total = results.len();
        let output = results.join("\n");

        if display_total {
            Ok(format!("Total found: {}\n\n{}", total, output).into_bytes())
        } else {
            Ok(output.into_bytes())
        }
    }
}
