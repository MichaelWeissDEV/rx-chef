/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rotate right operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Rotate right operation
///
/// Rotates each byte to the right by the number of bits specified,
/// optionally carrying the excess bits over to the next byte.
pub struct RotateRight;

impl Operation for RotateRight {
    fn name(&self) -> &'static str {
        "Rotate right"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Rotates each byte to the right by the number of bits specified, optionally carrying the \
         excess bits over to the next byte. Currently only supports 8-bit values."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Amount",
                description: "Number of bits to rotate right",
                default_value: "1",
            },
            ArgSchema {
                name: "Carry through",
                description: "If true, carry bits from one byte to the next across all bytes",
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
        let amount = args.first().and_then(|a| a.as_usize()).unwrap_or(1);
        let carry = args.get(1).and_then(|a| a.as_bool()).unwrap_or(false);

        if carry {
            Ok(rotr_carry(&input, amount))
        } else {
            Ok(rot(&input, amount, rotr_byte))
        }
    }
}

/// Rotate a single byte right by 1 bit (wrapping within the byte).
fn rotr_byte(b: u8) -> u8 {
    let bit = (b & 1) << 7;
    (b >> 1) | bit
}

/// Apply a per-byte rotation function `amount` times to each byte.
fn rot(data: &[u8], amount: usize, algo: fn(u8) -> u8) -> Vec<u8> {
    data.iter()
        .map(|&b| {
            let mut val = b;
            for _ in 0..amount {
                val = algo(val);
            }
            val
        })
        .collect()
}

/// Rotate the entire byte array right by `amount` bits, carrying across byte boundaries.
/// Mirrors JS rotrCarry: iterates forward through the array.
fn rotr_carry(data: &[u8], amount: usize) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let amount = amount % 8;
    if amount == 0 {
        return data.to_vec();
    }

    let mut result = vec![0u8; data.len()];
    let mut carry_bits: u8 = 0;
    let mask = (1u8 << amount).wrapping_sub(1);

    for i in 0..data.len() {
        let old_byte = data[i];
        let new_byte = (old_byte >> amount) | carry_bits;
        carry_bits = (old_byte & mask) << (8 - amount);
        result[i] = new_byte;
    }
    // Wrap carry bits to the first byte (index 0)
    result[0] |= carry_bits;

    result
}
