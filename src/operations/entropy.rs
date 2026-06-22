/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Entropy operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Entropy operation
///
/// Calculates Shannon entropy of the input data.
/// Optionally calculates entropy in chunks (scanning entropy).
pub struct Entropy;

impl Operation for Entropy {
    fn name(&self) -> &'static str {
        "Entropy"
    }

    fn module(&self) -> &'static str {
        "Charts"
    }

    fn description(&self) -> &'static str {
        "Shannon Entropy, in the context of information theory, is a measure of the rate at which \
         information is produced by a source of data. 8 is the maximum, representing highly \
         unstructured, random data. English language text usually falls somewhere between 3.5 and \
         5. Properly encrypted or compressed data should have an entropy of over 7.5."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Chunk size",
            description:
                "Size of each chunk for scanning entropy. 0 means calculate for whole input.",
            default_value: "0",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let chunk_size = args.first().and_then(|a| a.as_usize()).unwrap_or(0);

        if chunk_size == 0 {
            let entropy = calculate_shannon_entropy(&input);
            Ok(format!("{:.17}", entropy).into_bytes())
        } else {
            let mut results = Vec::new();
            for chunk in input.chunks(chunk_size) {
                let entropy = calculate_shannon_entropy(chunk);
                results.push(format!("{:.17}", entropy));
            }
            Ok(results.join("\n").into_bytes())
        }
    }
}

/// Calculate Shannon entropy for a byte slice.
fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut occurrences = [0u64; 256];
    for &byte in data {
        occurrences[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0_f64;

    for &count in occurrences.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy += p * p.log2();
        }
    }

    -entropy
}
