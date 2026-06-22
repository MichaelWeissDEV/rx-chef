/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Zlib Inflate operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Zlib Inflate operation
///
/// Decompresses zlib-compressed data (RFC 1950).
pub struct ZlibInflate;

impl Operation for ZlibInflate {
    fn name(&self) -> &'static str {
        "Zlib Inflate"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data compressed with the zlib deflate algorithm (RFC 1950)."
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
        let mut decoder = ZlibDecoder::new(&input[..]);
        let mut output = Vec::new();
        decoder
            .read_to_end(&mut output)
            .map_err(|e| OperationError::ProcessingError(format!("Zlib inflate failed: {}", e)))?;
        Ok(output)
    }
}
