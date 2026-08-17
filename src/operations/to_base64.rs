/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
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
            kind: crate::operation::ArgKind::String,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }
    fn input_type(&self) -> DataType {
        DataType::Bytes
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }
    /// Verified against upstream CyberChef by the differential harness.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let alphabet_arg = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("A-Za-z0-9+/=");
        let expanded = crate::alphabet::expand_alphabet(alphabet_arg);
        // Upstream accepts 64 or 65 symbols: the 65th is the padding
        // character, and its presence is what decides whether the output is
        // padded. An alphabet such as "A-Za-z0-9-_" has no padding character,
        // so its output must not carry "=" either.
        let padded = expanded.chars().count() == 65;
        let alphabet_str: String = if padded {
            expanded.chars().take(64).collect()
        } else {
            expanded
        };

        if alphabet_str.chars().count() != 64 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: format!(
                    "expected 64 symbols, or 64 plus a padding character; got {}",
                    alphabet_str.chars().count()
                ),
            });
        }
        if padded
            && alphabet_str == "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        {
            return Ok(general_purpose::STANDARD.encode(input).into_bytes());
        }
        let custom_alphabet = alphabet::Alphabet::new(&alphabet_str).map_err(|e| {
            OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: e.to_string(),
            }
        })?;
        let config = if padded {
            general_purpose::PAD
        } else {
            general_purpose::NO_PAD
        };
        let engine = engine::GeneralPurpose::new(&custom_alphabet, config);
        Ok(engine.encode(input).into_bytes())
    }
}
