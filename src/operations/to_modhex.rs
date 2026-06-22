/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Modhex operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Modhex operation - converts raw bytes to Modhex encoding (used in YubiKey).
///
/// Modhex alphabet: cbdefghijklnrtuv (maps to 0-15)
pub struct ToModhex;

const MODHEX_ALPHABET: &[u8] = b"cbdefghijklnrtuv";

impl Operation for ToModhex {
    fn name(&self) -> &'static str {
        "To Modhex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts the input string to modhex bytes separated by the specified delimiter."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between modhex pairs (None, Space, Comma, Semi-colon, Colon, Line feed, CRLF)",
                default_value: "None",
            },
            ArgSchema {
                name: "Bytes per line",
                description: "Number of bytes per output line (0 = no limit)",
                default_value: "0",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let delim_name = args.first().and_then(|v| v.as_str()).unwrap_or("None");
        let bytes_per_line = args.get(1).and_then(|v| v.as_usize()).unwrap_or(0);

        let delim = match delim_name {
            "None" => "",
            "Space" => " ",
            "Comma" => ",",
            "Semi-colon" => ";",
            "Colon" => ":",
            "Line feed" => "\n",
            "CRLF" => "\r\n",
            other => other,
        };

        let encoded: Vec<String> = input
            .iter()
            .map(|&b| {
                let hi = MODHEX_ALPHABET[(b >> 4) as usize] as char;
                let lo = MODHEX_ALPHABET[(b & 0x0f) as usize] as char;
                format!("{}{}", hi, lo)
            })
            .collect();

        let result = if bytes_per_line > 0 {
            encoded
                .chunks(bytes_per_line)
                .map(|chunk| chunk.join(delim))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            encoded.join(delim)
        };

        Ok(result.into_bytes())
    }
}
