/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Hexdump operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Hexdump operation
pub struct FromHexdump;

impl Operation for FromHexdump {
    fn name(&self) -> &'static str {
        "From Hexdump"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Attempts to convert a hexdump back into raw data. This operation supports many different hexdump variations, but probably not all. Make sure you verify that the data it gives you is correct before continuing analysis."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        // CyberChef regex for hexdump line matching
        // It tries to capture the hex bytes part
        let re = Regex::new(r"(?im)^\s*(?:[\dA-F]{4,16}h?:?)?[ \t]+((?:[\dA-F]{2} ){1,8}(?:[ \t]|[\dA-F]{2}-)(?:[\dA-F]{2} ){1,8}|(?:[\dA-F]{4} )+(?:[\dA-F]{2})?|(?:[\dA-F]{2} )*[\dA-F]{2})")
            .map_err(|e| OperationError::InvalidInput(format!("Invalid regex: {}", e)))?;

        let mut output = Vec::new();
        for cap in re.captures_iter(&input_str) {
            let hex_part = cap[1].replace('-', " ");
            let clean_hex = hex_part.replace(|c: char| !c.is_ascii_hexdigit(), "");

            // Lenient hex decoding (ignore last nibble if odd)
            let mut i = 0;
            let bytes = clean_hex.as_bytes();
            while i + 1 < bytes.len() {
                if let Ok(b) =
                    u8::from_str_radix(std::str::from_utf8(&bytes[i..i + 2]).unwrap(), 16)
                {
                    output.push(b);
                }
                i += 2;
            }
        }

        Ok(output)
    }
}
