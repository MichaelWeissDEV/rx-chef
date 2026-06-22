/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Text-Integer Conversion operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use num_traits::Zero;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Text-Integer Converter operation
///
/// Converts between text strings and large integers (decimal or hexadecimal).
/// Text is interpreted as a big-endian sequence of byte values.
pub struct TextIntegerConverter;

/// Convert a Latin-1 text string to a BigUint (big-endian byte interpretation).
fn text_to_bigint(text: &str) -> Result<BigUint, OperationError> {
    let mut result = BigUint::zero();
    for (i, ch) in text.chars().enumerate() {
        let code = ch as u32;
        if code > 255 {
            return Err(OperationError::InvalidInput(format!(
                "Character at position {} exceeds Latin-1 range (0-255).\nOnly ASCII and Latin-1 characters are supported.",
                i
            )));
        }
        result = (result << 8u32) | BigUint::from(code as u8);
    }
    Ok(result)
}

/// Convert a BigUint to a Latin-1 text string (big-endian byte interpretation).
fn bigint_to_text(mut value: BigUint) -> String {
    if value.is_zero() {
        return String::new();
    }
    let mask = BigUint::from(0xFF_u32);
    let mut bytes: Vec<u8> = Vec::new();
    while !value.is_zero() {
        use num_traits::ToPrimitive;
        let b = (&value & &mask).to_u8().unwrap_or(0);
        bytes.push(b);
        value >>= 8u32;
    }
    bytes.reverse();
    bytes.iter().map(|&b| b as char).collect()
}

impl Operation for TextIntegerConverter {
    fn name(&self) -> &'static str {
        "Text-Integer Conversion"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts between text strings and large integers. Text is interpreted as a big-endian sequence of character codes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Output format",
            description: "String, Decimal, or Hexadecimal",
            default_value: "String",
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
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
        let output_format = args.first().and_then(|v| v.as_str()).unwrap_or("String");
        let trimmed = input_str.trim();

        let big_val: BigUint = if trimmed.is_empty() {
            BigUint::zero()
        } else if let Some(hex_str) = trimmed
            .strip_prefix("0x")
            .or_else(|| trimmed.strip_prefix("0X"))
        {
            // Hexadecimal integer input
            BigUint::parse_bytes(hex_str.as_bytes(), 16).ok_or_else(|| {
                OperationError::InvalidInput(format!("Invalid hex integer: '{}'", trimmed))
            })?
        } else if trimmed.chars().all(|c| c.is_ascii_digit())
            || (trimmed.starts_with('+') && trimmed[1..].chars().all(|c| c.is_ascii_digit()))
        {
            // Decimal integer input
            let digits = trimmed.trim_start_matches('+');
            digits.parse::<BigUint>().map_err(|_| {
                OperationError::InvalidInput(format!("Invalid decimal integer: '{}'", trimmed))
            })?
        } else if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            // Quoted string: strip quotes then convert
            let inner = &trimmed[1..trimmed.len() - 1];
            text_to_bigint(inner)?
        } else {
            // Unquoted text
            text_to_bigint(trimmed)?
        };

        let output = match output_format {
            "String" => bigint_to_text(big_val),
            "Decimal" => big_val.to_string(),
            "Hexadecimal" => format!("0x{}", big_val.to_str_radix(16)),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Output format".to_string(),
                    reason: format!("Unknown format: {}", output_format),
                })
            }
        };

        Ok(output.into_bytes())
    }
}
