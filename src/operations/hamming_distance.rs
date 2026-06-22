/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Hamming Distance operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Hamming Distance operation
///
/// In information theory, the Hamming distance between two strings of equal
/// length is the number of positions at which the corresponding symbols are
/// different.
pub struct HammingDistance;

impl Operation for HammingDistance {
    fn name(&self) -> &'static str {
        "Hamming Distance"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "In information theory, the Hamming distance between two strings of equal length is the number of positions at which the corresponding symbols are different."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Delimiter between strings",
                default_value: "\\n\\n",
            },
            ArgSchema {
                name: "Unit",
                description: "Unit: Byte or Bit",
                default_value: "Byte",
            },
            ArgSchema {
                name: "Input type",
                description: "Input type: Raw string or Hex",
                default_value: "Raw string",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let delim = args.first().and_then(|a| a.as_str()).unwrap_or("\\n\\n");
        let by_byte = args.get(1).and_then(|a| a.as_str()).unwrap_or("Byte") == "Byte";
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw string");

        let strings: Vec<String> = input
            .split(|&b| {
                if delim == "\\n\\n" {
                    b == b'\n'
                } else {
                    b == delim.as_bytes()[0]
                }
            })
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect();

        let strings_ref: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

        if strings_ref.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Error: You can only calculate the edit distance between 2 strings. Please ensure exactly two inputs are provided, separated by the specified delimiter."
                    .to_string(),
            ));
        }

        if strings[0].len() != strings[1].len() {
            return Err(OperationError::InvalidInput(
                "Error: Both inputs must be of the same length.".to_string(),
            ));
        }

        let sample0 = if input_type == "Hex" {
            hex::decode(&strings[0])
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
        } else {
            strings[0].as_bytes().to_vec()
        };

        let sample1 = if input_type == "Hex" {
            hex::decode(&strings[1])
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
        } else {
            strings[1].as_bytes().to_vec()
        };

        let mut dist = 0;

        for i in 0..sample0.len() {
            let lhs = sample0[i];
            let rhs = sample1[i];

            if by_byte && lhs != rhs {
                dist += 1;
            } else if !by_byte {
                let xord = lhs ^ rhs;
                dist += xord.count_ones() as usize;
            }
        }

        Ok(dist.to_string().into_bytes())
    }
}
