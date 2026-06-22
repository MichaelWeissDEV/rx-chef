/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parity Bit operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parity Bit operation
///
/// A parity bit, or check bit, is the simplest form of error detection.
pub struct ParityBit;

impl Operation for ParityBit {
    fn name(&self) -> &'static str {
        "Parity Bit"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "A parity bit, or check bit, is the simplest form of error detection. It is a bit which is added to a string of bits and represents if the number of 1's in the binary string is an even number or odd number."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Mode",
                description: "Even or Odd parity",
                default_value: "Even Parity",
            },
            ArgSchema {
                name: "Position",
                description: "Start or End",
                default_value: "Start",
            },
            ArgSchema {
                name: "Encode or Decode",
                description: "Encode or Decode",
                default_value: "Encode",
            },
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter for blocks",
                default_value: "",
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
        if input.is_empty() {
            return Ok(input);
        }

        let mode = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("Even Parity");
        let position = args.get(1).and_then(|a| a.as_str()).unwrap_or("Start");
        let operation = args.get(2).and_then(|a| a.as_str()).unwrap_or("Encode");
        let delimiter = args.get(3).and_then(|a| a.as_str()).unwrap_or("");

        if !delimiter.is_empty() {
            let blocks: Vec<&[u8]> = input.split(|&b| b == delimiter.as_bytes()[0]).collect();
            let mut results = Vec::new();

            for block in &blocks {
                let result = self.process_block(block, mode, position, operation)?;
                results.extend_from_slice(&result);
                if !delimiter.is_empty() {
                    results.push(delimiter.as_bytes()[0]);
                }
            }

            // Remove trailing delimiter
            if !results.is_empty() && results[results.len() - 1] == delimiter.as_bytes()[0] {
                results.pop();
            }

            return Ok(results);
        }

        self.process_block(&input, mode, position, operation)
    }
}

impl ParityBit {
    fn process_block(
        &self,
        input: &[u8],
        mode: &str,
        position: &str,
        operation: &str,
    ) -> Result<Vec<u8>, OperationError> {
        if operation == "Encode" {
            self.encode_parity(input, mode, position)
        } else {
            self.decode_parity(input, mode, position)
        }
    }

    fn encode_parity(
        &self,
        input: &[u8],
        mode: &str,
        position: &str,
    ) -> Result<Vec<u8>, OperationError> {
        let odd_parity = mode == "Odd Parity";
        let mut output = Vec::new();

        for &byte in input {
            let ones = byte.count_ones();
            let parity = (ones % 2) == 1;

            let parity_bit = if odd_parity { !parity } else { parity } as u8;

            if position == "Start" {
                output.push(parity_bit);
            }
            output.push(byte);
            if position == "End" {
                output.push(parity_bit);
            }
        }

        Ok(output)
    }

    fn decode_parity(
        &self,
        input: &[u8],
        mode: &str,
        position: &str,
    ) -> Result<Vec<u8>, OperationError> {
        let odd_parity = mode == "Odd Parity";
        let mut output = Vec::new();
        let mut i = 0;

        while i < input.len() {
            let parity_bit = if position == "Start" && i < input.len() {
                input[i]
            } else {
                0
            };

            i += 1;

            if i >= input.len() {
                break;
            }

            let byte = input[i];
            i += 1;

            let ones = byte.count_ones();
            let actual_parity = (ones % 2) == 1;

            let expected_parity = if odd_parity {
                !actual_parity
            } else {
                actual_parity
            };

            if parity_bit as u32 == expected_parity as u32 {
                output.push(byte);
            }
            // Skip byte if parity doesn't match
        }

        Ok(output)
    }
}
