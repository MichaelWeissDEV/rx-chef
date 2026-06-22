/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bcrypt operation.
 * -----------------------------------------------------------------------------
 */

use bcrypt::{hash, DEFAULT_COST};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bcrypt operation
///
/// bcrypt is a password hashing function designed by Niels Provos and David Mazires,
/// based on the Blowfish cipher, and presented at USENIX in 1999. Besides incorporating
/// a salt to protect against rainbow table attacks, bcrypt is an adaptive function:
/// over time, the iteration count (rounds) can be increased to make it slower.
pub struct Bcrypt;

impl Operation for Bcrypt {
    fn name(&self) -> &'static str {
        "Bcrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "bcrypt is a password hashing function designed by Niels Provos and David Mazires, based on the Blowfish cipher, and presented at USENIX in 1999. Besides incorporating a salt to protect against rainbow table attacks, bcrypt is an adaptive function: over time, the iteration count (rounds) can be increased to make it slower."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Rounds",
            description: "Number of rounds (10-31, default 10)",
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
        let rounds = args
            .get(0)
            .and_then(|a| a.as_f64())
            .map(|r| r as u32)
            .unwrap_or(DEFAULT_COST);

        // Validate rounds (must be between 4 and 31)
        if rounds < 4 || rounds > 31 {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Rounds must be between 4 and 31".to_string(),
            });
        }

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let hashed =
            hash(input_str, rounds).map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        Ok(hashed.into_bytes())
    }
}
