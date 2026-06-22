/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base operation - converts a number from a given radix (2-36) to decimal.
pub struct FromBase;

impl Operation for FromBase {
    fn name(&self) -> &'static str {
        "From Base"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a number to decimal from a given numerical base (radix 2-36)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Radix",
            description: "The base of the input number (2-36)",
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

        let trimmed = input_str.trim().replace(|c: char| c.is_whitespace(), "");
        if trimmed.is_empty() {
            return Ok(b"0".to_vec());
        }

        // Split on decimal point to handle fractional parts
        let parts: Vec<&str> = trimmed.splitn(2, '.').collect();
        let integer_part = parts[0];

        // Parse integer part using u128 for large numbers, fall back to i128 for negatives
        let is_negative = integer_part.starts_with('-');
        let abs_str = if is_negative {
            &integer_part[1..]
        } else {
            integer_part
        };

        let integer_val = u128::from_str_radix(abs_str, radix as u32).map_err(|e| {
            OperationError::InvalidInput(format!(
                "Cannot parse '{}' in base {}: {}",
                abs_str, radix, e
            ))
        })?;

        let result_int: i128 = if is_negative {
            -(integer_val as i128)
        } else {
            integer_val as i128
        };

        if parts.len() == 1 {
            return Ok(result_int.to_string().into_bytes());
        }

        // Fractional part: sum digit_i / radix^(i+1)
        let frac_str = parts[1];
        let mut frac_val: f64 = 0.0;
        for (i, c) in frac_str.chars().enumerate() {
            let digit = c.to_digit(radix as u32).ok_or_else(|| {
                OperationError::InvalidInput(format!("Invalid digit '{}' for base {}", c, radix))
            })?;
            frac_val += digit as f64 / (radix as f64).powi((i + 1) as i32);
        }

        let total = result_int as f64 + if is_negative { -frac_val } else { frac_val };
        Ok(format!("{}", total).into_bytes())
    }
}
