/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the TCP/IP Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// TCP/IP Checksum operation
pub struct TCPIPChecksum;

impl Operation for TCPIPChecksum {
    fn name(&self) -> &'static str {
        "TCP/IP Checksum"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Calculates the checksum for a TCP (Transport Control Protocol) or IP (Internet Protocol) header from an input of raw bytes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut csum: u32 = 0;

        for i in 0..input.len() {
            if i % 2 == 0 {
                csum += (input[i] as u32) << 8;
            } else {
                csum += input[i] as u32;
            }
        }

        while (csum >> 16) > 0 {
            csum = (csum >> 16) + (csum & 0xffff);
        }

        let result = format!("{:04x}", 0xffff - (csum as u16));

        Ok(result.into_bytes())
    }
}
