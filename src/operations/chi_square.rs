/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Chi Square operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Chi Square operation
///
/// Calculates the Chi Square distribution of values.
pub struct ChiSquare;

impl Operation for ChiSquare {
    fn name(&self) -> &'static str {
        "Chi Square"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates the Chi Square distribution of values."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let data_len = input.len() as f64;
        let mut dist_array = [0u32; 256];

        for &byte in &input {
            dist_array[byte as usize] += 1;
        }

        let expected = data_len / 256.0;
        let mut total = 0.0;

        for &count in &dist_array {
            if count > 0 {
                let diff = count as f64 - expected;
                total += (diff * diff) / expected;
            }
        }

        // `output_type()` is `Number`, which the runtime validates as UTF-8
        // decimal text. Emitting `to_le_bytes()` wrote the raw IEEE-754
        // representation instead, so every run was rejected by the output
        // contract with "declared numeric output but produced invalid UTF-8"
        // and the operation could never return a result.
        Ok(total.to_string().into_bytes())
    }
}
