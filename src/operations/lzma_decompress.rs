/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LZMA Decompress operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use xz2::read::XzDecoder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct LZMADecompress;

impl Operation for LZMADecompress {
    fn name(&self) -> &'static str {
        "LZMA Decompress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data using the Lempel-Ziv-Markov chain Algorithm."
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
        let mut decoder = XzDecoder::new(&input[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to decompress LZMA: {}", e))
        })?;

        Ok(decompressed)
    }
}
