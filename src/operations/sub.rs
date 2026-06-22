/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SUB operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SUB operation
///
/// SUB the input with the given key (e.g. `fe023da5`), MOD 256.
///
/// Applies the same `bitOp` pattern as XOR/AND/OR/ADD but uses subtraction:
/// `result = (input_byte - key_byte + 256) % 256`
///
/// The key cycles over the input.  Differential schemes ("Input differential"
/// and "Output differential") are supported exactly as in the JS source.
pub struct SUB;

/// Parse a hex key string into bytes.  Mirrors the logic used by the existing
/// add.rs / xor.rs implementations.
fn parse_key(args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
    let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");
    if key_str.is_empty() {
        return Ok(Vec::new());
    }

    if let Some(hex_digits) = key_str
        .strip_prefix("0x")
        .or_else(|| key_str.strip_prefix("0X"))
    {
        return parse_hex(hex_digits);
    }

    // All hex non-digit chars and even length -> treat as raw hex
    let is_pure_hex_alpha = key_str
        .chars()
        .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_digit());
    if is_pure_hex_alpha && key_str.len().is_multiple_of(2) {
        return parse_hex(key_str);
    }

    // Decimal single byte
    if key_str.chars().all(|c| c.is_ascii_digit()) {
        if let Ok(v) = key_str.parse::<u8>() {
            return Ok(vec![v]);
        }
        return Ok(key_str.bytes().collect());
    }

    // Fallback: raw UTF-8 bytes
    Ok(key_str.bytes().collect())
}

fn parse_hex(s: &str) -> Result<Vec<u8>, OperationError> {
    let lower = s.to_lowercase();
    let mut bytes = Vec::new();
    for chunk in lower.as_bytes().chunks(2) {
        let cs = std::str::from_utf8(chunk).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: "Invalid hex string".to_string(),
        })?;
        let b = u8::from_str_radix(cs, 16).map_err(|_| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: format!("Invalid hex value: {}", cs),
        })?;
        bytes.push(b);
    }
    Ok(bytes)
}

impl Operation for SUB {
    fn name(&self) -> &'static str {
        "SUB"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "SUB the input with the given key (e.g. fe023da5), MOD 256"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Key to SUB from input bytes",
                default_value: "",
            },
            ArgSchema {
                name: "Scheme",
                description: "Scheme (Standard, Input differential, Output differential)",
                default_value: "Standard",
            },
            ArgSchema {
                name: "Null preserving",
                description: "If true, bytes that are 0 or equal to the key byte are not modified",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut key_bytes = parse_key(args)?;

        if key_bytes.is_empty() {
            return Ok(input);
        }

        let scheme = args.get(1).and_then(|a| a.as_str()).unwrap_or("Standard");
        let null_preserving = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        let mut result = Vec::with_capacity(input.len());

        for (i, &byte) in input.iter().enumerate() {
            let key_pos = i % key_bytes.len();
            let k = key_bytes[key_pos];

            let x = if null_preserving && (byte == 0 || byte == k) {
                byte
            } else {
                // (byte - k + 256) % 256   avoids underflow on u8
                byte.wrapping_sub(k)
            };

            result.push(x);

            // Update key for differential schemes (only when not null-preserved)
            if scheme != "Standard" && !(null_preserving && (byte == 0 || byte == k)) {
                match scheme {
                    "Input differential" => key_bytes[key_pos] = byte,
                    "Output differential" => key_bytes[key_pos] = x,
                    _ => {}
                }
            }
        }

        Ok(result)
    }
}
