/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LZ4 Decompress operation.
 * -----------------------------------------------------------------------------
 */

use lz4_flex::decompress_size_prepended;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct LZ4Decompress;

impl Operation for LZ4Decompress {
    fn name(&self) -> &'static str {
        "LZ4 Decompress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "LZ4 is a lossless data compression algorithm that is focused on compression and decompression speed. It belongs to the LZ77 family of byte-oriented compression schemes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        decompress_size_prepended(&input).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to decompress LZ4: {}", e))
        })
    }
}
