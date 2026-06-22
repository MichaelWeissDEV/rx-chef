/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Get Time operation.
 * -----------------------------------------------------------------------------
 */

use std::time::{SystemTime, UNIX_EPOCH};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Get Time operation
///
/// Returns the current Unix timestamp in the chosen granularity.
pub struct GetTime;

impl Operation for GetTime {
    fn name(&self) -> &'static str {
        "Get Time"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Generates a timestamp showing the amount of time since the UNIX epoch (1970-01-01 00:00:00 UTC). Granularity options: Nanoseconds (ns), Microseconds (us), Milliseconds (ms), Seconds (s)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Granularity",
            description:
                "Time unit: Nanoseconds (ns), Microseconds (us), Milliseconds (ms), Seconds (s)",
            default_value: "Milliseconds (ms)",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let granularity = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Milliseconds (ms)");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let value: u128 = match granularity {
            "Nanoseconds (ns)" => now.as_nanos(),
            "Microseconds (us)" => now.as_micros(),
            "Milliseconds (ms)" => now.as_millis(),
            "Seconds (s)" => now.as_secs() as u128,
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Granularity".to_string(),
                    reason: format!("Unknown granularity: {}", other),
                })
            }
        };

        Ok(value.to_string().into_bytes())
    }
}
