/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Translate DateTime Format operation.
 * -----------------------------------------------------------------------------
 */

use chrono::NaiveDateTime;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Translate DateTime Format operation
///
/// Parses a datetime string in one format and re-writes it in another.
/// Timezone handling is limited to UTC (named timezone DB not included);
/// offset-based timezones can be embedded in the format strings.
pub struct TranslateDateTimeFormat;

impl Operation for TranslateDateTimeFormat {
    fn name(&self) -> &'static str {
        "Translate DateTime Format"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses a datetime string in one format and re-writes it in another. \
         Uses strftime/strptime format strings (e.g. %d/%m/%Y %H:%M:%S). \
         Timezone names are noted but conversion uses UTC unless a numeric \
         offset is embedded in the format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input format string",
                description: "strftime format string for parsing input (e.g. %d/%m/%Y %H:%M:%S)",
                default_value: "%d/%m/%Y %H:%M:%S",
            },
            ArgSchema {
                name: "Input timezone",
                description: "Timezone of the input datetime (informational; UTC assumed unless offset in format)",
                default_value: "UTC",
            },
            ArgSchema {
                name: "Output format string",
                description: "strftime format string for the output (e.g. %A %d %B %Y %H:%M:%S %z)",
                default_value: "%A %d %B %Y %H:%M:%S",
            },
            ArgSchema {
                name: "Output timezone",
                description: "Timezone for the output datetime (informational; UTC assumed unless offset in format)",
                default_value: "UTC",
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
        let input_str = input_str.trim();

        if input_str.is_empty() {
            return Ok(b"Invalid format.".to_vec());
        }

        let input_fmt = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("%d/%m/%Y %H:%M:%S");

        let output_fmt = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("%A %d %B %Y %H:%M:%S");

        // Strip angle brackets from format strings (CyberChef moment.js quirk)
        let input_fmt = input_fmt.replace(['<', '>'], "");
        let output_fmt = output_fmt.replace(['<', '>'], "");

        let naive_dt = match NaiveDateTime::parse_from_str(input_str, &input_fmt) {
            Ok(dt) => dt,
            Err(_) => {
                // Try as NaiveDate
                match chrono::NaiveDate::parse_from_str(input_str, &input_fmt) {
                    Ok(d) => d.and_hms_opt(0, 0, 0).unwrap(),
                    Err(e) => {
                        return Err(OperationError::InvalidInput(format!(
                            "Failed to parse '{}' with format '{}': {}",
                            input_str, input_fmt, e
                        )));
                    }
                }
            }
        };

        let result = format!("{}", naive_dt.format(&output_fmt));
        Ok(result.into_bytes())
    }
}
