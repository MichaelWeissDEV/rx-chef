/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the XOR Brute Force operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// XOR Brute Force operation
///
/// Enumerate all possible XOR keys of a given length, apply each to a sample
/// of the input, and collect results that contain the crib string (if supplied).
///
/// Args (positional, matching the JS source):
///   0  key_length    (number, default 1)
///   1  sample_length (number, default 100)
///   2  sample_offset (number, default 0)
///   3  scheme        ("Standard" | "Input differential" | "Output differential", default "Standard")
///   4  null_preserving (bool, default false)
///   5  print_key     (bool, default true)
///   6  output_hex    (bool, default false)
///   7  crib          (string, default "")
pub struct XORBruteForce;

/// Apply XOR bitOp to the sample with the given multi-byte key.
fn bit_op_xor(input: &[u8], key_init: &[u8], null_preserving: bool, scheme: &str) -> Vec<u8> {
    if key_init.is_empty() {
        return input.to_vec();
    }
    let mut key: Vec<u8> = key_init.to_vec();
    let mut result = Vec::with_capacity(input.len());

    for (i, &byte) in input.iter().enumerate() {
        let key_pos = i % key.len();
        let k = key[key_pos];

        let x = if null_preserving && (byte == 0 || byte == k) {
            byte
        } else {
            byte ^ k
        };

        result.push(x);

        if scheme != "Standard" && !(null_preserving && (byte == 0 || byte == k)) {
            match scheme {
                "Input differential" => key[key_pos] = byte,
                "Output differential" => key[key_pos] = x,
                _ => {}
            }
        }
    }

    result
}

/// Convert an integer to a big-endian byte array of given length.
fn int_to_byte_array(mut n: u64, len: usize) -> Vec<u8> {
    let mut res = vec![0u8; len];
    for i in (0..len).rev() {
        res[i] = (n & 0xff) as u8;
        n >>= 8;
    }
    res
}

/// Escape non-printable ASCII characters, replicating JS `Utils.escapeWhitespace`.
fn escape_whitespace(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

impl Operation for XORBruteForce {
    fn name(&self) -> &'static str {
        "XOR Brute Force"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Enumerate all possible XOR solutions. Optionally enter a string that you expect to find in the plaintext to filter results (crib)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key length",
                description: "Length of the XOR key in bytes (1..=2 recommended)",
                default_value: "1",
            },
            ArgSchema {
                name: "Sample length",
                description: "Number of bytes of input to process",
                default_value: "100",
            },
            ArgSchema {
                name: "Sample offset",
                description: "Byte offset to start sampling from",
                default_value: "0",
            },
            ArgSchema {
                name: "Scheme",
                description: "Standard, Input differential, or Output differential",
                default_value: "Standard",
            },
            ArgSchema {
                name: "Null preserving",
                description: "Do not XOR null bytes or bytes equal to the key",
                default_value: "false",
            },
            ArgSchema {
                name: "Print key",
                description: "Prefix each result with the key used",
                default_value: "true",
            },
            ArgSchema {
                name: "Output as hex",
                description: "Output results as hex instead of text",
                default_value: "false",
            },
            ArgSchema {
                name: "Crib (known plaintext string)",
                description: "Filter results to those containing this string",
                default_value: "",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_length: usize = args
            .get(0)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.first()
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(1)
            .min(8); // guard against huge iteration counts

        let sample_length: usize = args
            .get(1)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.get(1)
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(100);

        let sample_offset: usize = args
            .get(2)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.get(2)
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(0);

        let scheme = args.get(3).and_then(|a| a.as_str()).unwrap_or("Standard");

        let null_preserving = args.get(4).and_then(|a| a.as_bool()).unwrap_or(false);

        let print_key = args.get(5).and_then(|a| a.as_bool()).unwrap_or(true);

        let output_hex = args.get(6).and_then(|a| a.as_bool()).unwrap_or(false);

        let raw_crib = args
            .get(7)
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .to_lowercase();

        // Slice the sample
        let end = (sample_offset + sample_length).min(input.len());
        let sample = &input[sample_offset.min(input.len())..end];

        let total = 256u64.pow(key_length as u32);
        let mut output_lines: Vec<String> = Vec::new();

        // JS iterates key from 1..total (key=0 is trivially the identity)
        for key_int in 1..total {
            let key_bytes = int_to_byte_array(key_int, key_length);
            let result = bit_op_xor(sample, &key_bytes, null_preserving, scheme);

            // Use lossy UTF-8 conversion for crib matching
            let result_str = String::from_utf8_lossy(&result);

            if !raw_crib.is_empty() && !result_str.to_lowercase().contains(&raw_crib) {
                continue;
            }

            let mut record = String::new();
            if print_key {
                record.push_str(&format!(
                    "Key = {:0width$x}: ",
                    key_int,
                    width = 2 * key_length
                ));
            }

            if output_hex {
                record.push_str(&hex::encode(&result));
            } else {
                record.push_str(&escape_whitespace(&result_str));
            }

            output_lines.push(record);
        }

        Ok(output_lines.join("\n").into_bytes())
    }
}
