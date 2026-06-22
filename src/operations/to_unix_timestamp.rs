/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To UNIX Timestamp operation.
 * -----------------------------------------------------------------------------
 */

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To UNIX Timestamp operation
pub struct ToUNIXTimestamp;

impl Operation for ToUNIXTimestamp {
    fn name(&self) -> &'static str {
        "To UNIX Timestamp"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses a datetime string in UTC and returns the corresponding UNIX timestamp.\n\ne.g. Mon 1 January 2001 11:00:00 becomes 978346800\n\nA UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Units",
                description: "The unit of the timestamp",
                default_value: "Seconds (s)",
            },
            ArgSchema {
                name: "Treat as UTC",
                description: "Treat the input as UTC",
                default_value: "true",
            },
            ArgSchema {
                name: "Show parsed datetime",
                description: "Show the parsed datetime in the output",
                default_value: "true",
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
        let input_str = String::from_utf8_lossy(&input).trim().to_string();
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let units = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("Seconds (s)");
        let treat_as_utc = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let show_datetime = args.get(2).and_then(|v| v.as_bool()).unwrap_or(true);

        let dt = parse_loose_datetime(&input_str, treat_as_utc).ok_or_else(|| {
            OperationError::InvalidInput("Could not parse datetime string".to_string())
        })?;

        let result: f64 = match units {
            "Seconds (s)" => dt.timestamp() as f64,
            "Milliseconds (ms)" => dt.timestamp_millis() as f64,
            "Microseconds (\u{03bc}s)" => dt.timestamp_millis() as f64 * 1000.0,
            "Nanoseconds (ns)" => dt.timestamp_millis() as f64 * 1_000_000.0,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Units".to_string(),
                    reason: format!("Unknown units: {}", units),
                })
            }
        };

        let output = if show_datetime {
            format!(
                "{} ({} UTC)",
                result,
                dt.with_timezone(&Utc).format("%a %-d %B %Y %H:%M:%S")
            )
        } else {
            result.to_string()
        };

        Ok(output.into_bytes())
    }
}

fn parse_loose_datetime(input: &str, treat_as_utc: bool) -> Option<DateTime<Utc>> {
    // Try RFC 3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Some(dt.with_timezone(&Utc));
    }
    // Try RFC 2822
    if let Ok(dt) = DateTime::parse_from_rfc2822(input) {
        return Some(dt.with_timezone(&Utc));
    }

    // Try some common formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d",
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y %H:%M",
        "%d/%m/%Y",
        "%a %d %B %Y %H:%M:%S",
        "%a %-d %B %Y %H:%M:%S",
        "%a %d %B %Y",
        "%d %B %Y %H:%M:%S",
        "%d %B %Y",
    ];

    for fmt in formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(input, fmt) {
            return if treat_as_utc {
                Utc.from_local_datetime(&naive).single()
            } else {
                // For simplicity in this library, we treat non-UTC as UTC if no offset is provided
                Utc.from_local_datetime(&naive).single()
            };
        }

        // Also try parsing as date only and assuming time is 00:00:00
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(input, fmt) {
            if let Some(naive) = naive_date.and_hms_opt(0, 0, 0) {
                return Utc.from_local_datetime(&naive).single();
            }
        }
    }

    // Try parsing as just a number (timestamp in seconds)
    if let Ok(seconds) = input.parse::<i64>() {
        return Utc.timestamp_opt(seconds, 0).single();
    }

    None
}
