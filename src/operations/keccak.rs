/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Keccak operation.
 * -----------------------------------------------------------------------------
 */

use sha3::{Digest, Keccak224, Keccak256, Keccak384, Keccak512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Keccak operation
///
/// The Keccak hash algorithm was designed by Guido Bertoni, Joan Daemen,
/// Michael Peeters, and Gilles Van Assche, building upon RadioGatan.
/// It was selected as the winner of the SHA-3 design competition.
///
/// This version uses Keccak[c=2d] which differs from the final SHA-3
/// specification (different padding).
pub struct Keccak;

impl Operation for Keccak {
    fn name(&self) -> &'static str {
        "Keccak"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "The Keccak hash algorithm was designed by Guido Bertoni, Joan Daemen, Michael Peeters, and Gilles Van Assche, building upon RadioGatan. It was selected as the winner of the SHA-3 design competition. This version of the algorithm is Keccak[c=2d] and differs from the SHA-3 specification."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Size",
            description: "Output size in bits: 512, 384, 256, or 224",
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
        let size_str = args.first().and_then(|a| a.as_str()).unwrap_or("256");
        let size: usize = size_str
            .parse()
            .map_err(|_| OperationError::InvalidArgument {
                name: "Size".to_string(),
                reason: format!("Cannot parse '{}' as a number", size_str),
            })?;

        let digest = match size {
            224 => {
                let mut hasher = Keccak224::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            256 => {
                let mut hasher = Keccak256::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            384 => {
                let mut hasher = Keccak384::new();
                hasher.update(&input);
                hasher.finalize().to_vec()
            }
            512 => {
                let mut hasher = Keccak512::new();
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

        Ok(hex::encode(digest).into_bytes())
    }
}
