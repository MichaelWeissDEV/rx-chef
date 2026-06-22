/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Cartesian Product operation.
 * -----------------------------------------------------------------------------
 */

use itertools::Itertools;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Cartesian Product operation
pub struct CartesianProduct;

impl Operation for CartesianProduct {
    fn name(&self) -> &'static str {
        "Cartesian Product"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Calculates the cartesian product of multiple sets of data, returning all possible combinations."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Sample delimiter",
                description: "Delimiter between sets",
                default_value: "\\n\\n",
            },
            ArgSchema {
                name: "Item delimiter",
                description: "Delimiter between items in a set",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let sample_delim = if let Some(arg) = args.first() {
            parse_delimiter(arg.as_str().unwrap_or("\n\n"))
        } else {
            "\n\n".to_string()
        };

        let item_delim = if let Some(arg) = args.get(1) {
            parse_delimiter(arg.as_str().unwrap_or(","))
        } else {
            ",".to_string()
        };

        let sets: Vec<Vec<String>> = input_str
            .split(&sample_delim)
            .map(|s| s.split(&item_delim).map(|i| i.to_string()).collect())
            .collect();

        if sets.len() < 2 {
            return Err(OperationError::InvalidInput(
                "Incorrect number of sets, perhaps you need to modify the sample delimiter or add more samples?".to_string(),
            ));
        }

        // Use itertools to calculate cartesian product
        let product = sets.iter().multi_cartesian_product();

        let result = product
            .map(|set| format!("({})", set.into_iter().join(",")))
            .collect::<Vec<String>>()
            .join(&item_delim);

        Ok(result.into_bytes())
    }
}

fn parse_delimiter(delim: &str) -> String {
    delim
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
}
