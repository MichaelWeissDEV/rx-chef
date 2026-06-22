/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the AND operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// AND operation
///
/// AND the input with the given key.
pub struct AndOp;

impl Operation for AndOp {
    fn name(&self) -> &'static str {
        "AND"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "AND the input with the given key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Key",
            description: "Key to AND with input",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_bytes = parse_key(args)?;

        if key_bytes.is_empty() {
            return Ok(input);
        }

        let mut result = Vec::with_capacity(input.len());
        for (i, byte) in input.iter().enumerate() {
            let key_byte = key_bytes[i % key_bytes.len()];
            result.push(byte & key_byte);
        }

        Ok(result)
    }
}

/// Parse the key argument into bytes
fn parse_key(args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
    let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");

    if key_str.is_empty() {
        return Ok(Vec::new());
    }

    // If it starts with 0x, parse as hex
    if key_str.len() >= 2 && key_str.starts_with("0x") {
        return parse_hex(&key_str[2..]);
    }

    // Check if it looks like hex (only hex digits: 0-9, a-f, A-F)
    let is_hex = key_str
        .chars()
        .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_digit());

    // If it's all hex digits (a-f, A-F only) with even length, parse as hex
    if is_hex && key_str.len().is_multiple_of(2) {
        return parse_hex(key_str);
    }

    // If string contains only digits, parse as decimal
    if key_str.chars().all(|c| c.is_ascii_digit()) {
        // Try to parse as single byte decimal value
        if let Ok(value) = key_str.parse::<u8>() {
            return Ok(vec![value]);
        }
        // If it doesn't fit in a byte, use UTF-8 bytes
        return Ok(key_str.bytes().collect());
    }

    // Default: treat as UTF-8 string and use byte values
    Ok(key_str.bytes().collect())
}

/// Parse a hex string into bytes
fn parse_hex(hex_str: &str) -> Result<Vec<u8>, OperationError> {
    let mut bytes = Vec::new();
    let hex_lower = hex_str.to_lowercase();

    for chunk in hex_lower.as_bytes().chunks(2) {
        let chunk_str =
            std::str::from_utf8(chunk).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Invalid hex string".to_string(),
            })?;
        let byte =
            u8::from_str_radix(chunk_str, 16).map_err(|_| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid hex value: {}", chunk_str),
            })?;
        bytes.push(byte);
    }

    Ok(bytes)
}
