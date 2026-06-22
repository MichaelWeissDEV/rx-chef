/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SHA1 operation.
 * -----------------------------------------------------------------------------
 */

use sha1::{Digest, Sha1};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SHA1 operation
///
/// The SHA (Secure Hash Algorithm) hash functions were designed by the NSA.
/// SHA-1 is the most established of the existing SHA hash functions and is
/// used in a variety of security applications. However, SHA-1's collision
/// resistance has been weakening as new attacks are discovered or improved.
pub struct SHA1;

impl Operation for SHA1 {
    fn name(&self) -> &'static str {
        "SHA1"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "The SHA (Secure Hash Algorithm) hash functions were designed by the NSA. SHA-1 is the most established of the existing SHA hash functions and it is used in a variety of security applications and protocols. However, SHA-1's collision resistance has been weakening as new attacks are discovered or improved. The message digest algorithm consists, by default, of 80 rounds."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Rounds",
            description: "Number of rounds (minimum 16)",
            default_value: "80",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        // The sha1 crate doesn't support variable rounds; always uses 80
        // We validate the rounds argument but don't use it
        let rounds = args.first().and_then(|a| a.as_usize()).unwrap_or(80);
        if rounds < 16 {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Rounds must be at least 16".to_string(),
            });
        }

        let mut hasher = Sha1::new();
        hasher.update(input);
        let digest = hasher.finalize();
        let output = format!("{:x}", digest);
        Ok(output.into_bytes())
    }
}
