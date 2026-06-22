/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Gunzip operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use flate2::read::GzDecoder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Gunzip operation
///
/// Decompresses data which has been compressed using the deflate algorithm with gzip headers.
pub struct Gunzip;

impl Operation for Gunzip {
    fn name(&self) -> &'static str {
        "Gunzip"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data which has been compressed using the deflate algorithm with gzip headers."
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
        let mut decoder = GzDecoder::new(&input[..]);
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).map_err(|e| {
            OperationError::ProcessingError(format!("Gzip decompression failed: {}", e))
        })?;
        Ok(output)
    }
}
