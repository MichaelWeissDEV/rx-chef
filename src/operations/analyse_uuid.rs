/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Analyse UUID operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{DataType, Operation, OperationError};

/// Analyse UUID operation
pub struct AnalyseUUID;

impl Operation for AnalyseUUID {
    fn name(&self) -> &'static str {
        "Analyse UUID"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Operation for extracting metadata and detecting the version of a given UUID."
    }

    fn args_schema(&self) -> &'static [crate::operation::ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(
        &self,
        input: Vec<u8>,
        _args: &[crate::operation::ArgValue],
    ) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
        let input = input_str.trim();

        // UUID format: 8-4-4-4-12 hex digits
        let uuid_pattern = regex::Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .map_err(|_| OperationError::InvalidInput("Invalid regex".to_string()))?;

        if !uuid_pattern.is_match(input) {
            return Err(OperationError::InvalidInput("Invalid UUID".to_string()));
        }

        // Parse UUID bytes - CyberChef uses the uuid library which parses the UUID
        // The version is stored in byte 6 (0-indexed), in the 4 most significant bits
        let uuid_bytes = parse_uuid_bytes(input)
            .ok_or_else(|| OperationError::InvalidInput("Invalid UUID".to_string()))?;
        let version = (uuid_bytes[6] >> 4) as u8;

        let mut output = String::new();
        output.push_str(&format!("Version:\n{}\n", version));

        match version {
            1 | 6 => {
                output.push_str("\nThis UUID is version 1 or 6 (time-based).\n");
                output.push_str("Timestamp and node information is available.");
            }
            3 => {
                output.push_str(
                    "\nThis UUID was generated using UUID version 3 (name-based, MD5 hashing).\n",
                );
                output.push_str("It is deterministic based on a namespace and name.");
            }
            4 => {
                output.push_str("\nThis UUID was generated using UUID version 4 (random).\n");
                output.push_str("It contains random bits.");
            }
            5 => {
                output.push_str(
                    "\nThis UUID was generated using UUID version 5 (name-based, SHA-1 hashing).\n",
                );
                output.push_str("It is deterministic based on a namespace and name.");
            }
            7 => {
                output.push_str(
                    "\nThis UUID was generated using UUID version 7 (Unix epoch time-based).\n",
                );
                output.push_str("It contains timestamp and random bits.");
            }
            8 => {
                output
                    .push_str("\nThis UUID was generated using UUID version 8 (custom format).\n");
                output.push_str("It is a variant for custom implementations.");
            }
            _ => {
                output.push_str(&format!("\nUnknown UUID version: {}", version));
            }
        }

        Ok(output.into_bytes())
    }
}

/// Parse a UUID string into 16 bytes
fn parse_uuid_bytes(uuid_str: &str) -> Option<[u8; 16]> {
    let parts: Vec<&str> = uuid_str.split('-').collect();
    if parts.len() != 5 {
        return None;
    }

    let mut bytes = [0u8; 16];
    let mut idx = 0;

    // Parse each part
    for part in parts {
        let len = part.len();
        if len == 8 || len == 4 || len == 12 {
            for i in (0..len).step_by(2) {
                let hex = &part[i..(i + 2).min(len)];
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    bytes[idx] = byte;
                    idx += 1;
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }

    Some(bytes)
}
