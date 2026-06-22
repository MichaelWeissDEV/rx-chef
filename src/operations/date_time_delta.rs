/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the DateTime Delta operation.
 * -----------------------------------------------------------------------------
 */

use chrono::{Duration, NaiveDateTime};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// DateTime Delta operation
pub struct DateTimeDelta;

impl Operation for DateTimeDelta {
    fn name(&self) -> &'static str {
        "DateTime Delta"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates a new DateTime value given an input DateTime value and a time difference (delta) from the input DateTime value. \
         Uses strftime format strings."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Built in formats",
                description: "Common datetime formats",
                default_value: "Standard date and time",
            },
            ArgSchema {
                name: "Input format string",
                description:
                    "strftime format string for parsing and formatting (e.g. %d/%m/%Y %H:%M:%S)",
                default_value: "%d/%m/%Y %H:%M:%S",
            },
            ArgSchema {
                name: "Time Operation",
                description: "Whether to add or subtract the delta",
                default_value: "Add",
            },
            ArgSchema {
                name: "Days",
                description: "Number of days",
                default_value: "0",
            },
            ArgSchema {
                name: "Hours",
                description: "Number of hours",
                default_value: "0",
            },
            ArgSchema {
                name: "Minutes",
                description: "Number of minutes",
                default_value: "0",
            },
            ArgSchema {
                name: "Seconds",
                description: "Number of seconds",
                default_value: "0",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let input_str = input_str.trim();

        if input_str.is_empty() {
            return Ok(b"Invalid format.".to_vec());
        }

        let input_fmt = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("%d/%m/%Y %H:%M:%S");

        // Strip angle brackets from format strings (CyberChef moment.js quirk)
        let input_fmt = input_fmt.replace(['<', '>'], "");

        let op_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Add");

        let days = args.get(3).and_then(|a| a.as_f64()).unwrap_or(0.0);
        let hours = args.get(4).and_then(|a| a.as_f64()).unwrap_or(0.0);
        let mins = args.get(5).and_then(|a| a.as_f64()).unwrap_or(0.0);
        let secs = args.get(6).and_then(|a| a.as_f64()).unwrap_or(0.0);

        let total_ms_f64 =
            days * 86_400_000.0 + hours * 3_600_000.0 + mins * 60_000.0 + secs * 1000.0;
        let total_ms = total_ms_f64.round() as i64;
        let duration = Duration::milliseconds(total_ms);

        let naive_dt = NaiveDateTime::parse_from_str(input_str, &input_fmt).map_err(|e| {
            OperationError::InvalidInput(format!(
                "Failed to parse '{}' with format '{}': {}",
                input_str, input_fmt, e
            ))
        })?;

        let new_dt = if op_type == "Add" {
            naive_dt.checked_add_signed(duration)
        } else {
            naive_dt.checked_sub_signed(duration)
        };

        let new_dt = new_dt.ok_or_else(|| {
            OperationError::ProcessingError("DateTime calculation out of bounds".to_string())
        })?;

        let result = format!("{}", new_dt.format(&input_fmt));
        Ok(result.into_bytes())
    }
}
