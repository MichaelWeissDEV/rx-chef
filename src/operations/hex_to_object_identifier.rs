/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Hex to Object Identifier operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Hex to Object Identifier operation
///
/// Converts a hexadecimal string (DER OID value octets) back to a dotted
/// OID string such as "1.2.840.113549".
pub struct HexToObjectIdentifier;

fn der_bytes_to_oid(bytes: &[u8]) -> Result<String, OperationError> {
    if bytes.is_empty() {
        return Err(OperationError::InvalidInput("Empty input".to_string()));
    }

    // Decode base-128 values
    let mut arcs: Vec<u64> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let mut val: u64 = 0;
        loop {
            if i >= bytes.len() {
                return Err(OperationError::InvalidInput(
                    "Truncated base-128 integer".to_string(),
                ));
            }
            let b = bytes[i] as u64;
            i += 1;
            val = val
                .checked_shl(7)
                .ok_or_else(|| OperationError::InvalidInput("OID arc overflow".to_string()))?;
            val |= b & 0x7f;
            if b & 0x80 == 0 {
                break;
            }
        }
        arcs.push(val);
    }

    if arcs.is_empty() {
        return Err(OperationError::InvalidInput(
            "No OID arcs decoded".to_string(),
        ));
    }

    // The first encoded value encodes two arcs: 40*arc0 + arc1
    let first = arcs[0];
    let (arc0, arc1) = if first < 40 {
        (0u64, first)
    } else if first < 80 {
        (1u64, first - 40)
    } else {
        (2u64, first - 80)
    };

    let mut parts = vec![arc0.to_string(), arc1.to_string()];
    for arc in &arcs[1..] {
        parts.push(arc.to_string());
    }

    Ok(parts.join("."))
}

impl Operation for HexToObjectIdentifier {
    fn name(&self) -> &'static str {
        "Hex to Object Identifier"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Converts a hexadecimal string into an object identifier (OID) dotted string."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let s = String::from_utf8_lossy(&input);
        // Strip all whitespace before decoding
        let hex_str: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let bytes = hex::decode(&hex_str)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?;
        let oid = der_bytes_to_oid(&bytes)?;
        Ok(oid.into_bytes())
    }
}
