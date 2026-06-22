/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract ID3 operation.
 * -----------------------------------------------------------------------------
 */

use lofty::prelude::*;
use serde_json::json;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract ID3 operation
pub struct ExtractID3;

impl Operation for ExtractID3 {
    fn name(&self) -> &'static str {
        "Extract ID3"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "This operation extracts ID3 metadata from an MP3 file."
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
        let mut reader = std::io::Cursor::new(&input);
        let tagged_file = lofty::probe::Probe::new(&mut reader)
            .guess_file_type()
            .map_err(|e| {
                OperationError::ProcessingError(format!("Failed to guess file type: {}", e))
            })?
            .read()
            .map_err(|e| {
                OperationError::ProcessingError(format!("Failed to read metadata: {}", e))
            })?;

        let mut tags_json = serde_json::Map::new();

        if let Some(tag) = tagged_file.primary_tag() {
            for item in tag.items() {
                let key = format!("{:?}", item.key());
                let value = item.value().text().unwrap_or("").to_string();
                tags_json.insert(
                    key.clone(),
                    json!({
                        "Description": key,
                        "Data": value
                    }),
                );
            }
        }

        let result = json!({
            "Type": "ID3",
            "Tags": tags_json
        });

        serde_json::to_vec_pretty(&result).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to serialize JSON: {}", e))
        })
    }
}
