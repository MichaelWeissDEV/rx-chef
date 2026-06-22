/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Strip UDP header operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Strip UDP Header operation
///
/// Strips the UDP header from a UDP datagram, outputting the payload.
pub struct StripUDPHeader;

impl Operation for StripUDPHeader {
    fn name(&self) -> &'static str {
        "Strip UDP header"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Strips the UDP header from a UDP datagram, outputting the payload."
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
        const HEADER_LEN: usize = 8;

        if input.len() < HEADER_LEN {
            return Err(OperationError::InvalidArgument {
                name: "input".to_string(),
                reason: "Need 8 bytes for a UDP Header".to_string(),
            });
        }

        Ok(input[HEADER_LEN..].to_vec())
    }
}
