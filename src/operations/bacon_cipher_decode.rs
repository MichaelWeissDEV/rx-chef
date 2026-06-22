/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bacon Cipher Decode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bacon Cipher Decode operation
///
/// Bacon's cipher or the Baconian cipher is a method of steganography devised by
/// Francis Bacon in 1605. A message is concealed in the presentation of text,
/// rather than its content.
pub struct BaconCipherDecode;

impl Operation for BaconCipherDecode {
    fn name(&self) -> &'static str {
        "Bacon Cipher Decode"
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
                description: "Translation method (0/1, A/B, Case, or A-M/N-Z)",
                default_value: "0/1",
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
        let invert = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // Get alphabet
        let alphabet_obj = match alphabet {
            "Standard (I=J and U=V)" => "ABCDEFGHIKLMNOPQRSTUWXYZ".to_string(),
            "Complete" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            _ => "ABCDEFGHIKLMNOPQRSTUWXYZ".to_string(),
        };

        let mut cleaned_input = input_str.clone();

        // Clear invalid characters based on translation
        match translation {
            "0/1" => {
                cleaned_input = cleaned_input.replace(|c: char| c != '0' && c != '1', "");
            }
            "A/B" => {
                cleaned_input = cleaned_input
                    .replace(|c: char| c != 'A' && c != 'B' && c != 'a' && c != 'b', "");
            }
            "Case" => {
                // For Case translation, convert to 0/1 based on uppercase/lowercase
                let mut result = String::new();
                for c in cleaned_input.chars() {
                    if c.is_uppercase() {
                        result.push('1');
                    } else if c.is_lowercase() {
                        result.push('0');
                    }
                }
                cleaned_input = result;
            }
            "A-M/N-Z first letter" => {
                // For A-M/N-Z translation, check first letter of each word
                let mut result = String::new();
                for word in cleaned_input.split_whitespace() {
                    if let Some(first_char) = word.chars().next() {
                        let upper_first = first_char.to_ascii_uppercase();
                        if upper_first >= 'A' && upper_first <= 'M' {
                            result.push('0');
                        } else if upper_first >= 'N' && upper_first <= 'Z' {
                            result.push('1');
                        }
                    }
                }
                cleaned_input = result;
            }
            _ => {
                cleaned_input = cleaned_input.replace(|c: char| c != '0' && c != '1', "");
            }
        }

        // Normalize A/B to 0/1
        if translation == "A/B" {
            cleaned_input = cleaned_input
                .replace('A', "0")
                .replace('a', "0")
                .replace('B', "1")
                .replace('b', "1");
        }

        if invert {
            cleaned_input = cleaned_input
                .replace('0', "x")
                .replace('1', "0")
                .replace('x', "1");
        }

        // Group into 5-character chunks
        let chunks: Vec<String> = cleaned_input
            .chars()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        let mut output = String::new();
        for chunk in chunks {
            if let Ok(code) = u8::from_str_radix(&chunk, 2) {
                let idx = code as usize;
                if idx < alphabet_obj.len() {
                    let ch = alphabet_obj.chars().nth(idx).ok_or_else(|| {
                        OperationError::ProcessingError(format!(
                            "index {} out of alphabet range",
                            idx
                        ))
                    })?;
                    output.push(ch);
                } else {
                    output.push('?');
                }
            }
        }

        Ok(output.into_bytes())
    }
}
