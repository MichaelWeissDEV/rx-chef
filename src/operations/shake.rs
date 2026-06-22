/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SHAKE operation.
 * -----------------------------------------------------------------------------
 */

use sha3::{
    digest::{ExtendableOutput, Update},
    Shake128, Shake256,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SHAKE operation
///
/// SHAKE is an Extendable Output Function (XOF) of the SHA-3 hash algorithm,
/// part of the Keccak family, allowing for variable output length/size.
pub struct SHAKE;

impl Operation for SHAKE {
    fn name(&self) -> &'static str {
        "SHAKE"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "SHAKE is an Extendable Output Function (XOF) of the SHA-3 hash algorithm, part of the Keccak family, allowing for variable output length/size."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Capacity",
                description: "Capacity (256 or 128)",
                default_value: "256",
            },
            ArgSchema {
                name: "Size",
                description: "Output size in bytes (minimum 1)",
                default_value: "512",
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
        let capacity = args.first().and_then(|a| a.as_usize()).unwrap_or(256);
        let size = args.get(1).and_then(|a| a.as_usize()).unwrap_or(512);

        if size == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Size".to_string(),
                reason: "Size must be greater than 0".to_string(),
            });
        }

        let mut output = vec![0u8; size];

        match capacity {
            128 => {
                let mut hasher = Shake128::default();
                hasher.update(&input);
                hasher.finalize_xof_into(output.as_mut_slice());
            }
            256 => {
                let mut hasher = Shake256::default();
                hasher.update(&input);
                hasher.finalize_xof_into(output.as_mut_slice());
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Capacity".to_string(),
                    reason: "Capacity must be 128 or 256".to_string(),
                });
            }
        }

        let output_hex = hex::encode(output);
        Ok(output_hex.into_bytes())
    }
}
