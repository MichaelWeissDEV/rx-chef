/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MurmurHash3 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// MurmurHash3 operation
pub struct MurmurHash3;

impl Operation for MurmurHash3 {
    fn name(&self) -> &'static str {
        "MurmurHash3"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "Generates a MurmurHash v3 for a string input and an optional seed input"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Seed",
                description: "Positive integer only",
                default_value: "0",
            },
            ArgSchema {
                name: "Convert to Signed",
                description: "Whether to convert the output to a signed 32-bit integer",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let seed = args.first().and_then(|v| v.as_f64()).unwrap_or(0.0) as u32;
        let signed = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8_lossy(&input);
        let hash = mmh3_32(input_str.as_ref(), seed);

        if signed {
            Ok((hash as i32).to_string().into_bytes())
        } else {
            Ok(hash.to_string().into_bytes())
        }
    }
}

/// 32-bit MurmurHash3 implementation
fn mmh3_32(input: &str, seed: u32) -> u32 {
    let bytes = input.as_bytes();
    let n_blocks = bytes.len() / 4;
    let mut h1 = seed;

    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;

    // Body
    for i in 0..n_blocks {
        let mut k1 = u32::from_le_bytes([
            bytes[i * 4],
            bytes[i * 4 + 1],
            bytes[i * 4 + 2],
            bytes[i * 4 + 3],
        ]);

        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    // Tail
    let tail = &bytes[n_blocks * 4..];
    let mut k1 = 0u32;
    match tail.len() {
        3 => {
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        _ => {}
    }

    // Finalization
    h1 ^= bytes.len() as u32;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;

    h1
}
