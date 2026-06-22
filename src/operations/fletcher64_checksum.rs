/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Fletcher-64 Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Fletcher-64 Checksum operation
pub struct Fletcher64Checksum;

impl Operation for Fletcher64Checksum {
    fn name(&self) -> &'static str {
        "Fletcher-64 Checksum"
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
        let mut a: u64 = 0;
        let mut b: u64 = 0;

        for chunk in input.chunks_exact(4) {
            let val = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as u64;
            a = (a + val) % 0xffffffff;
            b = (b + a) % 0xffffffff;
        }

        let rem = input.len() % 4;
        if rem != 0 {
            let mut last_value: u32 = 0;
            for i in 0..rem {
                last_value = (last_value << 8) | input[input.len() - 1 - i] as u32;
            }
            a = (a + last_value as u64) % 0xffffffff;
            b = (b + a) % 0xffffffff;
        }

        Ok(format!("{:08x}{:08x}", b as u32, a as u32).into_bytes())
    }
}
