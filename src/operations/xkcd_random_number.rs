/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XKCD Random Number operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// XKCD Random Number operation
pub struct XkcdRandomNumberOp;

impl Operation for XkcdRandomNumberOp {
    fn name(&self) -> &'static str {
        "XKCD Random Number"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "RFC 1149.5 specifies 4 as the standard IEEE-vetted random number."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, _input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        Ok("4".to_string().into_bytes())
    }
}
