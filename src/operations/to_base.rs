/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Base operation - converts a decimal number to a given numerical base (2-36).
pub struct ToBase;

impl Operation for ToBase {
    fn name(&self) -> &'static str {
        "To Base"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a decimal number to a given numerical base (radix 2-36)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Radix",
            description: "The base to convert to (2-36)",
            default_value: "36",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let radix = args.first().and_then(|v| v.as_usize()).unwrap_or(36);

        if radix < 2 || radix > 36 {
            return Err(OperationError::InvalidArgument {
                name: "Radix".to_string(),
                reason: "Radix must be between 2 and 36".to_string(),
            });
        }

        let trimmed = input_str.trim();
        if trimmed.is_empty() {
            return Err(OperationError::InvalidInput(
                "Input must be a number".to_string(),
            ));
        }

        let is_negative = trimmed.starts_with('-');
        let abs_str = if is_negative { &trimmed[1..] } else { trimmed };

        let n: u128 = abs_str.parse::<u128>().map_err(|e| {
            OperationError::InvalidInput(format!("Cannot parse '{}' as integer: {}", trimmed, e))
        })?;

        let converted = to_base_string(n, radix as u32);
        let result = if is_negative {
            format!("-{}", converted)
        } else {
            converted
        };

        Ok(result.into_bytes())
    }
}

/// Convert a u128 integer to the given base string using digits 0-9 then a-z.
fn to_base_string(mut n: u128, radix: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut buf = Vec::new();
    while n > 0 {
        buf.push(DIGITS[(n % radix as u128) as usize]);
        n /= radix as u128;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap_or_default()
}
