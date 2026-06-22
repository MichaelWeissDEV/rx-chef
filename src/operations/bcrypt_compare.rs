/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bcrypt compare operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bcrypt compare operation
///
/// Tests whether the input matches the given bcrypt hash.
pub struct BcryptCompare;

impl Operation for BcryptCompare {
    fn name(&self) -> &'static str {
        "Bcrypt compare"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Tests whether the input matches the given bcrypt hash. To test multiple possible passwords, use the Fork operation."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Hash",
            description: "Bcrypt hash to compare against",
            default_value: "",
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
        let hash = args.first().and_then(|a| a.as_str()).ok_or_else(|| {
            OperationError::InvalidArgument {
                name: "Hash".to_string(),
                reason: "Hash argument is required".to_string(),
            }
        })?;

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // Verify the hash is valid bcrypt format
        let is_valid_hash = hash.starts_with("$2") && hash.len() >= 28;
        if !is_valid_hash {
            return Err(OperationError::InvalidArgument {
                name: "Hash".to_string(),
                reason: "Invalid bcrypt hash format".to_string(),
            });
        }

        // Use bcrypt crate to verify hash
        let match_result = bcrypt::verify(&input_str, hash)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        if match_result {
            Ok(format!("Match: {}", input_str).into_bytes())
        } else {
            Ok("No match".as_bytes().to_vec())
        }
    }
}
