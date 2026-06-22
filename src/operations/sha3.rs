/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SHA3 operation.
 * -----------------------------------------------------------------------------
 */

use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SHA3 operation
///
/// The SHA-3 (Secure Hash Algorithm 3) hash functions were released by NIST
/// on August 5, 2015. Although part of the same series of standards, SHA-3 is
/// internally quite different from the MD5-like structure of SHA-1 and SHA-2.
/// SHA-3 is a subset of the broader cryptographic primitive family Keccak
/// designed by Guido Bertoni, Joan Daemen, Michal Peeters, and Gilles Van Assche,
/// building upon RadioGatn.
pub struct SHA3;

impl Operation for SHA3 {
    fn name(&self) -> &'static str {
        "SHA3"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "SHA-3 (Secure Hash Algorithm 3) is a cryptographic hash function standard released by NIST on August 5, 2015. SHA-3 is internally quite different from the MD5-like structure of SHA-1 and SHA-2, and is a subset of the Keccak family designed by Guido Bertoni, Joan Daemen, Michal Peeters, and Gilles Van Assche."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Size",
            description: "Output size in bits (512, 384, 256, or 224)",
            default_value: "256",
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
        let size = args.first().and_then(|a| a.as_usize()).unwrap_or(256);

        let digest = match size {
            224 => {
                let mut hasher = Sha3_224::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            256 => {
                let mut hasher = Sha3_256::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            384 => {
                let mut hasher = Sha3_384::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            512 => {
                let mut hasher = Sha3_512::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Size".to_string(),
                    reason: "Size must be 512, 384, 256, or 224".to_string(),
                });
            }
        };

        let output = hex::encode(digest);
        Ok(output.into_bytes())
    }
}
