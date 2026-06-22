/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the BSON serialise operation.
 * -----------------------------------------------------------------------------
 */

use bson::{to_document, Document};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// BSON Serialise operation
///
/// BSON is a computer data interchange format used mainly as a data storage and
/// network transfer format in the MongoDB database. Input data should be valid JSON.
pub struct BsonSerialise;

impl Operation for BsonSerialise {
    fn name(&self) -> &'static str {
        "BSON serialise"
    }

    fn module(&self) -> &'static str {
        "Serialise"
    }

    fn description(&self) -> &'static str {
        "BSON is a computer data interchange format used mainly as a data storage and \
        network transfer format in the MongoDB database. It is a binary form for \
        representing simple data structures, associative arrays (called objects or \
        documents in MongoDB), and various data types of specific interest to MongoDB. \
        The name 'BSON' is based on the term JSON and stands for 'Binary JSON'. \
        Input data should be valid JSON."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = String::from_utf8_lossy(&input);
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let json_value: serde_json::Value = serde_json::from_str(trimmed)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON: {}", e)))?;

        let doc: Document = match &json_value {
            serde_json::Value::Object(_) => to_document(&json_value).map_err(|e| {
                OperationError::ProcessingError(format!(
                    "Failed to convert JSON to BSON document: {}",
                    e
                ))
            })?,
            _ => {
                return Err(OperationError::InvalidInput(
                    "Input JSON must be an object (document) for BSON serialisation".to_string(),
                ));
            }
        };

        let mut buf = Vec::new();
        doc.to_writer(&mut buf).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to serialise BSON: {}", e))
        })?;

        Ok(buf)
    }
}
