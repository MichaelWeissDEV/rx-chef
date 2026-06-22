/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Swap endianness operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Swap endianness operation
///
/// Switches the data from big-endian to little-endian or vice-versa.
/// Data can be read in as hexadecimal or raw bytes.
pub struct SwapEndianness;

impl Operation for SwapEndianness {
    fn name(&self) -> &'static str {
        "Swap endianness"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Switches the data from big-endian to little-endian or vice-versa. Data can be read in as \
         hexadecimal or raw bytes. It will be returned in the same format as it is entered."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Data format",
                description: "Input/output format: Hex or Raw",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Word length (bytes)",
                description: "Number of bytes per word",
                default_value: "4",
            },
            ArgSchema {
                name: "Pad incomplete words",
                description: "If true, pad incomplete words with zero bytes",
                default_value: "true",
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
        let data_format = args.first().and_then(|a| a.as_str()).unwrap_or("Raw");
        let word_length = args.get(1).and_then(|a| a.as_usize()).unwrap_or(4);
        let pad = args.get(2).and_then(|a| a.as_bool()).unwrap_or(true);

        if word_length == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Word length (bytes)".to_string(),
                reason: "Word length must be greater than 0".to_string(),
            });
        }

        let input_str = String::from_utf8_lossy(&input).to_string();

        // Parse input into raw bytes
        let data: Vec<u8> = match data_format {
            "Hex" => {
                let clean: String = input_str
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect();
                hex::decode(&clean).map_err(|e| OperationError::InvalidInput(e.to_string()))?
            }
            _ => input_str.as_bytes().to_vec(),
        };

        // Split into words and reverse each word
        let mut result: Vec<u8> = Vec::with_capacity(data.len());

        for chunk in data.chunks(word_length) {
            let mut word = chunk.to_vec();
            // Pad if needed
            if pad && word.len() < word_length {
                word.resize(word_length, 0);
            }
            // Reverse the word (swap endianness)
            word.reverse();
            result.extend_from_slice(&word);
        }

        // Convert result back to the original format
        match data_format {
            "Hex" => {
                let hex_str = result
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                Ok(hex_str.into_bytes())
            }
            _ => Ok(result),
        }
    }
}
