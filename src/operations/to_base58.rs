/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base58 operation.
 * -----------------------------------------------------------------------------
 */

use std::convert::TryInto;

use bs58;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToBase58;

impl Operation for ToBase58 {
    fn name(&self) -> &'static str {
        "To Base58"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Base58 is a notation for encoding arbitrary byte data, removing easily misread characters."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Alphabet",
            description: "The Base58 alphabet",
            default_value: "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
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
        let alphabet = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");
        if alphabet.len() != 58 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: "Must be 58 chars".to_string(),
            });
        }
        let alphabet: &[u8; 58] =
            alphabet
                .as_bytes()
                .try_into()
                .map_err(|_| OperationError::InvalidArgument {
                    name: "Alphabet".to_string(),
                    reason: "Invalid Base58 alphabet".to_string(),
                })?;
        let alphabet =
            bs58::Alphabet::new(alphabet).map_err(|_| OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: "Invalid Base58 alphabet".to_string(),
            })?;
        Ok(bs58::encode(input)
            .with_alphabet(&alphabet)
            .into_string()
            .into_bytes())
    }
}
