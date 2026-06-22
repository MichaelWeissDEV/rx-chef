/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Symmetric Difference operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Symmetric Difference operation
///
/// Calculates the symmetric difference of two sets: elements in either
/// set but not in both.
pub struct SymmetricDifference;

fn unescape_str(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\0", "\0")
}

impl Operation for SymmetricDifference {
    fn name(&self) -> &'static str {
        "Symmetric Difference"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates the symmetric difference of two sets."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample delimiter",
                description: "Delimiter separating the two input sets",
                default_value: "\\n\\n",
            },
            ArgSchema {
                name: "Item delimiter",
                description: "Delimiter separating items within each set",
                default_value: ",",
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
        let text = String::from_utf8_lossy(&input).into_owned();

        let sample_delim = args.first().and_then(|a| a.as_str()).unwrap_or("\\n\\n");
        let item_delim = args.get(1).and_then(|a| a.as_str()).unwrap_or(",");

        let sample_delim = unescape_str(sample_delim);
        let item_delim_str = unescape_str(item_delim);

        let parts: Vec<&str> = text.splitn(2, sample_delim.as_str()).collect();
        if parts.len() != 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of sets, perhaps you need to modify the sample delimiter or add more samples?".to_string(),
            ));
        }

        let set_a: Vec<&str> = parts[0].split(item_delim_str.as_str()).collect();
        let set_b: Vec<&str> = parts[1].split(item_delim_str.as_str()).collect();

        // Elements in a but not in b
        let diff_ab: Vec<&str> = set_a
            .iter()
            .filter(|i| !set_b.contains(i))
            .copied()
            .collect();
        // Elements in b but not in a
        let diff_ba: Vec<&str> = set_b
            .iter()
            .filter(|i| !set_a.contains(i))
            .copied()
            .collect();

        let mut result = diff_ab;
        result.extend(diff_ba);

        let output = result.join(item_delim_str.as_str());
        Ok(output.into_bytes())
    }
}
