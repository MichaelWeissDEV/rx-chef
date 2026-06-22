/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CBOR Decode operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CBOR Decode operation
pub struct CBORDecode;

impl Operation for CBORDecode {
    fn name(&self) -> &'static str {
        "CBOR Decode"
    }

    fn module(&self) -> &'static str {
        "Serialise"
    }

    fn description(&self) -> &'static str {
        "Concise Binary Object Representation (CBOR) is a binary data serialization format loosely based on JSON. Like JSON it allows the transmission of data objects that contain namevalue pairs, but in a more concise manner. This increases processing and transfer speeds at the cost of human readability. It is defined in IETF RFC 8949."
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

        let value: Value = ciborium::from_reader(&input[..])
            .map_err(|e| OperationError::InvalidInput(format!("CBOR decode failed: {}", e)))?;

        let output = serde_json::to_string_pretty(&value).map_err(|e| {
            OperationError::InvalidInput(format!("JSON serialization failed: {}", e))
        })?;

        Ok(output.into_bytes())
    }
}
