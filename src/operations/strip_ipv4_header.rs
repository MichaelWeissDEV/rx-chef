/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Strip IPv4 header operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Strip IPv4 Header operation
///
/// Strips the IPv4 header from an IPv4 packet, outputting the payload.
/// The IPv4 IHL field (bits 3-0 of byte 0) specifies the header length
/// in 32-bit words (minimum 5 words = 20 bytes).
pub struct StripIPv4Header;

impl Operation for StripIPv4Header {
    fn name(&self) -> &'static str {
        "Strip IPv4 header"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Strips the IPv4 header from an IPv4 packet, outputting the payload."
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

        if input.len() < MIN_HEADER_LEN {
            return Err(OperationError::InvalidInput(
                "Input length is less than minimum IPv4 header length".to_string(),
            ));
        }

        // IHL is the lower 4 bits of the first byte; it is measured in 32-bit words
        let ihl = (input[0] & 0x0f) as usize;
        let header_bytes = ihl * 4;

        if input.len() < header_bytes {
            return Err(OperationError::InvalidInput(
                "Input length is less than IHL".to_string(),
            ));
        }

        Ok(input[header_bytes..].to_vec())
    }
}
