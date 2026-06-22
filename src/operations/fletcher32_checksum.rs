/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Fletcher-32 Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Fletcher-32 Checksum operation
pub struct Fletcher32Checksum;

impl Operation for Fletcher32Checksum {
    fn name(&self) -> &'static str {
        "Fletcher-32 Checksum"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "The Fletcher checksum is an algorithm for computing a position-dependent checksum devised by John Gould Fletcher at Lawrence Livermore Labs in the late 1970s.<br><br>The objective of the Fletcher checksum was to provide error-detection properties approaching those of a cyclic redundancy check but with the lower computational effort associated with summation techniques."
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
        let mut a: u32 = 0;
        let mut b: u32 = 0;

        for chunk in input.chunks_exact(2) {
            let val = u16::from_le_bytes([chunk[0], chunk[1]]) as u32;
            a = (a + val) % 0xffff;
            b = (b + a) % 0xffff;
        }

        if input.len() % 2 != 0 {
            let val = input[input.len() - 1] as u32;
            a = (a + val) % 0xffff;
            b = (b + a) % 0xffff;
        }

        let checksum = ((b << 16) | a) as u32;
        Ok(format!("{:08x}", checksum).into_bytes())
    }
}
