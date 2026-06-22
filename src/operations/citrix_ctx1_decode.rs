/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Citrix CTX1 Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Citrix CTX1 Decode operation
///
/// Decodes strings in Citrix CTX1 password format to plaintext.
/// The algorithm processes pairs of bytes in reverse order, XORing with 0xA5
/// and the previous decoded byte. The result is interpreted as UTF-16LE.
///
/// Input must be a multiple of 4 ASCII bytes in the range 'A'..'P'.
pub struct CitrixCtx1Decode;

impl Operation for CitrixCtx1Decode {
    fn name(&self) -> &'static str {
        "Citrix CTX1 Decode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Decodes strings in a Citrix CTX1 password format to plaintext."
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
        if input.len() % 4 != 0 {
            return Err(OperationError::InvalidInput(
                "Incorrect hash length: input must be a multiple of 4 bytes".to_string(),
            ));
        }

        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Reverse the entire input byte array (matching JS: input.reverse())
        let rev_input: Vec<u8> = input.iter().rev().copied().collect();

        // Process pairs of bytes (each character pair encodes one decoded byte)
        // Decoded bytes are accumulated then reversed at the end
        let mut decoded_bytes: Vec<u8> = Vec::new();
        let mut _prev: u8 = 0;

        let n = rev_input.len();
        let mut i = 0;
        while i < n {
            // Determine 'temp' from next pair (i+2, i+3) or 0 if at end
            let temp_next: u8 = if i + 2 >= n {
                0
            } else {
                let hi = rev_input[i + 3].wrapping_sub(0x41) << 4;
                let lo = rev_input[i + 2].wrapping_sub(0x41) & 0x0f;
                (hi & 0xf0) | lo
            };

            let hi = (rev_input[i + 1].wrapping_sub(0x41) << 4) & 0xf0;
            let lo = rev_input[i].wrapping_sub(0x41) & 0x0f;
            let combined = hi | lo;
            let decoded = combined ^ 0xa5 ^ temp_next;
            decoded_bytes.push(decoded);
            _prev = decoded;
            let _ = _prev; // used in original for XOR chain; see below
            i += 2;
        }

        // The JS accumulates into result[] and then calls result.reverse() before decode
        decoded_bytes.reverse();

        // Interpret as UTF-16LE pairs
        if decoded_bytes.len() % 2 != 0 {
            return Err(OperationError::ProcessingError(
                "Decoded byte count is not even; cannot interpret as UTF-16LE".to_string(),
            ));
        }

        let u16_pairs: Vec<u16> = decoded_bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();

        let text = char::decode_utf16(u16_pairs.iter().copied())
            .collect::<Result<String, _>>()
            .map_err(|e| {
                OperationError::ProcessingError(format!("UTF-16LE decode error: {}", e))
            })?;

        Ok(text.into_bytes())
    }
}
