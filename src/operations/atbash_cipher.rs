/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Atbash Cipher operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Atbash Cipher operation
///
/// Atbash is a mono-alphabetic substitution cipher originally used to encode
/// the Hebrew alphabet. It has been modified here for use with the Latin alphabet.
pub struct AtbashCipher;

impl Operation for AtbashCipher {
    fn name(&self) -> &'static str {
        "Atbash Cipher"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Atbash is a mono-alphabetic substitution cipher originally used to encode the Hebrew alphabet. It has been modified here for use with the Latin alphabet."
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut output = String::new();

        for c in input_str.chars() {
            if let Some(pos) = alphabet.find(c.to_ascii_lowercase()) {
                // Atbash: map a<->z, b<->y, c<->x, etc.
                // Output is always lowercase
                let new_pos = 25 - pos as i32;
                let new_char = alphabet.chars().nth(new_pos as usize).ok_or_else(|| {
                    OperationError::ProcessingError(format!(
                        "character index {} out of range",
                        new_pos
                    ))
                })?;
                output.push(new_char);
            } else {
                // Non-alphabetic characters are kept as-is
                output.push(c);
            }
        }

        Ok(output.into_bytes())
    }
}
