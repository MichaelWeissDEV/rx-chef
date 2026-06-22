/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From HTML Entity operation.
 * -----------------------------------------------------------------------------
 */

use html_escape::decode_html_entities;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From HTML Entity operation
pub struct FromHTMLEntity;

impl Operation for FromHTMLEntity {
    fn name(&self) -> &'static str {
        "From HTML Entity"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Converts HTML entities back to characters."
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let decoded = decode_html_entities(&input_str);
        Ok(decoded.into_owned().into_bytes())
    }
}
