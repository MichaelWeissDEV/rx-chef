/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From MessagePack operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From MessagePack operation
pub struct FromMessagePack;

impl Operation for FromMessagePack {
    fn name(&self) -> &'static str {
        "From MessagePack"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Converts MessagePack encoded data to JSON. MessagePack is a computer data interchange format. It is a binary form for representing simple data structures like arrays and associative arrays."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let value: Value = rmp_serde::from_slice(&input[..]).map_err(|e| {
            OperationError::InvalidInput(format!("MessagePack decode failed: {}", e))
        })?;

        let output = serde_json::to_string_pretty(&value).map_err(|e| {
            OperationError::InvalidInput(format!("JSON serialization failed: {}", e))
        })?;

        Ok(output.into_bytes())
    }
}
