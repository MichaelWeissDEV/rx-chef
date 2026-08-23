/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Get Time operation.
 * -----------------------------------------------------------------------------
 */

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Get Time operation
///
/// Returns the current Unix timestamp in the chosen granularity.
pub struct GetTime;

fn timestamp_value(duration: Duration, granularity: &str) -> Result<u128, OperationError> {
    match granularity {
        "Nanoseconds (ns)" => Ok(duration.as_nanos()),
        "Microseconds (us)" => Ok(duration.as_micros()),
        "Milliseconds (ms)" => Ok(duration.as_millis()),
        "Seconds (s)" => Ok(duration.as_secs() as u128),
        other => Err(OperationError::InvalidArgument {
            name: "Granularity".to_string(),
            reason: format!("Unknown granularity: {}", other),
        }),
    }
}

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
            kind: crate::operation::ArgKind::String,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn input_requirement(&self) -> crate::operation::InputRequirement {
        crate::operation::InputRequirement::Ignored
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    /// Reads the current clock.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Time]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let granularity = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Milliseconds (ms)");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let value = timestamp_value(now, granularity)?;

        Ok(value.to_string().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::timestamp_value;
    use std::time::Duration;

    #[test]
    fn fixed_posix_duration_has_exact_values_in_every_unit() {
        // 1.5 seconds after the POSIX epoch is exactly representable in all
        // four units and independently defines the expected conversions.
        let fixed = Duration::new(1, 500_000_000);
        assert_eq!(timestamp_value(fixed, "Seconds (s)").unwrap(), 1);
        assert_eq!(timestamp_value(fixed, "Milliseconds (ms)").unwrap(), 1_500);
        assert_eq!(
            timestamp_value(fixed, "Microseconds (us)").unwrap(),
            1_500_000
        );
        assert_eq!(
            timestamp_value(fixed, "Nanoseconds (ns)").unwrap(),
            1_500_000_000
        );
    }
}
