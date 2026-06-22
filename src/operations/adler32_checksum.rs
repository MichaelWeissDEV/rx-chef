/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Adler-32 Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Adler-32 Checksum operation
///
/// Adler-32 is a checksum algorithm which was invented by Mark Adler in 1995,
/// and is a modification of the Fletcher checksum. Compared to a cyclic
/// redundancy check of the same length, it trades reliability for speed
/// (preferring the latter).
pub struct Adler32Checksum;

impl Operation for Adler32Checksum {
    fn name(&self) -> &'static str {
        "Adler-32 Checksum"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Adler-32 is a checksum algorithm which was invented by Mark Adler in 1995, and is a modification of the Fletcher checksum. Compared to a cyclic redundancy check of the same length, it trades reliability for speed (preferring the latter)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        const MOD_ADLER: u32 = 65521;

        let mut a: u32 = 1;
        let mut b: u32 = 0;

        for byte in &input {
            a = (a + *byte as u32) % MOD_ADLER;
            b = (b + a) % MOD_ADLER;
        }

        let result = (b << 16) | a;
        let hex_str = format!("{:08x}", result);

        Ok(hex_str.into_bytes())
    }
}
