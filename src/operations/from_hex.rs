/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Hex operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Hex operation
pub struct FromHex;

impl Operation for FromHex {
    fn name(&self) -> &'static str {
        "From Hex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a hexadecimal byte string back into its raw value."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "The hexadecimal delimiter",
            default_value: "Auto",
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
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let delim = args.first().and_then(|v| v.as_str()).unwrap_or("Auto");

        let clean_input = if delim == "Auto" {
            // Very basic auto-detection: remove everything that isn't hex or 0x/ \x
            input_str
                .replace("0x", "")
                .replace("\\x", "")
                .replace(|c: char| !c.is_ascii_hexdigit(), "")
        } else {
            match delim {
                "Space" => input_str.replace(' ', ""),
                "Comma" => input_str.replace(',', ""),
                "Semi-colon" => input_str.replace(';', ""),
                "Colon" => input_str.replace(':', ""),
                "Line feed" => input_str.replace('\n', ""),
                "CRLF" => input_str.replace("\r\n", ""),
                "0x" => input_str.replace("0x", ""),
                "0x with comma" => input_str.replace("0x", "").replace(',', ""),
                "\\x" => input_str.replace("\\x", ""),
                "None" => input_str,
                _ => input_str.replace(delim, ""),
            }
        };

        // If the string has an odd length, it's invalid hex
        if clean_input.len() % 2 != 0 {
            // In CyberChef, fromHex is quite lenient. Let's try to pad with 0 if needed?
            // Actually, CyberChef just ignores the last nibble if it's incomplete.
            let mut result = Vec::new();
            let bytes = clean_input.as_bytes();
            for i in (0..bytes.len() - 1).step_by(2) {
                let hex_str = std::str::from_utf8(&bytes[i..i + 2]).map_err(|e| {
                    OperationError::InvalidInput(format!("invalid UTF-8 in hex data: {}", e))
                })?;
                if let Ok(b) = u8::from_str_radix(hex_str, 16) {
                    result.push(b);
                }
            }
            return Ok(result);
        }

        hex::decode(clean_input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))
    }
}
