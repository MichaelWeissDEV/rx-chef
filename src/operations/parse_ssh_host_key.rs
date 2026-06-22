/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse SSH Host Key operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose, Engine as _};
use hex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse SSH Host Key operation
pub struct ParseSshHostKey;

impl Operation for ParseSshHostKey {
    fn name(&self) -> &'static str {
        "Parse SSH Host Key"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses a SSH host key and extracts fields from it.<br>The key type can be:<ul><li>ssh-rsa</li><li>ssh-dss</li><li>ecdsa-sha2</li><li>ssh-ed25519</li></ul>The key format can be either Hex or Base64."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input Format",
            description: "The format of the input key",
            default_value: "Auto",
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
        let input_str = String::from_utf8_lossy(&input).trim().to_string();
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let input_format = args.first().and_then(|a| a.as_str()).unwrap_or("Auto");

        let binary_key = self.convert_key_to_binary(&input_str, input_format)?;
        let fields = self.parse_key(&binary_key);

        if fields.is_empty() {
            return Err(OperationError::ProcessingError(
                "No fields found in key".to_string(),
            ));
        }

        let key_type = String::from_utf8_lossy(&fields[0]).to_string();
        let mut output = format!("Key type: {}", key_type);

        match key_type.as_str() {
            "ssh-rsa" => {
                if fields.len() >= 3 {
                    output += &format!("\nExponent: 0x{}", hex::encode(&fields[1]));
                    output += &format!("\nModulus: 0x{}", hex::encode(&fields[2]));
                }
            }
            "ssh-dss" => {
                if fields.len() >= 5 {
                    output += &format!("\np: 0x{}", hex::encode(&fields[1]));
                    output += &format!("\nq: 0x{}", hex::encode(&fields[2]));
                    output += &format!("\ng: 0x{}", hex::encode(&fields[3]));
                    output += &format!("\ny: 0x{}", hex::encode(&fields[4]));
                }
            }
            t if t.starts_with("ecdsa-sha2") => {
                if fields.len() >= 3 {
                    output += &format!("\nCurve: {}", String::from_utf8_lossy(&fields[1]));
                    output += &format!(
                        "\nPoint: 0x{}",
                        fields[2..]
                            .iter()
                            .map(hex::encode)
                            .collect::<Vec<_>>()
                            .join("")
                    );
                }
            }
            "ssh-ed25519" => {
                if fields.len() >= 2 {
                    output += &format!("\nx: 0x{}", hex::encode(&fields[1]));
                }
            }
            _ => {
                output += "\nUnsupported key type.";
                if fields.len() > 1 {
                    output += &format!(
                        "\nParameters: {}",
                        fields[1..]
                            .iter()
                            .map(hex::encode)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
            }
        }

        Ok(output.into_bytes())
    }
}

impl ParseSshHostKey {
    fn convert_key_to_binary(&self, input: &str, format: &str) -> Result<Vec<u8>, OperationError> {
        let mut key_data = input;
        if let Some(pos) = input.find(' ') {
            let first_part = &input[..pos];
            if first_part.starts_with("ssh") || first_part.starts_with("ecdsa-sha2") {
                let second_part = input[pos..].trim();
                if !second_part.is_empty() {
                    key_data = second_part;
                }
            }
        }

        let format = if format == "Auto" {
            self.detect_format(key_data)?
        } else {
            format
        };

        match format {
            "Hex" => {
                let cleaned = key_data
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect::<String>();
                hex::decode(cleaned).map_err(|e| OperationError::InvalidArgument {
                    name: "Input Format".to_string(),
                    reason: format!("Invalid hex: {}", e),
                })
            }
            "Base64" => general_purpose::STANDARD
                .decode(key_data.trim())
                .map_err(|e| OperationError::InvalidArgument {
                    name: "Input Format".to_string(),
                    reason: format!("Invalid base64: {}", e),
                }),
            _ => Err(OperationError::InvalidArgument {
                name: "Input Format".to_string(),
                reason: "Invalid input format.".to_string(),
            }),
        }
    }

    fn detect_format(&self, input: &str) -> Result<&'static str, OperationError> {
        let hex_pattern = input
            .chars()
            .all(|c| c.is_ascii_hexdigit() || " ,;:".contains(c));
        let b64_pattern = input
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "+/=\r\n ".contains(c));

        if hex_pattern && input.len() > 30 && input.contains(|c: char| " ,;:".contains(c)) {
            return Ok("Hex");
        }

        if b64_pattern {
            return Ok("Base64");
        }

        if hex_pattern {
            return Ok("Hex");
        }

        Err(OperationError::InvalidArgument {
            name: "Input Format".to_string(),
            reason: "Unable to detect input key format.".to_string(),
        })
    }

    fn parse_key(&self, key: &[u8]) -> Vec<Vec<u8>> {
        let mut fields = Vec::new();
        let mut offset = 0;
        while offset + 4 <= key.len() {
            let len = ((key[offset] as u32) << 24)
                | ((key[offset + 1] as u32) << 16)
                | ((key[offset + 2] as u32) << 8)
                | (key[offset + 3] as u32);
            offset += 4;
            let len = len as usize;
            if offset + len > key.len() {
                break;
            }
            fields.push(key[offset..offset + len].to_vec());
            offset += len;
            if len == 0 {
                // To avoid infinite loop if len is 0 and we are not at end
                // CyberChef breaks if len <= 0
                break;
            }
        }
        fields
    }
}
