/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Quoted Printable operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Quoted Printable operation
pub struct FromQuotedPrintable;

impl Operation for FromQuotedPrintable {
    fn name(&self) -> &'static str {
        "From Quoted Printable"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts QP-encoded text back to standard text. This format is a content transfer encoding common in email messages."
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
        if input.is_empty() {
            return Ok(Vec::new());
        }

        quoted_printable::decode(input, quoted_printable::ParseMode::Robust).map_err(|e| {
            OperationError::InvalidInput(format!("Quoted Printable decode failed: {:?}", e))
        })
    }
}
