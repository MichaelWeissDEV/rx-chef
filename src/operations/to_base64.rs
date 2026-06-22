/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base64 operation.
 * -----------------------------------------------------------------------------
 */

use base64::{alphabet, engine, engine::general_purpose, Engine as _};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToBase64;

impl Operation for ToBase64 {
    fn name(&self) -> &'static str {
        "To Base64"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Alphabet",
            description: "The Base64 alphabet",
            default_value: "A-Za-z0-9+/=",
        }];
        SCHEMA
    }
    fn input_type(&self) -> DataType {
        DataType::Bytes
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }
    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let alphabet_arg = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("A-Za-z0-9+/=");
        let alphabet_str = expand_alphabet(alphabet_arg);

        if alphabet_str == "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" {
            return Ok(general_purpose::STANDARD.encode(input).into_bytes());
        }
        if alphabet_str.len() != 64 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: "Must be 64 chars".to_string(),
            });
        }
        let custom_alphabet = alphabet::Alphabet::new(&alphabet_str).map_err(|e| {
            OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: e.to_string(),
            }
        })?;
        let engine = engine::GeneralPurpose::new(&custom_alphabet, general_purpose::PAD);
        Ok(engine.encode(input).into_bytes())
    }
}

fn expand_alphabet(alphabet: &str) -> String {
    if alphabet == "A-Za-z0-9+/=" {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string()
    } else {
        alphabet.replace(
            "A-Za-z0-9+/",
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
        )
    }
}
