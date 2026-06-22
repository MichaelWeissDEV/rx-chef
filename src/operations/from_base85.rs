/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base85 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base85 operation
pub struct FromBase85;

impl Operation for FromBase85 {
    fn name(&self) -> &'static str {
        "From Base85"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base85 (also called Ascii85) is a notation for encoding arbitrary byte data. It is usually more efficient than Base64.\n\nThis operation decodes data from an ASCII string (with an alphabet of your choosing, presets included).\n\ne.g. BOu!rD]j7BEbo7 becomes hello world\n\nBase85 is commonly used in Adobe's PostScript and PDF file formats."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Alphabet",
                description: "The Base85 alphabet",
                default_value: "!-u",
            },
            ArgSchema {
                name: "Remove non-alphabet chars",
                description: "Remove characters not in the alphabet before decoding",
                default_value: "true",
            },
            ArgSchema {
                name: "All-zero group char",
                description: "Character representing an all-zero group (default 'z')",
                default_value: "z",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        let alphabet_arg = args.first().and_then(|v| v.as_str()).unwrap_or("!-u");
        let remove_non_alph = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let all_zero_group_char = args
            .get(2)
            .and_then(|v| v.as_str())
            .and_then(|s| s.chars().next());

        let alphabet = expand_alphabet(alphabet_arg);
        if alphabet.chars().count() != 85 {
            return Err(OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: "Alphabet must be of length 85".to_string(),
            });
        }

        if let Some(c) = all_zero_group_char {
            if alphabet.contains(c) {
                return Err(OperationError::InvalidArgument {
                    name: "All-zero group char".to_string(),
                    reason: "The all-zero group char cannot appear in the alphabet".to_string(),
                });
            }
        }

        let mut input_data = input_str.to_string();

        // Remove delimiters if present
        if input_data.starts_with("<~") && input_data.ends_with("~>") {
            input_data = input_data[2..input_data.len() - 2].to_string();
        }

        // Remove non-alphabet characters
        if remove_non_alph {
            input_data = input_data
                .chars()
                .filter(|&c| {
                    alphabet.contains(c)
                        || (all_zero_group_char.is_some() && Some(c) == all_zero_group_char)
                })
                .collect();

            // Remove delimiters again if present (in case of non-alphabet characters in front/behind delimiters)
            if input_data.starts_with("<~") && input_data.ends_with("~>") {
                input_data = input_data[2..input_data.len() - 2].to_string();
            }
        }

        if input_data.is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();
        let chars: Vec<char> = input_data.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if all_zero_group_char.is_some() && Some(chars[i]) == all_zero_group_char {
                result.extend_from_slice(&[0, 0, 0, 0]);
                i += 1;
            } else {
                let mut digits = Vec::new();
                let mut chunk_size = 0;
                for j in 0..5 {
                    if i + j < chars.len() {
                        let c = chars[i + j];
                        if alphabet.contains(c) {
                            // Find byte index and convert to char index for 85-base
                            digits.push(alphabet.chars().take_while(|&x| x != c).count() as u64);
                            chunk_size += 1;
                        } else if all_zero_group_char.is_some() && Some(c) == all_zero_group_char {
                            // This case should probably not happen in the middle of a 5-char block
                            // based on CyberChef's implementation, it seems it handles 'z' separately.
                            break;
                        } else {
                            return Err(OperationError::InvalidInput(format!(
                                "Invalid character '{}' at index {}",
                                c,
                                i + j
                            )));
                        }
                    } else {
                        digits.push(84);
                    }
                }

                if chunk_size == 0 {
                    i += 1;
                    continue;
                }

                let block: u64 = digits[0] * 52200625
                    + digits[1] * 614125
                    + digits[2] * 7225
                    + digits[3] * 85
                    + digits[4];

                let block_bytes = [
                    ((block >> 24) & 0xff) as u8,
                    ((block >> 16) & 0xff) as u8,
                    ((block >> 8) & 0xff) as u8,
                    (block & 0xff) as u8,
                ];

                if chunk_size == 5 {
                    result.extend_from_slice(&block_bytes);
                } else {
                    // For chunks smaller than 5, we only take chunk_size - 1 bytes
                    for k in 0..(chunk_size - 1) {
                        result.push(block_bytes[k]);
                    }
                }
                i += chunk_size;
            }
        }

        Ok(result)
    }
}

fn expand_alphabet(alphabet: &str) -> String {
    // Handle standard alphabet specially
    if alphabet == "!-u" {
        return (33..=117)
            .map(|c| std::char::from_u32(c).unwrap())
            .collect();
    }
    if alphabet == "0-9a-zA-Z.\\-:+=^!/*?&<>()[]{}@%$#" {
        let mut res = String::new();
        for c in '0'..='9' {
            res.push(c);
        }
        for c in 'a'..='z' {
            res.push(c);
        }
        for c in 'A'..='Z' {
            res.push(c);
        }
        res.push_str(".-:+=^!/*?&<>()[]{}@%$#");
        return res;
    }
    if alphabet == "0-9A-Za-z!#$%&()*+\\-;<=>?@^_`{|}~" {
        let mut res = String::new();
        for c in '0'..='9' {
            res.push(c);
        }
        for c in 'A'..='Z' {
            res.push(c);
        }
        for c in 'a'..='z' {
            res.push(c);
        }
        res.push_str("!#$%&()*+-;<=>?@^_`{|}~");
        return res;
    }

    let mut result = String::new();
    let chars: Vec<char> = alphabet.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '-' {
            let start = chars[i] as u32;
            let end = chars[i + 2] as u32;
            for code in start..=end {
                if let Some(c) = std::char::from_u32(code) {
                    result.push(c);
                }
            }
            i += 3;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
