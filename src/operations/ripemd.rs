/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RIPEMD operation.
 * -----------------------------------------------------------------------------
 */

use ripemd::{Digest, Ripemd128, Ripemd160, Ripemd256, Ripemd320};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RIPEMD operation
///
/// RIPEMD (RACE Integrity Primitives Evaluation Message Digest) is a family
/// of cryptographic hash functions developed in Leuven, Belgium. RIPEMD was
/// based upon the design principles used in MD4, and is similar in performance
/// to the more popular SHA-1.
pub struct RIPEMD;

impl Operation for RIPEMD {
    fn name(&self) -> &'static str {
        "RIPEMD"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "RIPEMD (RACE Integrity Primitives Evaluation Message Digest) is a family of cryptographic hash functions developed in Leuven, Belgium. RIPEMD was based upon the design principles used in MD4, and is similar in performance to the more popular SHA-1."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Size",
            description: "Output size in bits (320, 256, 160, or 128)",
            default_value: "160",
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
        let size = args.first().and_then(|a| a.as_usize()).unwrap_or(160);

        let digest = match size {
            128 => {
                let mut hasher = Ripemd128::default();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            160 => {
                let mut hasher = Ripemd160::default();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            256 => {
                let mut hasher = Ripemd256::default();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            320 => {
                let mut hasher = Ripemd320::default();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Size".to_string(),
                    reason: "Size must be 320, 256, 160, or 128".to_string(),
                });
            }
        };

        let output = hex::encode(digest);
        Ok(output.into_bytes())
    }
}
