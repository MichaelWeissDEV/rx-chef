/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SM3 operation.
 * -----------------------------------------------------------------------------
 */

use sm3::{Digest, Sm3};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SM3 operation
///
/// SM3 is a cryptographic hash function used in the Chinese National Standard.
/// SM3 is mainly used in digital signatures, message authentication codes, and
/// pseudorandom number generators. It produces a 256-bit hash value.
pub struct SM3;

impl Operation for SM3 {
    fn name(&self) -> &'static str {
        "SM3"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "SM3 is a cryptographic hash function used in the Chinese National Standard. SM3 is mainly used in digital signatures, message authentication codes, and pseudorandom number generators. The message digest algorithm consists, by default, of 64 rounds and length of 256."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Output format",
            description: "Output format: Hex, Base64, or Raw",
            default_value: "Hex",
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
        let output_format = args.first().and_then(|a| a.as_str()).unwrap_or("Hex");

        let mut hasher = Sm3::new();
        hasher.update(&input);
        let digest = hasher.finalize();
        let digest_bytes = digest.as_slice().to_vec();

        match output_format {
            "Hex" => Ok(hex::encode(&digest_bytes).into_bytes()),
            "Base64" => {
                use base64::Engine;
                Ok(base64::engine::general_purpose::STANDARD
                    .encode(&digest_bytes)
                    .into_bytes())
            }
            "Raw" => Ok(digest_bytes),
            _ => Err(OperationError::InvalidArgument {
                name: "Output format".to_string(),
                reason: format!(
                    "Unknown output format '{}'. Use Hex, Base64, or Raw.",
                    output_format
                ),
            }),
        }
    }
}
