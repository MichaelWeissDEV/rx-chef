/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Charcode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Charcode operation
///
/// Converts text to its unicode character code equivalent.
/// e.g. ` ` becomes `0393 03b5 03b9 03ac 20 03c3 03bf 03c5`
pub struct ToCharcode;

fn char_rep(name: &str) -> &str {
    match name {
        "Space" => " ",
        "Comma" => ",",
        "Semi-colon" => ";",
        "Colon" => ":",
        "Line feed" => "\n",
        "CRLF" => "\r\n",
        "None" => "",
        other => other,
    }
}

impl Operation for ToCharcode {
    fn name(&self) -> &'static str {
        "To Charcode"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts text to its unicode character code equivalent."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The character code delimiter",
                default_value: "Space",
            },
            ArgSchema {
                name: "Base",
                description: "The numerical base of the codes (2-36)",
                default_value: "16",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = match String::from_utf8(input.clone()) {
            Ok(s) => s,
            Err(_) => input.into_iter().map(|b| b as char).collect(),
        };

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("Space");
        let base = args.get(1).and_then(|v| v.as_f64()).unwrap_or(16.0) as u32;

        if base < 2 || base > 36 {
            return Err(OperationError::InvalidArgument {
                name: "Base".to_string(),
                reason: "Base argument must be between 2 and 36".to_string(),
            });
        }

        let delim = char_rep(delim_name);

        // Iterate over unicode codepoints (not bytes), matching JS Utils.strToCharcode
        let mut parts: Vec<String> = Vec::new();
        for ch in input_str.chars() {
            let ordinal = ch as u32;
            if base == 16 {
                // Dynamic padding: 2 nibbles for < 256, 4 for < 65536, etc.
                let padding = if ordinal < 256 {
                    2
                } else if ordinal < 65536 {
                    4
                } else if ordinal < 16_777_216 {
                    6
                } else {
                    8
                };
                parts.push(format!("{:0>width$x}", ordinal, width = padding));
            } else {
                parts.push(radix_string(ordinal, base));
            }
        }

        Ok(parts.join(delim).into_bytes())
    }
}

/// Convert a u32 to a string in the given radix (2-36).
fn radix_string(mut n: u32, radix: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut buf = Vec::new();
    while n > 0 {
        buf.push(DIGITS[(n % radix) as usize] as char);
        n /= radix;
    }
    buf.iter().rev().collect()
}
