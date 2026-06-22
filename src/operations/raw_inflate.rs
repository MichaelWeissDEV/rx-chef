/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Raw Inflate operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use flate2::read::DeflateDecoder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Raw Inflate operation
///
/// Decompresses raw deflate-compressed data (no headers, RFC 1951).
pub struct RawInflate;

impl Operation for RawInflate {
    fn name(&self) -> &'static str {
        "Raw Inflate"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data compressed with the raw deflate algorithm (no headers, RFC 1951)."
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
        let mut decoder = DeflateDecoder::new(&input[..]);
        let mut output = Vec::new();
        decoder
            .read_to_end(&mut output)
            .map_err(|e| OperationError::ProcessingError(format!("Raw inflate failed: {}", e)))?;
        Ok(output)
    }
}
