/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bit shift left operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bit shift left operation
///
/// Shifts the bits in each byte towards the left by the specified amount.
pub struct BitShiftLeft;

impl Operation for BitShiftLeft {
    fn name(&self) -> &'static str {
        "Bit shift left"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Shifts the bits in each byte towards the left by the specified amount."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Amount",
            description: "Amount to shift left",
            default_value: "1",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let amount = if args.len() > 0 {
            args[0].as_f64().unwrap_or(1.0) as u8
        } else {
            1
        };

        let result: Vec<u8> = input.iter().map(|b| ((b << amount) & 0xff) as u8).collect();

        Ok(result)
    }
}
