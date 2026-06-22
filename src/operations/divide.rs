/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Divide operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Divide operation
///
/// Divides a list of numbers. If an item in the string is not a number it is
/// excluded from the list.  e.g. `0x0a 8 .5` becomes `2.5`
pub struct Divide;

// ---------------------------------------------------------------------------
// Shared arithmetic helpers (also used by multiply, subtract, sum, mean,
// median, standard_deviation)
// ---------------------------------------------------------------------------

/// Map a delimiter name to its character sequence.
pub(crate) fn delim_to_str(delim: &str) -> &'static str {
    match delim {
        "Space" => " ",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "CRLF" => "\r\n",
        _ => "\n", // "Line feed" is the default
    }
}

/// Parse the input string into a Vec<f64>, skipping tokens that are not valid
/// numbers.  Supports `0x`/`0X` hex prefixes and leading-dot floats (`.5`).
pub(crate) fn create_num_array(input: &str, delim: &str) -> Vec<f64> {
    let sep = delim_to_str(delim);
    let mut numbers: Vec<f64> = Vec::new();
    for token in input.split(sep) {
        let t = token.trim();
        if t.is_empty() {
            continue;
        }
        // Hex with 0x/0X prefix
        if let Some(hex_digits) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
            if let Ok(v) = i64::from_str_radix(hex_digits, 16) {
                numbers.push(v as f64);
                continue;
            }
        }
        if let Ok(v) = t.parse::<f64>() {
            if !v.is_nan() {
                numbers.push(v);
            }
        }
    }
    numbers
}

/// Format an f64 as a human-readable string without unnecessary trailing zeros.
pub(crate) fn format_number(v: f64) -> String {
    if v.is_nan() || v.is_infinite() {
        return "NaN".to_string();
    }
    if v.fract() == 0.0 && v.abs() < 1e15 {
        return format!("{}", v as i64);
    }
    // Use up to 15 decimal places then strip trailing zeros / dot
    let raw = format!("{:.15}", v);
    let trimmed = raw.trim_end_matches('0');
    let trimmed = trimmed.trim_end_matches('.');
    trimmed.to_string()
}

impl Operation for Divide {
    fn name(&self) -> &'static str {
        "Divide"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Divides a list of numbers. If an item in the string is not a number it is excluded from the list. e.g. 0x0a 8 .5 becomes 2.5"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "Character that separates numbers in the input",
            default_value: "Line feed",
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
        let delim = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let input_str = String::from_utf8_lossy(&input);
        let nums = create_num_array(&input_str, delim);

        if nums.is_empty() {
            return Ok(b"NaN".to_vec());
        }

        let result = nums.iter().skip(1).fold(nums[0], |acc, &x| acc / x);
        Ok(format_number(result).into_bytes())
    }
}
