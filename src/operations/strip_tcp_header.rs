/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Strip TCP header operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Strip TCP Header operation
///
/// Strips the TCP header from a TCP segment, outputting the payload.
/// The data offset field at byte 12 (upper 4 bits) specifies the header
/// length in 32-bit words (minimum 5 words = 20 bytes).
pub struct StripTCPHeader;

impl Operation for StripTCPHeader {
    fn name(&self) -> &'static str {
        "Strip TCP header"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Strips the TCP header from a TCP segment, outputting the payload."
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
        const MIN_HEADER_LEN: usize = 20;
        const DATA_OFFSET_BYTE: usize = 12;

        if input.len() < MIN_HEADER_LEN {
            return Err(OperationError::InvalidInput(
                "Need at least 20 bytes for a TCP Header".to_string(),
            ));
        }

        // Data offset is in the upper 4 bits of byte 12, measured in 32-bit words
        let data_offset_words = (input[DATA_OFFSET_BYTE] >> 4) as usize;
        let data_offset_bytes = data_offset_words * 4;

        if input.len() < data_offset_bytes {
            return Err(OperationError::InvalidInput(
                "Input length is less than data offset".to_string(),
            ));
        }

        Ok(input[data_offset_bytes..].to_vec())
    }
}
