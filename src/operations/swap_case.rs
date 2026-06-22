/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Swap case operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Swap case operation
///
/// Converts uppercase letters to lowercase ones, and lowercase ones to
/// uppercase ones.
pub struct SwapCase;

impl Operation for SwapCase {
    fn name(&self) -> &'static str {
        "Swap case"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts uppercase letters to lowercase ones, and lowercase ones to uppercase ones."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let mut result = String::new();

        for c in input_str.chars() {
            if c.is_uppercase() {
                result.push(c.to_lowercase().next().unwrap_or(c));
            } else if c.is_lowercase() {
                result.push(c.to_uppercase().next().unwrap_or(c));
            } else {
                result.push(c);
            }
        }

        Ok(result.into_bytes())
    }
}
