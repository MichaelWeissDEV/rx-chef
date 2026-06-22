/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Strip HTTP headers operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Strip HTTP Headers operation
///
/// Removes HTTP headers from a request or response by looking for the first
/// instance of a double newline (CRLF-CRLF or LF-LF).
pub struct StripHTTPHeaders;

impl Operation for StripHTTPHeaders {
    fn name(&self) -> &'static str {
        "Strip HTTP headers"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Removes HTTP headers from a request or response by looking for the first instance of a double newline."
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
        let text = String::from_utf8_lossy(&input).into_owned();

        // Try CRLF-CRLF first, then LF-LF
        let header_end = if let Some(pos) = text.find("\r\n\r\n") {
            pos + 4
        } else if let Some(pos) = text.find("\n\n") {
            pos + 2
        } else {
            // No double newline found; return the whole input
            return Ok(input);
        };

        Ok(text[header_end..].as_bytes().to_vec())
    }
}
