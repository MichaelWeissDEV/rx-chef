/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bcrypt parse operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bcrypt parse operation
///
/// Parses a bcrypt hash to determine the number of rounds used, the salt, and the password hash.
pub struct BcryptParse;

impl Operation for BcryptParse {
    fn name(&self) -> &'static str {
        "Bcrypt parse"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Parses a bcrypt hash to determine the number of rounds used, the salt, and the password hash."
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

        // Bcrypt hash format: $2a$rounds$salt+hash
        // Example: $2a$10$somesalt1234567890abcdef1234567890abcdef1234567890ab

        let hash = input_str.trim();
        if !hash.starts_with('$') {
            return Err(OperationError::InvalidInput(
                "Invalid bcrypt hash format".to_string(),
            ));
        }

        // Parse the hash parts
        let parts: Vec<&str> = hash.split('$').collect();
        if parts.len() < 4 {
            return Err(OperationError::InvalidInput(
                "Invalid bcrypt hash format".to_string(),
            ));
        }

        // parts[1] = version (2a, 2b, 2y)
        // parts[2] = rounds
        // parts[3] = salt + hash (combined)

        let version = parts[1];
        let rounds_str = parts[2];
        let salt_and_hash = parts[3];

        // Extract salt (first 22 characters after $2a$rounds$)
        let salt = salt_and_hash.chars().take(22).collect::<String>();

        // Extract hash (remaining characters)
        let password_hash = salt_and_hash.chars().skip(22).collect::<String>();

        let output = format!(
            "Version: {}\nRounds: {}\nSalt: ${}\nPassword hash: ${}\nFull hash: {}",
            version, rounds_str, salt, password_hash, hash
        );

        Ok(output.into_bytes())
    }
}
