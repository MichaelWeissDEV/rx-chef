/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Filter operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Filter operation - splits input by delimiter and filters lines by regex.
pub struct Filter;

impl Operation for Filter {
    fn name(&self) -> &'static str {
        "Filter"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Splits up the input using the specified delimiter and then filters each branch based on a regular expression."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description:
                    "Character(s) to split the input on (Line feed, CRLF, Space, Comma, Semi-colon)",
                default_value: "Line feed",
            },
            ArgSchema {
                name: "Regex",
                description: "Regular expression to filter lines",
                default_value: "",
            },
            ArgSchema {
                name: "Invert condition",
                description: "Return lines that do NOT match the regex",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("Line feed");
        let regex_str = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let invert = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

        let delim = match delim_name {
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            other => other,
        };

        if regex_str.is_empty() {
            return Ok(input_str.into_bytes());
        }

        let re = Regex::new(regex_str).map_err(|e| OperationError::InvalidArgument {
            name: "Regex".to_string(),
            reason: format!("Invalid regex: {}", e),
        })?;

        let filtered: Vec<&str> = input_str
            .split(delim)
            .filter(|line| {
                let matches = re.is_match(line);
                if invert {
                    !matches
                } else {
                    matches
                }
            })
            .collect();

        Ok(filtered.join(delim).into_bytes())
    }
}
