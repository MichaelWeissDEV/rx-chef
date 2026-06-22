/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the BSON deserialise operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use bson::Document;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// BSON Deserialise operation
///
/// BSON is a computer data interchange format used mainly as a data storage and
/// network transfer format in the MongoDB database. Input data should be in raw
/// bytes format.
pub struct BsonDeserialise;

impl Operation for BsonDeserialise {
    fn name(&self) -> &'static str {
        "BSON deserialise"
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
        Input data should be in a raw bytes format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
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

        let mut cursor = Cursor::new(input);
        let doc = Document::from_reader(&mut cursor).map_err(|e| {
            OperationError::InvalidInput(format!("Failed to deserialise BSON: {}", e))
        })?;

        let json_value = serde_json::to_value(&doc).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to convert BSON to JSON: {}", e))
        })?;

        let pretty = serde_json::to_string_pretty(&json_value).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to format JSON: {}", e))
        })?;

        Ok(pretty.into_bytes())
    }
}
