/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Whirlpool operation.
 * -----------------------------------------------------------------------------
 */

use whirlpool::{Digest, Whirlpool as WhirlpoolStd};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Whirlpool operation
///
/// Whirlpool is a cryptographic hash function designed by Vincent Rijmen
/// (co-creator of AES) and Paulo S. L. M. Barreto, who first described it in 2000.
///
/// Whirlpool is the latest revision, released in 2003, fixing a flaw in the diffusion matrix.
pub struct WHIRLPOOL;

impl Operation for WHIRLPOOL {
    fn name(&self) -> &'static str {
        "Whirlpool"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Whirlpool is a cryptographic hash function designed by Vincent Rijmen (co-creator of AES) and Paulo S. L. M. Barreto, who first described it in 2000. Whirlpool is the latest revision, released in 2003, fixing a flaw in the diffusion matrix."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Variant",
                description: "Hash variant (Whirlpool only)",
                default_value: "Whirlpool",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds (1-10)",
                default_value: "10",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let variant = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("whirlpool")
            .to_lowercase();
        let rounds = args.get(1).and_then(|a| a.as_usize()).unwrap_or(10);

        if rounds < 1 || rounds > 10 {
            return Err(OperationError::InvalidArgument {
                name: "Rounds".to_string(),
                reason: "Rounds must be between 1 and 10".to_string(),
            });
        }

        // The whirlpool crate only provides the standard Whirlpool variant
        // We validate the variant but always use WhirlpoolStd
        if variant != "whirlpool" {
            return Err(OperationError::InvalidArgument {
                name: "Variant".to_string(),
                reason: "Variant must be Whirlpool".to_string(),
            });
        }

        let mut hasher = WhirlpoolStd::new();
        hasher.update(input);
        let digest = hasher.finalize();
        let output = hex::encode(digest);
        Ok(output.into_bytes())
    }
}
