/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Affine Cipher Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Affine Cipher Encode operation
///
/// The Affine cipher is a type of monoalphabetic substitution cipher, wherein
/// each letter in an alphabet is mapped to its numeric equivalent, encrypted
/// using a simple mathematical function, (ax + b) % 26, and converted back
/// to a letter.
pub struct AffineCipherEncode;

impl Operation for AffineCipherEncode {
    fn name(&self) -> &'static str {
        "Affine Cipher Encode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "The Affine cipher is a type of monoalphabetic substitution cipher, wherein each letter in an alphabet is mapped to its numeric equivalent, encrypted using a simple mathematical function, (ax + b) % 26, and converted back to a letter."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "a",
                description: "Multiplier parameter (must be coprime to 26)",
                default_value: "1",
            },
            ArgSchema {
                name: "b",
                description: "Shift parameter",
                default_value: "0",
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
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        // Parse a and b parameters
        let a = if args.len() > 0 {
            args[0].as_f64().unwrap_or(1.0)
        } else {
            1.0
        };

        let b = if args.len() > 1 {
            args[1].as_f64().unwrap_or(0.0)
        } else {
            0.0
        };

        // Check that a and b are positive integers
        if a < 0.0 || a != a.floor() || b < 0.0 || b != b.floor() {
            return Err(OperationError::InvalidArgument {
                name: "a, b".to_string(),
                reason: "The values of a and b can only be integers.".to_string(),
            });
        }

        let a = a as u32;
        let b = b as u32;

        // Check that a is coprime to 26
        if gcd(a, 26) != 1 {
            return Err(OperationError::InvalidArgument {
                name: "a".to_string(),
                reason: "The value of `a` must be coprime to 26.".to_string(),
            });
        }

        let input_str = String::from_utf8_lossy(&input);
        let mut output = String::new();

        for c in input_str.chars() {
            let lower_c = c.to_lowercase().next().unwrap_or(c);
            if let Some(pos) = alphabet.find(lower_c) {
                let pos = pos as u32;
                let new_pos = (a * pos + b) % 26;
                let new_char = alphabet.chars().nth(new_pos as usize).unwrap_or(c);

                if c.is_uppercase() {
                    output.push(new_char.to_uppercase().next().unwrap_or(new_char));
                } else {
                    output.push(new_char);
                }
            } else {
                // Non-alphabetic characters are preserved
                output.push(c);
            }
        }

        Ok(output.into_bytes())
    }
}

/// Calculate the greatest common divisor of two numbers
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
