/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the VarInt Encode operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// VarInt Encode operation
pub struct VarIntEncode;

impl Operation for VarIntEncode {
    fn name(&self) -> &'static str {
        "VarInt Encode"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Encodes a Vn integer as a VarInt. VarInt is an efficient way of encoding variable length integers and is commonly used with Protobuf."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let mut value = input_str
            .trim()
            .parse::<BigUint>()
            .map_err(|e| OperationError::InvalidInput(format!("Invalid integer: {}", e)))?;

        let mut result = Vec::new();
        let mask = BigUint::from_u8(0x7f).unwrap();
        let eighty = BigUint::from_u8(0x80).unwrap();

        if value == BigUint::from_u8(0).unwrap() {
            return Ok(vec![0]);
        }

        while value >= eighty {
            let part = (&value & &mask).to_u8().unwrap();
            result.push(part | 0x80);
            value >>= 7;
        }
        result.push(value.to_u8().unwrap());

        Ok(result)
    }
}
