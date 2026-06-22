/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Cetacean Cipher Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Cetacean Cipher Decode operation
pub struct CetaceanCipherDecode;

impl Operation for CetaceanCipherDecode {
    fn name(&self) -> &'static str {
        "Cetacean Cipher Decode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Decode Cetacean Cipher input. <br/><br/>e.g. <code>EEEEEEEEEeeEeEEEEEEEEEEEEeeEeEEe</code> becomes <code>hi</code>"
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

        let mut binary_str = String::new();
        for c in input_str.chars() {
            if c == ' ' {
                // Space is encoded as a 16-bit space character in binary (32)
                binary_str.push_str("0000000000100000");
            } else if c == 'e' {
                binary_str.push('1');
            } else if c == 'E' {
                binary_str.push('0');
            }
        }

        let mut result = String::new();
        for chunk in binary_str.as_bytes().chunks(16) {
            if chunk.len() == 16 {
                let chunk_str = std::str::from_utf8(chunk).map_err(|e| {
                    OperationError::InvalidInput(format!("invalid UTF-8 in cetacean chunk: {}", e))
                })?;
                if let Ok(code) = u32::from_str_radix(chunk_str, 2) {
                    if let Some(c) = std::char::from_u32(code) {
                        result.push(c);
                    }
                }
            }
        }

        Ok(result.into_bytes())
    }
}
