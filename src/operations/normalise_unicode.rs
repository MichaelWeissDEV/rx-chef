/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Normalise Unicode operation.
 * -----------------------------------------------------------------------------
 */

use unicode_normalization::UnicodeNormalization;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Normalise Unicode operation
pub struct NormaliseUnicode;

impl Operation for NormaliseUnicode {
    fn name(&self) -> &'static str {
        "Normalise Unicode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Transform Unicode characters to one of the Normalisation Forms"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Normal Form",
            description: "Unicode Normalisation Form",
            default_value: "NFD",
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
        let normal_form = if args.is_empty() {
            "NFD"
        } else {
            args[0].as_str().unwrap_or("NFD")
        };

        let result = match normal_form {
            "NFD" => input_str.nfd().collect::<String>(),
            "NFC" => input_str.nfc().collect::<String>(),
            "NFKD" => input_str.nfkd().collect::<String>(),
            "NFKC" => input_str.nfkc().collect::<String>(),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Normal Form".to_string(),
                    reason: "Unknown Normalisation Form".to_string(),
                })
            }
        };

        Ok(result.into_bytes())
    }
}
