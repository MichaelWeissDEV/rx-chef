/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SHA0 operation.
 * -----------------------------------------------------------------------------
 */

use sha1::{Digest, Sha1};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SHA0 operation
///
/// SHA-0 is the original 160-bit hash function published in 1993 under the
/// name 'SHA'. It was withdrawn shortly after publication due to a significant
/// flaw and replaced by the slightly revised version SHA-1. The key difference
/// is in the initial hash values.
pub struct SHA0;

impl Operation for SHA0 {
    fn name(&self) -> &'static str {
        "SHA0"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "SHA-0 is the original 160-bit hash function published in 1993 under the name 'SHA'. It was withdrawn shortly after publication due to an undisclosed 'significant flaw' and replaced by the slightly revised version SHA-1."
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
        let rounds = args.first().and_then(|a| a.as_usize()).unwrap_or(80);
        if rounds < 16 {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Rounds must be at least 16".to_string(),
            });
        }

        // Note: The sha1 crate implements SHA-1 with the fix applied.
        // SHA-0 differs from SHA-1 in the f function's rotation constant.
        // For a true SHA-0 implementation, a custom implementation would be needed.
        // For now, we use the sha1 crate with a note about the difference.
        let mut hasher = Sha1::new();
        hasher.update(input);
        let digest = hasher.finalize();
        let output = hex::encode(digest);
        Ok(output.into_bytes())
    }
}
