/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
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
            kind: crate::operation::ArgKind::Integer,
            required: false,
            choices: &[],
            minimum: None,
            maximum: Some(crate::operation::NumericBound::Integer(7)),
            sensitive: false,
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    /// Matches upstream CyberChef byte for byte on the recorded
    /// differential case.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let amount = if !args.is_empty() {
            args[0].as_f64().unwrap_or(1.0) as u32
        } else {
            1
        };

        if amount > 7 {
            return Err(OperationError::InvalidArgument {
                name: "Amount".to_string(),
                reason: "Amount must be between 0 and 7".to_string(),
            });
        }

        let result: Vec<u8> = input
            .iter()
            .map(|byte| ((u32::from(*byte) << amount) & 0xff) as u8)
            .collect();

        Ok(result)
    }
}
