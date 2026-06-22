/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Change IP format operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Change IP format operation
///
/// Convert an IP address from one format to another.
pub struct ChangeIPFormat;

impl Operation for ChangeIPFormat {
    fn name(&self) -> &'static str {
        "Change IP format"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Convert an IP address from one format to another, e.g. 172.20.23.54 to ac141736"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input format",
                description: "Input format: Dotted Decimal, Decimal, Octal, Hex",
                default_value: "Dotted Decimal",
            },
            ArgSchema {
                name: "Output format",
                description: "Output format: Dotted Decimal, Decimal, Octal, Hex",
                default_value: "Dotted Decimal",
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
        let in_format = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Dotted Decimal");
        let out_format = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("Dotted Decimal");

        let input_str = String::from_utf8_lossy(&input);
        let lines: Vec<&str> = input_str.lines().collect();
        let mut output = String::new();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let ba_ip = match self.from_input_format(line, in_format)? {
                Some(ip) => ip,
                None => continue,
            };

            let result = self.to_output_format(&ba_ip, out_format)?;
            output.push_str(&result);
            output.push('\n');
        }

        // Remove trailing newline
        if !output.is_empty() && output.ends_with('\n') {
            output.pop();
        }

        Ok(output.into_bytes())
    }
}

impl ChangeIPFormat {
    fn from_input_format(
        &self,
        value: &str,
        format: &str,
    ) -> Result<Option<[u8; 4]>, OperationError> {
        match format {
            "Dotted Decimal" => {
                let octets: Vec<u32> = value
                    .split('.')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect();

                if octets.len() != 4 {
                    return Err(OperationError::InvalidInput(format!(
                        "Invalid dotted decimal IP: {}",
                        value
                    )));
                }

                let ip: [u8; 4] = [
                    octets[0] as u8,
                    octets[1] as u8,
                    octets[2] as u8,
                    octets[3] as u8,
                ];
                Ok(Some(ip))
            }
            "Decimal" => Ok(Some(self.from_number(value, 10)?)),
            "Octal" => Ok(Some(self.from_number(value, 8)?)),
            "Hex" => {
                let hex = value.trim_start_matches("0x").trim_start_matches("0X");
                if hex.len() != 8 {
                    return Err(OperationError::InvalidInput(format!(
                        "Invalid hex IP: {}. Expected 8 hex digits.",
                        value
                    )));
                }
                let val = u32::from_str_radix(hex, 16).map_err(|e| {
                    OperationError::InvalidInput(format!("Invalid hex value: {}", e))
                })?;
                let ip: [u8; 4] = val.to_be_bytes();
                Ok(Some(ip))
            }
            _ => Err(OperationError::InvalidInput(format!(
                "Unsupported input format: {}",
                format
            ))),
        }
    }

    fn from_number(&self, value: &str, radix: u32) -> Result<[u8; 4], OperationError> {
        let decimal = u32::from_str_radix(value, radix)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid number: {}", e)))?;
        let ip: [u8; 4] = [
            (decimal >> 24) as u8,
            (decimal >> 16) as u8,
            (decimal >> 8) as u8,
            (decimal & 0xff) as u8,
        ];
        Ok(ip)
    }

    fn to_output_format(&self, ip: &[u8; 4], format: &str) -> Result<String, OperationError> {
        let dec_ip = u32::from_be_bytes(*ip);

        match format {
            "Dotted Decimal" => Ok(format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])),
            "Decimal" => Ok(dec_ip.to_string()),
            "Octal" => Ok(format!("0{:o}", dec_ip)),
            "Hex" => Ok(format!("{:08x}", dec_ip)),
            _ => Err(OperationError::InvalidInput(format!(
                "Unsupported output format: {}",
                format
            ))),
        }
    }
}
