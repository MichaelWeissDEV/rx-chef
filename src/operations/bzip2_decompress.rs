/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bzip2 Decompress operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use bzip2::read::BzDecoder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bzip2 Decompress operation
pub struct Bzip2Decompress;

impl Operation for Bzip2Decompress {
    fn name(&self) -> &'static str {
        "Bzip2 Decompress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data using the Bzip2 algorithm."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Use low-memory, slower decompression algorithm",
            description: "Use a slower algorithm that requires less memory",
            default_value: "false",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Err(OperationError::InvalidInput(
                "Please provide an input.".to_string(),
            ));
        }

        let mut decoder = BzDecoder::new(&input[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            OperationError::InvalidInput(format!("Bzip2 decompression failed: {}", e))
        })?;

        Ok(decompressed)
    }
}
