/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Modhex operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Modhex operation - converts Modhex encoding (used in YubiKey) to raw bytes.
///
/// Modhex alphabet: cbdefghijklnrtuv (maps to 0-15)
pub struct FromModhex;

const MODHEX_ALPHABET: &[u8] = b"cbdefghijklnrtuv";

impl Operation for FromModhex {
    fn name(&self) -> &'static str {
        "From Modhex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a modhex byte string back into its raw value. Modhex is used in YubiKey."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "Delimiter between modhex pairs (None, Space, Comma, Semi-colon, Colon, Line feed, CRLF)",
            default_value: "None",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("None");

        let clean = match delim_name {
            "None" | "Auto" => input_str.to_lowercase(),
            "Space" => input_str.replace(' ', "").to_lowercase(),
            "Comma" => input_str.replace(',', "").to_lowercase(),
            "Semi-colon" => input_str.replace(';', "").to_lowercase(),
            "Colon" => input_str.replace(':', "").to_lowercase(),
            "Line feed" => input_str.replace('\n', "").to_lowercase(),
            "CRLF" => input_str.replace("\r\n", "").to_lowercase(),
            other => input_str.replace(other, "").to_lowercase(),
        };

        if clean.len() % 2 != 0 {
            return Err(OperationError::InvalidInput(
                "Modhex input length must be even".to_string(),
            ));
        }

        let mut result = Vec::new();
        let chars: Vec<char> = clean.chars().collect();
        let mut i = 0;
        while i + 1 < chars.len() {
            let hi = modhex_val(chars[i]).ok_or_else(|| {
                OperationError::InvalidInput(format!("Invalid modhex character: '{}'", chars[i]))
            })?;
            let lo = modhex_val(chars[i + 1]).ok_or_else(|| {
                OperationError::InvalidInput(format!(
                    "Invalid modhex character: '{}'",
                    chars[i + 1]
                ))
            })?;
            result.push((hi << 4) | lo);
            i += 2;
        }

        Ok(result)
    }
}

fn modhex_val(c: char) -> Option<u8> {
    MODHEX_ALPHABET
        .iter()
        .position(|&b| b == c as u8)
        .map(|i| i as u8)
}
