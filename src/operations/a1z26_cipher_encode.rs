/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the A1Z26 Cipher Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// A1Z26 Cipher Encode operation
///
/// Converts alphabet characters into their corresponding alphabet order number.
/// e.g. `a` becomes `1` and `b` becomes `2`.
/// Non-alphabet characters are dropped.
pub struct A1Z26CipherEncode;

impl Operation for A1Z26CipherEncode {
    fn name(&self) -> &'static str {
        "A1Z26 Cipher Encode"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Converts alphabet characters into their corresponding alphabet order number.<br><br>e.g. <code>a</code> becomes <code>1</code> and <code>b</code> becomes <code>2</code>.<br><br>Non-alphabet characters are dropped."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "Delimiter between numbers",
            default_value: "Space",
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

        let delim_str = if !args.is_empty() {
            args[0].as_str().unwrap_or("Space")
        } else {
            "Space"
        };
        let delim = parse_delimiter(delim_str);

        let mut output = String::new();

        for c in input_str.to_lowercase().chars() {
            if c.is_ascii_alphabetic() {
                let ordinal = (c as u8 - b'a') + 1;
                if ordinal > 0 && ordinal <= 26 {
                    if !output.is_empty() {
                        output.push_str(delim);
                    }
                    output.push_str(&ordinal.to_string());
                }
            }
        }

        Ok(output.into_bytes())
    }
}

/// Parse delimiter string to actual character (matching CyberChef's charRep)
fn parse_delimiter(delim: &str) -> &'static str {
    match delim {
        "Space" => " ",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "Line feed" => "\n",
        "CRLF" => "\r\n",
        _ => " ",
    }
}
