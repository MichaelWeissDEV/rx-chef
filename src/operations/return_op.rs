/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Return operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Return operation
pub struct ReturnOp;

impl Operation for ReturnOp {
    fn name(&self) -> &'static str {
        "Return"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "End execution of operations at this point in the recipe."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        // In this implementation, Return just passes the input through.
        // The actual flow control logic must be handled by the recipe runner.
        Ok(input)
    }
}
