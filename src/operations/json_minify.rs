/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JSON Minify operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JSON Minify operation
pub struct JSONMinify;

impl Operation for JSONMinify {
    fn name(&self) -> &'static str {
        "JSON Minify"
    }

    fn module(&self) -> &'static str {
        "Code"
    }

    fn description(&self) -> &'static str {
        "Compresses JavaScript Object Notation (JSON) code."
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
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let json: Value = serde_json::from_str(&input_str)
            .map_err(|e| OperationError::InvalidInput(format!("Unable to parse JSON: {}", e)))?;

        let minified = serde_json::to_string(&json).map_err(|e| {
            OperationError::ProcessingError(format!("Unable to minify JSON: {}", e))
        })?;

        Ok(minified.into_bytes())
    }
}
