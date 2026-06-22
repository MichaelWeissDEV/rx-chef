/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ROT47 Brute Force operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ROT47 Brute Force operation.
///
/// Tries all 93 meaningful rotation amounts for ROT47 (printable ASCII rotation
/// over codepoints 33-126). Optionally filters results by a known plaintext crib.
pub struct ROT47BruteForce;

fn rot47_by(data: &[u8], amount: u8) -> Vec<u8> {
    data.iter()
        .map(|&b| {
            if (33..=126).contains(&b) {
                (b - 33 + amount) % 94 + 33
            } else {
                b
            }
        })
        .collect()
}

impl Operation for ROT47BruteForce {
    fn name(&self) -> &'static str {
        "ROT47 Brute Force"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Try all meaningful amounts for ROT47. Optionally you can enter your known plaintext (crib) to filter the result."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample length",
                description: "Number of bytes to sample from input",
                default_value: "100",
            },
            ArgSchema {
                name: "Sample offset",
                description: "Byte offset to start sampling",
                default_value: "0",
            },
            ArgSchema {
                name: "Print amount",
                description: "Prefix each result with its rotation amount (true/false)",
                default_value: "true",
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
        let sample_len: usize = args
            .get(0)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.first()
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(100);
        let sample_offset: usize = args
            .get(1)
            .and_then(|a| a.as_usize())
            .or_else(|| {
                args.get(1)
                    .and_then(|a| a.as_str())
                    .and_then(|s| s.parse().ok())
            })
            .unwrap_or(0);
        let print_amount = args
            .get(2)
            .and_then(|a| a.as_bool())
            .or_else(|| {
                args.get(2)
                    .and_then(|a| a.as_str())
                    .map(|s| s.eq_ignore_ascii_case("true"))
            })
            .unwrap_or(true);
        let crib = args
            .get(3)
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .to_lowercase();

        let end = (sample_offset + sample_len).min(input.len());
        let sample = if sample_offset < input.len() {
            &input[sample_offset..end]
        } else {
            &input[0..0]
        };

        let mut results: Vec<String> = Vec::new();

        for amount in 1u8..94 {
            let rotated = rot47_by(sample, amount);
            let rotated_str = String::from_utf8_lossy(&rotated).to_string();
            if crib.is_empty() || rotated_str.to_lowercase().contains(&crib) {
                if print_amount {
                    results.push(format!("Amount = {:2}: {}", amount, rotated_str));
                } else {
                    results.push(rotated_str);
                }
            }
        }

        Ok(results.join("\n").into_bytes())
    }
}
