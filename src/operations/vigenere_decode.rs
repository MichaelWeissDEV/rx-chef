/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Vigenre Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Vigenre Decode operation
pub struct VigenereDecodeOp;

impl Operation for VigenereDecodeOp {
    fn name(&self) -> &'static str {
        "Vigenre Decode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "The Vigenere cipher is a method of encrypting alphabetic text by using a series of different Caesar ciphers based on the letters of a keyword. It is a simple form of polyalphabetic substitution."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Key",
            description: "Key for the Vigenre cipher (letters only)",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let key = args.first().and_then(|a| a.as_str()).unwrap_or("");

        if key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "No key entered".to_string(),
            });
        }

        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "The key must consist only of letters".to_string(),
            });
        }

        let key = key.to_lowercase();
        let key_bytes = key.as_bytes();
        let mut output = String::with_capacity(input_str.len());
        let mut fail = 0;

        for (i, c) in input_str.chars().enumerate() {
            if c.is_ascii_lowercase() {
                let key_char = key_bytes[(i - fail) % key_bytes.len()];
                let key_index = key_char - b'a';
                let msg_index = (c as u8) - b'a';
                let decoded_index = (msg_index + 26 - key_index) % 26;
                output.push((b'a' + decoded_index) as char);
            } else if c.is_ascii_uppercase() {
                let key_char = key_bytes[(i - fail) % key_bytes.len()];
                let key_index = key_char - b'a';
                let msg_index = (c.to_ascii_lowercase() as u8) - b'a';
                let decoded_index = (msg_index + 26 - key_index) % 26;
                output.push(((b'a' + decoded_index) as char).to_ascii_uppercase());
            } else {
                output.push(c);
                fail += 1;
            }
        }

        Ok(output.into_bytes())
    }
}
