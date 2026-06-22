/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Power Set operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Power Set operation
///
/// Calculates all the subsets of a set.
pub struct PowerSet;

impl Operation for PowerSet {
    fn name(&self) -> &'static str {
        "Power Set"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates all the subsets of a set."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Item delimiter",
            description: "Delimiter between items",
            default_value: ",",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let item_delimiter = args.first().and_then(|a| a.as_str()).unwrap_or(",");

        let input_str = String::from_utf8_lossy(&input);
        let items: Vec<&str> = input_str
            .split(item_delimiter)
            .filter(|s| !s.is_empty())
            .collect();

        if items.is_empty() {
            return Ok(b"".to_vec());
        }

        let mut items_vec: Vec<String> = items.iter().map(|s| s.to_string()).collect();
        let subsets = self.generate_power_set(&mut items_vec, item_delimiter);
        Ok(subsets)
    }
}

impl PowerSet {
    fn generate_power_set(&self, items: &mut [String], item_delimiter: &str) -> Vec<u8> {
        if items.is_empty() {
            return Vec::new();
        }

        let n = items.len();
        let total = 1 << n;
        let mut subsets = Vec::new();

        for i in 0..total {
            let mut subset = Vec::new();
            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    subset.push(items[j].clone());
                }
            }

            if !subset.is_empty() {
                subsets.push(subset.join(item_delimiter));
            }
        }

        // Sort by length
        subsets.sort_by_key(|s| s.len());

        // Format output
        let output = subsets
            .into_iter()
            .map(|s| format!("{}\n", s))
            .collect::<String>();

        output.into_bytes()
    }
}
