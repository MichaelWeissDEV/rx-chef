/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PEM to Hex operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::STANDARD, Engine};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PEM to Hex operation
///
/// Converts PEM (Privacy Enhanced Mail) format to a hexadecimal DER string.
pub struct PEMToHex;

impl Operation for PEMToHex {
    fn name(&self) -> &'static str {
        "PEM to Hex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts PEM (Privacy Enhanced Mail) format to a hexadecimal DER (Distinguished Encoding Rules) string."
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
        let text = String::from_utf8_lossy(&input);
        let mut results: Vec<String> = Vec::new();

        // Find all BEGIN/END blocks
        let mut search_start = 0usize;
        while let Some(begin_pos) = text[search_start..].find("-----BEGIN ") {
            let abs_begin = search_start + begin_pos;
            // Find end of BEGIN line
            let header_end = text[abs_begin..]
                .find("-----\n")
                .or_else(|| text[abs_begin..].find("-----\r\n"))
                .map(|p| abs_begin + p + 5)
                .ok_or_else(|| OperationError::InvalidInput("Malformed PEM header".to_string()))?;

            // Extract label
            let begin_marker = &text[abs_begin..header_end];
            let label = begin_marker
                .trim_start_matches("-----BEGIN ")
                .trim_end_matches("-----")
                .trim();
            let footer = format!("-----END {}-----", label);

            let body_start = header_end + 1; // skip newline
            let footer_pos = text[body_start..]
                .find(&footer)
                .map(|p| body_start + p)
                .ok_or_else(|| {
                    OperationError::InvalidInput(format!("PEM footer '{}' not found", footer))
                })?;

            let b64_body: String = text[body_start..footer_pos]
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            let der = STANDARD.decode(&b64_body).map_err(|e| {
                OperationError::InvalidInput(format!("Base64 decode failed: {}", e))
            })?;

            results.push(hex::encode(&der));
            search_start = footer_pos + footer.len();
        }

        if results.is_empty() {
            return Err(OperationError::InvalidInput(
                "No PEM data found in input".to_string(),
            ));
        }

        Ok(results.join("\n").into_bytes())
    }
}
