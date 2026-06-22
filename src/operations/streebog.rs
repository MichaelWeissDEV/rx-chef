/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Streebog operation.
 * -----------------------------------------------------------------------------
 */

use digest::Digest;
use streebog::{Streebog256, Streebog512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Streebog operation
///
/// Streebog is a cryptographic hash function defined in the Russian national standard GOST R 34.11-2012
pub struct Streebog;

impl Operation for Streebog {
    fn name(&self) -> &'static str {
        "Streebog"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Streebog is a cryptographic hash function defined in the Russian national standard GOST R 34.11-2012 <i>Information Technology  Cryptographic Information Security  Hash Function</i>. It was created to replace an obsolete GOST hash function defined in the old standard GOST R 34.11-94, and as an asymmetric reply to SHA-3 competition by the US National Institute of Standards and Technology."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Digest length",
            description: "The length of the digest to produce.",
            default_value: "512",
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
        let digest_length = args.first().and_then(|a| a.as_str()).unwrap_or("512");

        let result = match digest_length {
            "256" => hex::encode(Streebog256::digest(&input)),
            "512" => hex::encode(Streebog512::digest(&input)),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Digest length".to_string(),
                    reason: "Invalid digest length for Streebog. Must be 256 or 512.".to_string(),
                })
            }
        };

        Ok(result.into_bytes())
    }
}
