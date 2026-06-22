/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Luhn Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Luhn Checksum operation
///
/// Validates and calculates Luhn checksums for numbers using the Luhn mod N algorithm.
pub struct LuhnChecksum;

impl Operation for LuhnChecksum {
    fn name(&self) -> &'static str {
        "Luhn Checksum"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "The Luhn mod N algorithm using the english alphabet. The Luhn mod N algorithm is an \
         extension to the Luhn algorithm (also known as mod 10 algorithm) that allows it to work \
         with sequences of values in any even-numbered base."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Radix",
            description: "The base/radix to use (must be even, 2-36)",
            default_value: "10",
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
        let input_str = String::from_utf8_lossy(&input).to_string();

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let radix = args.first().and_then(|a| a.as_usize()).unwrap_or(10);

        if radix < 2 || radix > 36 {
            return Err(OperationError::InvalidArgument {
                name: "Radix".to_string(),
                reason: "Radix argument must be between 2 and 36".to_string(),
            });
        }

        if radix % 2 != 0 {
            return Err(OperationError::InvalidArgument {
                name: "Radix".to_string(),
                reason: "Radix argument must be divisible by 2".to_string(),
            });
        }

        let checksum =
            luhn_checksum(&input_str, radix).map_err(|e| OperationError::InvalidInput(e))?;
        let checksum_str = digit_to_char(checksum, radix);

        // checkdigit: checksum of input + "0", adjusted
        let input_with_zero = format!("{}0", input_str);
        let raw_check =
            luhn_checksum(&input_with_zero, radix).map_err(|e| OperationError::InvalidInput(e))?;
        let check_digit = if raw_check == 0 { 0 } else { radix - raw_check };
        let check_digit_str = digit_to_char(check_digit, radix);

        let output = format!(
            "Checksum: {}\nCheckdigit: {}\nLuhn Validated String: {}{}",
            checksum_str, check_digit_str, input_str, check_digit_str
        );

        Ok(output.into_bytes())
    }
}

/// Convert a digit value (0..radix-1) to its character representation.
fn digit_to_char(value: usize, _radix: usize) -> String {
    // Digits 0-9 map to '0'-'9', then 10-35 map to 'a'-'z'
    if value < 10 {
        char::from_digit(value as u32, 10)
            .unwrap_or('?')
            .to_string()
    } else {
        let c = (b'a' + (value as u8 - 10)) as char;
        c.to_string()
    }
}

/// Parse a character in the given radix to its integer value.
fn char_to_digit(c: char, radix: usize) -> Option<usize> {
    let val = c.to_digit(radix as u32)?;
    Some(val as usize)
}

/// Compute the Luhn checksum of input_str in the given radix.
fn luhn_checksum(input_str: &str, radix: usize) -> Result<usize, String> {
    let mut even = false;
    let mut acc: usize = 0;

    for c in input_str.chars().rev() {
        let mut temp = char_to_digit(c, radix)
            .ok_or_else(|| format!("Character: {} is not valid in radix {}.", c, radix))?;

        if even {
            temp *= 2;
            temp = temp / radix + temp % radix;
        }

        even = !even;
        acc += temp;
    }

    Ok(acc % radix)
}
