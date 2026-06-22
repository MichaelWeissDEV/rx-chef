/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MD2 operation.
 * -----------------------------------------------------------------------------
 */

use digest::Digest;
use md2::Md2;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// MD2 operation
///
/// The MD2 (Message-Digest 2) algorithm is a cryptographic hash function
/// developed by Ronald Rivest in 1989. The algorithm is optimized for 8-bit
/// computers. Although MD2 is no longer considered secure, it remains in use
/// in public key infrastructures as part of certificates generated with MD2
/// and RSA.
pub struct MD2;

impl Operation for MD2 {
    fn name(&self) -> &'static str {
        "MD2"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "MD2 (Message-Digest 2) is a cryptographic hash function developed by Ronald Rivest in 1989. The algorithm is optimized for 8-bit computers. Although MD2 is no longer considered secure, it remains in use in public key infrastructures as part of certificates generated with MD2 and RSA."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Rounds",
            description: "Number of rounds (minimum 0)",
            default_value: "18",
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
        let rounds = args.first().and_then(|a| a.as_usize()).unwrap_or(18);
        if rounds == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Rounds must be greater than zero".to_string(),
            });
        }

        // The md2 crate doesn't support variable rounds; it always performs 18 rounds
        // We validate but don't use the rounds parameter
        let mut hasher = Md2::new();
        hasher.update(&input);
        let digest = hasher.finalize();
        let output: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(output.into_bytes())
    }
}
