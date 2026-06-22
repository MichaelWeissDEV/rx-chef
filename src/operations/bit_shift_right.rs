/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bit shift right operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bit shift right operation
///
/// Shifts the bits in each byte towards the right by the specified amount.
pub struct BitShiftRight;

impl Operation for BitShiftRight {
    fn name(&self) -> &'static str {
        "Bit shift right"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Shifts the bits in each byte towards the right by the specified amount."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Amount",
                description: "Amount to shift right",
                default_value: "1",
            },
            ArgSchema {
                name: "Type",
                description: "Shift type (Logical or Arithmetic)",
                default_value: "Logical shift",
            },
        ];
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

        // "Logical shift" uses 0 mask, "Arithmetic shift" uses 0x80 to preserve MSB
        let shift_type = if args.len() > 1 {
            args[1].as_str().unwrap_or("Logical shift")
        } else {
            "Logical shift"
        };
        let mask = if shift_type == "Arithmetic shift" {
            0x80
        } else {
            0
        };

        let result: Vec<u8> = input
            .iter()
            .map(|b| {
                let logical_shift = *b >> amount;
                // Preserve the MSB if it was set and we're doing arithmetic shift
                (logical_shift ^ (*b & mask)) as u8
            })
            .collect();

        Ok(result)
    }
}
