/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XOR Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// XOR Checksum operation
///
/// XOR Checksum splits the input into blocks of a configurable size and
/// performs the XOR operation on these blocks.
///
/// Ports JS: split into chunks of `blocksize`, XOR positions individually.
/// Positions beyond the chunk end are treated as 0 (JS `chunk[i]` where
/// `i >= chunk.length` gives `undefined`, which coerces to 0 in XOR).
pub struct XORChecksum;

impl Operation for XORChecksum {
    fn name(&self) -> &'static str {
        "XOR Checksum"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "XOR Checksum splits the input into blocks of a configurable size and performs the XOR operation on these blocks."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Blocksize",
            description: "Number of bytes per block",
            default_value: "4",
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
        let blocksize: usize = args
            .get(0)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.first()
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(4);

        if blocksize == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Blocksize".to_string(),
                reason: "Blocksize must be greater than 0".to_string(),
            });
        }

        let mut res = vec![0u8; blocksize];

        for chunk in input.chunks(blocksize) {
            for i in 0..blocksize {
                // JS: res[i] ^= chunk[i]   if i >= chunk.len(), chunk[i] is undefined -> 0
                let chunk_byte = chunk.get(i).copied().unwrap_or(0);
                res[i] ^= chunk_byte;
            }
        }

        Ok(hex::encode(res).into_bytes())
    }
}
