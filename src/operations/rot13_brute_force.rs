/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ROT13 Brute Force operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ROT13BruteForce;

fn rot_by(input: &str, amount: u8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + amount) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + amount) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect()
}

impl Operation for ROT13BruteForce {
    fn name(&self) -> &'static str {
        "ROT13 Brute Force"
    }
    fn module(&self) -> &'static str {
        "Ciphers"
    }
    fn description(&self) -> &'static str {
        "Tries all 25 rotation values for ROT13 and outputs each result."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static S: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample length",
                description: "Max characters from each result to show",
                default_value: "100",
            },
            ArgSchema {
                name: "Sample offset",
                description: "Start offset in input",
                default_value: "0",
            },
            ArgSchema {
                name: "Print spaces",
                description: "Print spaces between results",
                default_value: "true",
            },
            ArgSchema {
                name: "Print letter score",
                description: "Include letter frequency score",
                default_value: "false",
            },
        ];
        S
    }
    fn input_type(&self) -> DataType {
        DataType::String
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let sample_len: usize = args
            .get(0)
            .and_then(|a| a.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);
        let text = String::from_utf8_lossy(&input);
        let mut output = String::new();
        for i in 1u8..26 {
            let rotated = rot_by(&text, i);
            let sample: String = rotated.chars().take(sample_len).collect();
            output.push_str(&format!("ROT{:02}: {}\n", i, sample));
        }
        Ok(output.into_bytes())
    }
}
