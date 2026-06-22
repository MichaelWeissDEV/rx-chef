/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Argon2 compare operation.
 * -----------------------------------------------------------------------------
 */

use argon2::PasswordVerifier;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Argon2 compare operation
///
/// Tests whether the input matches the given Argon2 hash.
pub struct Argon2Compare;

impl Operation for Argon2Compare {
    fn name(&self) -> &'static str {
        "Argon2 compare"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Tests whether the input matches the given Argon2 hash. To test multiple possible passwords, use the 'Fork' operation."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Encoded hash",
            description: "The Argon2 hash to compare against",
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
        let encoded_hash = args.first().and_then(|a| a.as_str()).unwrap_or("");

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // Parse the encoded hash format: $argon2<type>$v=<version>$m=<mem>,t=<iter>,p=<par>$<salt>$<hash>
        if !encoded_hash.starts_with("$argon2") {
            return Ok(b"No match".to_vec());
        }

        // Parse the PHC string
        let parsed_hash = match argon2::password_hash::PasswordHash::new(encoded_hash) {
            Ok(h) => h,
            Err(_) => return Ok(b"No match".to_vec()),
        };

        // Verify the hash
        let hasher = argon2::Argon2::default();
        match hasher.verify_password(input_str.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(format!("Match: {}", input_str).into_bytes()),
            Err(_) => Ok(b"No match".to_vec()),
        }
    }
}
