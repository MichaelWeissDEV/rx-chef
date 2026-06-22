/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bacon Cipher Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bacon Cipher Encode operation
///
/// Bacon's cipher or the Baconian cipher is a method of steganography devised by
/// Francis Bacon in 1605. A message is concealed in the presentation of text,
/// rather than its content.
pub struct BaconCipherEncode;

impl Operation for BaconCipherEncode {
    fn name(&self) -> &'static str {
        "Bacon Cipher Encode"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Bacon's cipher or the Baconian cipher is a method of steganography devised by Francis Bacon in 1605. A message is concealed in the presentation of text, rather than its content."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "Alphabet variant (Standard or Complete)",
                default_value: "Standard (I=J and U=V)",
            },
            ArgSchema {
                name: "Translation",
                description: "Translation method (0/1 or A/B)",
                default_value: "0/1",
            },
            ArgSchema {
                name: "Keep extra characters",
                description: "Keep non-alphabetic characters",
                default_value: "false",
            },
            ArgSchema {
                name: "Invert Translation",
                description: "Invert 0/1 or A/B",
                default_value: "false",
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
        let alphabet = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Standard (I=J and U=V)");
        let translation = args.get(1).and_then(|a| a.as_str()).unwrap_or("0/1");
        let keep = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);
        let invert = args.get(3).and_then(|a| a.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // Get alphabet
        let alphabet_obj = match alphabet {
            "Standard (I=J and U=V)" => "ABCDEFGHIKLMNOPQRSTUWXYZ",
            "Complete" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            _ => "ABCDEFGHIKLMNOPQRSTUWXYZ",
        };

        let mut output = String::new();

        for c in input_str.chars() {
            if let Some(pos) = alphabet_obj.find(c.to_ascii_uppercase()) {
                let code = pos as u8;
                let bacon = format!("{:05b}", code);
                output.push_str(&bacon);
            } else {
                if keep {
                    output.push(c);
                }
            }
        }

        if invert {
            output = output.replace('0', "x").replace('1', "0").replace('x', "1");
        }

        if !keep {
            output = output
                .chars()
                .filter(|c| *c == '0' || *c == '1')
                .collect::<String>();
            // Group into 5-character chunks
            output = output
                .chars()
                .collect::<Vec<char>>()
                .chunks(5)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(" ");
        }

        if translation == "A/B" {
            output = output.replace('0', "A").replace('1', "B");
        }

        Ok(output.into_bytes())
    }
}
