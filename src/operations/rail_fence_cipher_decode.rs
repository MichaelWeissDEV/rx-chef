/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rail Fence Cipher Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Rail Fence Cipher Decode operation
///
/// Decodes Strings that were created using the Rail fence Cipher.
pub struct RailFenceCipherDecode;

impl Operation for RailFenceCipherDecode {
    fn name(&self) -> &'static str {
        "Rail Fence Cipher Decode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Decodes Strings that were created using the Rail fence Cipher provided a key and an offset."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Number of rails (must be >= 2)",
                default_value: "2",
            },
            ArgSchema {
                name: "Offset",
                description: "Offset value",
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
        let key: usize = args.first().and_then(|a| a.as_usize()).unwrap_or(2);
        let offset: usize = args.get(1).and_then(|a| a.as_usize()).unwrap_or(0);

        if key < 2 {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Key must be at least 2".to_string(),
            });
        }

        let cipher = String::from_utf8_lossy(&input);

        if key > cipher.len() {
            return Err(OperationError::InvalidInput(
                "Key should be smaller than the cipher's length".to_string(),
            ));
        }

        if offset > 0 && offset >= cipher.len() {
            return Err(OperationError::InvalidInput(
                "Offset has to be a positive integer less than cipher length".to_string(),
            ));
        }

        let cycle = (key - 1) * 2;
        let mut plaintext = vec![' '; cipher.len()];

        let mut j = 0;
        for y in 0..key {
            for x in 0..cipher.len() {
                let x_eff = (x + offset) % cycle;
                if x_eff == y || x_eff == (cycle - y) % cycle {
                    if j < cipher.len() {
                        plaintext[x] = cipher.chars().nth(j).unwrap_or(' ');
                        j += 1;
                    }
                }
            }
        }

        Ok(plaintext.into_iter().collect::<String>().into_bytes())
    }
}
