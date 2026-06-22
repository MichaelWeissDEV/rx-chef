/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Hex Content operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Hex Content operation - decodes hex content embedded in text using SNORT pipe notation.
///
/// e.g. `foo|3d|bar` becomes `foo=bar`
pub struct FromHexContent;

impl Operation for FromHexContent {
    fn name(&self) -> &'static str {
        "From Hex Content"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Translates hexadecimal bytes in text back to raw bytes. e.g. 'foo|3d|bar' becomes 'foo=bar'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let re = Regex::new(r"(?i)\|([a-f0-9 ]{2,})\|")
            .map_err(|e| OperationError::ProcessingError(format!("Regex error: {}", e)))?;

        let mut output: Vec<u8> = Vec::new();
        let mut last_end = 0;

        for cap in re.captures_iter(&input_str) {
            let full_match = cap.get(0).unwrap();
            let hex_group = cap.get(1).unwrap();

            // Add text before this match as raw bytes
            let before = &input_str[last_end..full_match.start()];
            output.extend_from_slice(before.as_bytes());

            // Decode hex, stripping spaces
            let hex_str: String = hex_group
                .as_str()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            match hex::decode(&hex_str) {
                Ok(bytes) => output.extend(bytes),
                Err(_) => {
                    // Not valid hex: output the original pipe-delimited text as-is
                    output.extend_from_slice(full_match.as_str().as_bytes());
                }
            }

            last_end = full_match.end();
        }

        // Add remaining text
        output.extend_from_slice(input_str[last_end..].as_bytes());

        Ok(output)
    }
}
