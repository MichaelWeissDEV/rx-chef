/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Object Identifier to Hex operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Object Identifier to Hex operation
///
/// Converts an OID string (e.g. "1.2.840.113549.1.1.11") into its DER
/// value-octets hex representation.  The first two arcs are encoded as
/// 40*arc0 + arc1; every subsequent arc is encoded as variable-length
/// base-128 (big-endian, high bit set on all but the last byte).
pub struct ObjectIdentifierToHex;

fn oid_to_der_bytes(oid: &str) -> Result<Vec<u8>, OperationError> {
    let parts: Vec<&str> = oid.split('.').collect();
    if parts.len() < 2 {
        return Err(OperationError::InvalidInput(
            "OID must have at least two arcs".to_string(),
        ));
    }

    let arc0: u64 = parts[0]
        .parse()
        .map_err(|_| OperationError::InvalidInput("Invalid OID arc".to_string()))?;
    let arc1: u64 = parts[1]
        .parse()
        .map_err(|_| OperationError::InvalidInput("Invalid OID arc".to_string()))?;

    if arc0 > 2 {
        return Err(OperationError::InvalidInput(
            "First OID arc must be 0, 1, or 2".to_string(),
        ));
    }
    if arc0 < 2 && arc1 > 39 {
        return Err(OperationError::InvalidInput(
            "Second OID arc must be 0-39 for first arc 0 or 1".to_string(),
        ));
    }

    let first = 40 * arc0 + arc1;
    let mut out = encode_base128(first);

    for part in &parts[2..] {
        let val: u64 = part
            .parse()
            .map_err(|_| OperationError::InvalidInput("Invalid OID arc".to_string()))?;
        out.extend(encode_base128(val));
    }

    Ok(out)
}

fn encode_base128(mut val: u64) -> Vec<u8> {
    if val == 0 {
        return vec![0x00];
    }
    let mut bytes = Vec::new();
    while val > 0 {
        bytes.push((val & 0x7f) as u8);
        val >>= 7;
    }
    bytes.reverse();
    // set high bit on all but the last byte
    let last = bytes.len() - 1;
    for i in 0..last {
        bytes[i] |= 0x80;
    }
    bytes
}

impl Operation for ObjectIdentifierToHex {
    fn name(&self) -> &'static str {
        "Object Identifier to Hex"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Converts an object identifier (OID) into a hexadecimal string representing the DER value octets."
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
        let oid = String::from_utf8_lossy(&input);
        let oid = oid.trim();
        let der = oid_to_der_bytes(oid)?;
        let hex = hex::encode(der);
        Ok(hex.into_bytes())
    }
}
