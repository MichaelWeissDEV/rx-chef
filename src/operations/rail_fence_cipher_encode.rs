/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rail Fence Cipher Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Rail Fence Cipher Encode operation
///
/// Encodes Strings using the Rail fence Cipher provided a key and an offset.
pub struct RailFenceCipherEncode;

impl Operation for RailFenceCipherEncode {
    fn name(&self) -> &'static str {
        "Rail Fence Cipher Encode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Encodes Strings using the Rail fence Cipher provided a key and an offset."
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

        let plaintext = String::from_utf8_lossy(&input);

        if key > plaintext.len() {
            return Err(OperationError::InvalidInput(
                "Key should be smaller than the plain text's length".to_string(),
            ));
        }

        if offset > plaintext.len() {
            return Err(OperationError::InvalidInput(
                "Offset has to be a positive integer less than plaintext length".to_string(),
            ));
        }

        let cycle = (key - 1) * 2;
        let mut rows: Vec<String> = vec![String::new(); key];

        for pos in 0..plaintext.len() {
            let row_idx = key - 1 - ((cycle / 2 - (pos + offset) % cycle) as i32).abs() as usize;
            if let Some(c) = plaintext.chars().nth(pos) {
                rows[row_idx].push(c);
            }
        }

        Ok(rows.join("").into_bytes())
    }
}
