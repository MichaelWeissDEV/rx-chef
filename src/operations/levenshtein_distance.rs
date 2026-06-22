/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Levenshtein Distance operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Levenshtein Distance operation
///
/// Calculates the Levenshtein edit distance between two strings split by a delimiter.
/// Supports custom insertion, deletion, and substitution costs.
pub struct LevenshteinDistance;

impl Operation for LevenshteinDistance {
    fn name(&self) -> &'static str {
        "Levenshtein Distance"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Levenshtein Distance (also known as Edit Distance) is a string metric to measure a \
         difference between two strings that counts operations (insertions, deletions, and \
         substitutions) on single character that are required to change one string to another."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample delimiter",
                description: "Delimiter separating the two input samples",
                default_value: "\\n",
            },
            ArgSchema {
                name: "Insertion cost",
                description: "Cost of inserting a character",
                default_value: "1",
            },
            ArgSchema {
                name: "Deletion cost",
                description: "Cost of deleting a character",
                default_value: "1",
            },
            ArgSchema {
                name: "Substitution cost",
                description: "Cost of substituting a character",
                default_value: "1",
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
        let input_str = String::from_utf8_lossy(&input).to_string();

        let delim = args.first().and_then(|a| a.as_str()).unwrap_or("\\n");
        let ins_cost = args.get(1).and_then(|a| a.as_f64()).unwrap_or(1.0);
        let del_cost = args.get(2).and_then(|a| a.as_f64()).unwrap_or(1.0);
        let sub_cost = args.get(3).and_then(|a| a.as_f64()).unwrap_or(1.0);

        // Validate costs
        if ins_cost < 0.0 || del_cost < 0.0 || sub_cost < 0.0 {
            return Err(OperationError::InvalidInput(
                "Negative costs are not allowed.".to_string(),
            ));
        }

        // Split input by delimiter
        let samples: Vec<&str> = input_str.splitn(3, delim).collect();
        if samples.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of samples. Check your input and/or delimiter.".to_string(),
            ));
        }

        let src: Vec<char> = samples[0].chars().collect();
        let dest: Vec<char> = samples[1].chars().collect();

        let ins = ins_cost as u64;
        let del = del_cost as u64;
        let sub = sub_cost as u64;

        let distance = compute_distance(&src, &dest, ins, del, sub);

        Ok(format!("{}", distance).into_bytes())
    }
}

/// Compute the weighted Levenshtein distance between two character slices.
fn compute_distance(
    src: &[char],
    dest: &[char],
    ins_cost: u64,
    del_cost: u64,
    sub_cost: u64,
) -> u64 {
    let m = src.len();
    let n = dest.len();

    let mut current_cost: Vec<u64> = (0..=(m as u64)).map(|i| del_cost * i).collect();
    let mut next_cost: Vec<u64> = vec![0; m + 1];

    for i in 0..n {
        let destc = dest[i];
        next_cost[0] = current_cost[0] + ins_cost;

        for j in 0..m {
            // Insertion from current column
            let mut opt = current_cost[j + 1] + ins_cost;

            // Deletion from next row
            let candidate = next_cost[j] + del_cost;
            if candidate < opt {
                opt = candidate;
            }

            // Substitution or match
            let candidate = current_cost[j] + if src[j] != destc { sub_cost } else { 0 };
            if candidate < opt {
                opt = candidate;
            }

            next_cost[j + 1] = opt;
        }

        std::mem::swap(&mut current_cost, &mut next_cost);
    }

    current_cost[m]
}
