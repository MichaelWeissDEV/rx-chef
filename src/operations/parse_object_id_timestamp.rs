/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse ObjectID timestamp operation.
 * -----------------------------------------------------------------------------
 */

use chrono::{DateTime, TimeZone, Utc};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse ObjectID Timestamp operation
///
/// Extracts a Unix timestamp from the first 4 bytes of a MongoDB/BSON ObjectID
/// (24 hex characters) and formats it as a human-readable UTC date/time.
pub struct ParseObjectIDTimestamp;

impl Operation for ParseObjectIDTimestamp {
    fn name(&self) -> &'static str {
        "Parse ObjectID timestamp"
    }

    fn module(&self) -> &'static str {
        "Serialise"
    }

    fn description(&self) -> &'static str {
        "Parse timestamp from MongoDB/BSON ObjectID hex string. The first 4 bytes of the ObjectID encode a Unix timestamp."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let raw = String::from_utf8_lossy(&input);
        let raw = raw.trim();

        if raw.is_empty() {
            return Err(OperationError::InvalidInput(
                "No input provided".to_string(),
            ));
        }

        // ObjectID is 12 bytes = 24 hex chars; accept with/without hyphens/spaces
        let clean: String = raw.chars().filter(|c| c.is_ascii_hexdigit()).collect();

        if clean.len() < 8 {
            return Err(OperationError::InvalidInput(
                "Input too short: need at least 4 bytes (8 hex chars) for an ObjectID timestamp"
                    .to_string(),
            ));
        }

        // First 4 bytes (8 hex chars) are the big-endian Unix timestamp
        let ts_hex = &clean[..8];
        let ts_secs = u32::from_str_radix(ts_hex, 16).map_err(|e| {
            OperationError::InvalidInput(format!(
                "Failed to parse timestamp hex '{}': {}",
                ts_hex, e
            ))
        })?;

        let dt: DateTime<Utc> = Utc
            .timestamp_opt(ts_secs as i64, 0)
            .single()
            .ok_or_else(|| {
                OperationError::ProcessingError(format!("Invalid Unix timestamp: {}", ts_secs))
            })?;

        // JS produces ISO 8601 with milliseconds: 2020-01-01T00:00:00.000Z
        let output = dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        Ok(output.into_bytes())
    }
}
