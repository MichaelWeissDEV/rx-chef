/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the URL Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// URL Decode operation
///
/// Converts URI/URL percent-encoded characters back to their raw values.
/// e.g. `%3d` becomes `=`
pub struct URLDecode;

/// Decode a single hex digit.
fn from_hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Percent-decode a byte slice. Returns raw bytes (not necessarily valid UTF-8).
pub fn percent_decode(input: &[u8], plus_is_space: bool) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        let b = input[i];
        if b == b'+' && plus_is_space {
            output.push(b' ');
            i += 1;
        } else if b == b'%' && i + 2 < input.len() {
            let hi = from_hex_digit(input[i + 1]);
            let lo = from_hex_digit(input[i + 2]);
            match (hi, lo) {
                (Some(h), Some(l)) => {
                    output.push((h << 4) | l);
                    i += 3;
                }
                _ => {
                    output.push(b);
                    i += 1;
                }
            }
        } else {
            output.push(b);
            i += 1;
        }
    }
    output
}

impl Operation for URLDecode {
    fn name(&self) -> &'static str {
        "URL Decode"
    }

    fn module(&self) -> &'static str {
        "URL"
    }

    fn description(&self) -> &'static str {
        "Converts URI/URL percent-encoded characters back to their raw values. \
         e.g. %3d becomes ="
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Treat '+' as space",
            description: "Convert '+' characters to spaces in addition to %20",
            default_value: "true",
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
        let plus_is_space = args.first().and_then(|a| a.as_bool()).unwrap_or(true);

        Ok(percent_decode(&input, plus_is_space))
    }
}
