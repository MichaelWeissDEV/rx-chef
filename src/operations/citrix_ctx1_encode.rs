/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Citrix CTX1 Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Citrix CTX1 Encode operation
///
/// Encodes a plaintext string to Citrix CTX1 password format.
/// The input is converted to UTF-16LE, then each byte is XORed with 0xA5 and
/// the previous encoded byte. The result bytes are split into nibbles and
/// offset by 0x41 to produce printable ASCII output.
pub struct CitrixCtx1Encode;

impl Operation for CitrixCtx1Encode {
    fn name(&self) -> &'static str {
        "Citrix CTX1 Encode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Encodes strings to Citrix CTX1 password format."
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
        let text = String::from_utf8_lossy(&input);

        // Encode the input string as UTF-16LE bytes
        let utf16_bytes: Vec<u8> = text.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();

        let mut result: Vec<u8> = Vec::with_capacity(utf16_bytes.len() * 2);
        let mut prev: u8 = 0;

        for &byte in &utf16_bytes {
            let encoded = byte ^ 0xa5 ^ prev;
            prev = encoded;
            // High nibble -> first output byte; low nibble -> second output byte
            result.push(((encoded >> 4) & 0x0f) + 0x41);
            result.push((encoded & 0x0f) + 0x41);
        }

        Ok(result)
    }
}
