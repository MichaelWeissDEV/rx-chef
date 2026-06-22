/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Set Intersection operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Set Intersection operation
///
/// Calculates the intersection of two sets.
/// Input must contain exactly two samples separated by the sample delimiter.
pub struct SetIntersection;

fn unescape_str(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\0", "\0")
}

impl Operation for SetIntersection {
    fn name(&self) -> &'static str {
        "Set Intersection"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates the intersection of two sets."
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

        let result: Vec<&str> = set_a
            .iter()
            .filter(|item| set_b.contains(item))
            .copied()
            .collect();

        let output = result.join(item_delim_str.as_str());
        Ok(output.into_bytes())
    }
}
