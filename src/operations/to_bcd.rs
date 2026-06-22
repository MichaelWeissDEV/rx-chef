/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To BCD operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To BCD operation
///
/// Converts a decimal integer string to Binary-Coded Decimal.
pub struct ToBCD;

/// BCD encoding lookup table: index = decimal digit, value = encoded nibble.
fn encoding_lookup(scheme: &str) -> Option<[u8; 10]> {
    match scheme {
        "8 4 2 1" => Some([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        "7 4 2 1" => Some([0, 1, 2, 3, 4, 5, 6, 8, 9, 10]),
        "4 2 2 1" => Some([0, 1, 4, 5, 8, 9, 12, 13, 14, 15]),
        "2 4 2 1" => Some([0, 1, 2, 3, 4, 11, 12, 13, 14, 15]),
        "8 4 -2 -1" => Some([0, 7, 6, 5, 4, 11, 10, 9, 8, 15]),
        "Excess-3" => Some([3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
        "IBM 8 4 2 1" => Some([10, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        _ => None,
    }
}

impl Operation for ToBCD {
    fn name(&self) -> &'static str {
        "To BCD"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Binary-Coded Decimal (BCD) is a class of binary encodings of decimal numbers where each decimal digit is represented by a fixed number of bits, usually four or eight. Special bit patterns are sometimes used for a sign."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Scheme",
                description: "The BCD encoding scheme",
                default_value: "8 4 2 1",
            },
            ArgSchema {
                name: "Packed",
                description: "Whether to pack two digits per byte",
                default_value: "true",
            },
            ArgSchema {
                name: "Signed",
                description: "Whether to include a trailing sign nibble",
                default_value: "false",
            },
            ArgSchema {
                name: "Output format",
                description: "Nibbles, Bytes, or Raw",
                default_value: "Nibbles",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
        let trimmed = input_str.trim();

        if trimmed.is_empty() {
            return Err(OperationError::InvalidInput("Invalid input".to_string()));
        }

        let scheme = args.first().and_then(|v| v.as_str()).unwrap_or("8 4 2 1");
        let packed = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let signed = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);
        let output_format = args.get(3).and_then(|v| v.as_str()).unwrap_or("Nibbles");

        let encoding = encoding_lookup(scheme).ok_or_else(|| OperationError::InvalidArgument {
            name: "Scheme".to_string(),
            reason: format!("Unknown scheme: {}", scheme),
        })?;

        // Parse the number: detect sign then extract digits
        let (negative, digit_str) = if trimmed.starts_with('-') {
            (true, &trimmed[1..])
        } else if trimmed.starts_with('+') {
            (false, &trimmed[1..])
        } else {
            (false, trimmed)
        };

        // Validate all characters are decimal digits and no fractional part
        if !digit_str.chars().all(|c| c.is_ascii_digit()) || digit_str.is_empty() {
            return Err(OperationError::InvalidInput(
                "Input must be an integer value".to_string(),
            ));
        }

        let digits: Vec<u8> = digit_str.chars().map(|c| c as u8 - b'0').collect();

        // Build nibble list from digits
        let mut nibbles: Vec<u8> = digits.iter().map(|&d| encoding[d as usize]).collect();

        if signed {
            if packed && digits.len().is_multiple_of(2) {
                // Prepend a leading 0 nibble so sign nibble stays paired
                nibbles.insert(0, encoding[0]);
            }
            // 12 (0xC) for positive, 13 (0xD) for negative
            let sign_nibble: u8 = if negative { 13 } else { 12 };
            nibbles.push(sign_nibble);
        }

        // Pack nibbles into bytes
        let bytes: Vec<u8> = if packed {
            let mut encoded: u8 = 0;
            let mut little = false;
            let mut result = Vec::new();
            for &n in &nibbles {
                if !little {
                    encoded = n << 4;
                } else {
                    encoded ^= n;
                    result.push(encoded);
                    encoded = 0;
                }
                little = !little;
            }
            if little {
                result.push(encoded);
            }
            result
        } else {
            // Unpacked: each nibble becomes one byte (low nibble only)
            nibbles.iter().map(|&n| n).collect()
        };

        // Build nibble representation for Nibbles/Bytes output
        let output_nibbles: Vec<u8> = if packed {
            // Nibbles output: use the original nibbles list
            nibbles.clone()
        } else {
            // Unpacked: the original nibbles list becomes [0, n] pairs
            let mut nib = Vec::new();
            for &n in &nibbles {
                nib.push(0u8);
                nib.push(n);
            }
            nib
        };

        match output_format {
            "Nibbles" => {
                let parts: Vec<String> = output_nibbles
                    .iter()
                    .map(|&n| format!("{:04b}", n))
                    .collect();
                Ok(parts.join(" ").into_bytes())
            }
            "Bytes" => {
                let parts: Vec<String> = bytes.iter().map(|&b| format!("{:08b}", b)).collect();
                Ok(parts.join(" ").into_bytes())
            }
            "Raw" | _ => {
                // Raw: emit the packed bytes directly
                Ok(bytes)
            }
        }
    }
}
