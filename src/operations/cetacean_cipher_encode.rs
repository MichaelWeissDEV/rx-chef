/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Cetacean Cipher Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Cetacean Cipher Encode operation
pub struct CetaceanCipherEncode;

impl Operation for CetaceanCipherEncode {
    fn name(&self) -> &'static str {
        "Cetacean Cipher Encode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Converts any input into Cetacean Cipher. <br/><br/>e.g. <code>hi</code> becomes <code>EEEEEEEEEeeEeEEEEEEEEEEEEeeEeEEe</code>"
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

        let mut result = String::new();
        for c in input_str.chars() {
            if c == ' ' {
                result.push(' ');
            } else {
                let code = c as u32;
                // CyberChef uses 16-bit binary representation
                for i in (0..16).rev() {
                    if (code >> i) & 1 == 1 {
                        result.push('e');
                    } else {
                        result.push('E');
                    }
                }
            }
        }

        Ok(result.into_bytes())
    }
}
