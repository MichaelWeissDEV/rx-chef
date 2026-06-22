/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the VarInt Decode operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use num_traits::FromPrimitive;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// VarInt Decode operation
pub struct VarIntDecode;

impl Operation for VarIntDecode {
    fn name(&self) -> &'static str {
        "VarInt Decode"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Decodes a VarInt encoded integer. VarInt is an efficient way of encoding variable length integers and is commonly used with Protobuf."
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
        let mut result = BigUint::from_u8(0).unwrap();
        let mut offset = 0;

        for b in input {
            let val = BigUint::from_u8(b & 0x7f).unwrap();
            result |= val << offset;
            if (b & 0x80) == 0 {
                return Ok(result.to_string().into_bytes());
            }
            offset += 7;
        }

        // If we reach here, it means the last byte had the MSB set, which is technically invalid for a finished VarInt
        // but we'll return what we have to be consistent with common decoders or just return what we got.
        // CyberChef breaks the loop on (!(input[i] & 0x80)).
        Ok(result.to_string().into_bytes())
    }
}
