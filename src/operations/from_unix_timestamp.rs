/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From UNIX Timestamp operation.
 * -----------------------------------------------------------------------------
 */

use chrono::{TimeZone, Utc};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From UNIX Timestamp operation
pub struct FromUNIXTimestamp;

impl Operation for FromUNIXTimestamp {
    fn name(&self) -> &'static str {
        "From UNIX Timestamp"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a UNIX timestamp to a datetime string."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Units",
            description: "The unit of the timestamp",
            default_value: "Seconds (s)",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Number
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let input_str = input_str.trim();
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let timestamp: f64 = input_str
            .parse()
            .map_err(|_| OperationError::InvalidInput("Input must be a number".to_string()))?;

        let units = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("Seconds (s)");

        let dt = match units {
            "Seconds (s)" => {
                let secs = timestamp as i64;
                let nanos = (timestamp.fract() * 1_000_000_000.0) as u32;
                Utc.timestamp_opt(secs, nanos).single()
            }
            "Milliseconds (ms)" => {
                let secs = (timestamp / 1000.0) as i64;
                let nanos = ((timestamp % 1000.0) * 1_000_000.0) as u32;
                Utc.timestamp_opt(secs, nanos).single()
            }
            "Microseconds (\u{03bc}s)" => {
                let secs = (timestamp / 1_000_000.0) as i64;
                let nanos = ((timestamp % 1_000_000.0) * 1000.0) as u32;
                Utc.timestamp_opt(secs, nanos).single()
            }
            "Nanoseconds (ns)" => {
                let secs = (timestamp / 1_000_000_000.0) as i64;
                let nanos = (timestamp % 1_000_000_000.0) as u32;
                Utc.timestamp_opt(secs, nanos).single()
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Units".to_string(),
                    reason: format!("Unknown units: {}", units),
                })
            }
        };

        if let Some(dt) = dt {
            // CyberChef output format: "ddd D MMMM YYYY HH:mm:ss.SSS UTC"
            // Wait, for seconds it doesn't include .SSS if we follow the JS code
            let formatted = if units == "Seconds (s)" {
                dt.format("%a %-d %B %Y %H:%M:%S UTC").to_string()
            } else {
                dt.format("%a %-d %B %Y %H:%M:%S.%3f UTC").to_string()
            };
            Ok(formatted.into_bytes())
        } else {
            Err(OperationError::InvalidInput(
                "Invalid timestamp".to_string(),
            ))
        }
    }
}
