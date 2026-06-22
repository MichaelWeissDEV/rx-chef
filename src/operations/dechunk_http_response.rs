/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Dechunk HTTP response operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Dechunk HTTP response operation
///
/// Parses an HTTP response transferred using Transfer-Encoding: Chunked.
pub struct DechunkHttpResponse;

impl Operation for DechunkHttpResponse {
    fn name(&self) -> &'static str {
        "Dechunk HTTP response"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses an HTTP response transferred using Transfer-Encoding: Chunked."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let chunks = parse_chunks(&input_str)?;
        Ok(chunks.into_bytes())
    }
}

/// Parse chunked HTTP response
fn parse_chunks(input: &str) -> Result<String, OperationError> {
    let mut chunks = Vec::new();
    let mut input = input;

    while !input.is_empty() {
        // Find the end of the chunk size line (first \n)
        let chunk_size_end = input.find('\n').ok_or(OperationError::ProcessingError(
            "Invalid chunked encoding: no newline found".to_string(),
        ))?;

        // Extract chunk size from the beginning to the newline
        // Remove trailing \r if present (CRLF)
        let chunk_size_str = if chunk_size_end > 0 && input.as_bytes()[chunk_size_end - 1] == b'\r'
        {
            &input[..chunk_size_end - 1]
        } else {
            &input[..chunk_size_end]
        };

        let chunk_size = u32::from_str_radix(chunk_size_str.trim(), 16).map_err(|_| {
            OperationError::ProcessingError(format!("Invalid chunk size: {}", chunk_size_str))
        })?;

        if chunk_size == 0 {
            // Final chunk, we're done
            break;
        }

        // Skip the \n after the chunk size
        let after_chunk_size_line =
            if chunk_size_end > 0 && input.as_bytes()[chunk_size_end - 1] == b'\r' {
                chunk_size_end + 1 // \r\n
            } else {
                chunk_size_end + 1 // \n
            };

        input = &input[after_chunk_size_line..];

        // Extract the chunk data
        if input.len() < chunk_size as usize {
            return Err(OperationError::ProcessingError(
                "Invalid chunked encoding: chunk data too short".to_string(),
            ));
        }

        chunks.push(&input[..chunk_size as usize]);
        input = &input[chunk_size as usize..];

        // Skip the \r\n after the chunk data
        if input.len() >= 2 {
            if input.as_bytes()[0] == b'\r' && input.as_bytes()[1] == b'\n' {
                input = &input[2..];
            } else if input.as_bytes()[0] == b'\n' {
                input = &input[1..];
            }
        } else if input.len() == 1 && input.as_bytes()[0] == b'\n' {
            input = "";
        }
    }

    Ok(chunks.join(""))
}
