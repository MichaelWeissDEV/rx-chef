/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the YAML to JSON operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// YAML to JSON operation
pub struct YAMLToJSON;

impl Operation for YAMLToJSON {
    fn name(&self) -> &'static str {
        "YAML to JSON"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Convert YAML to JSON"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        if input_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let yaml: Value = serde_yaml::from_str(&input_str)
            .map_err(|e| OperationError::InvalidInput(format!("Unable to parse YAML: {}", e)))?;

        let json_str = serde_json::to_string_pretty(&yaml).map_err(|e| {
            OperationError::ProcessingError(format!("Unable to format JSON: {}", e))
        })?;

        Ok(json_str.into_bytes())
    }
}
