/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Median operation.
 * -----------------------------------------------------------------------------
 */

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::divide::{create_num_array, format_number},
};

/// Median operation
///
/// Computes the median of a number list. If an item in the string is not a number
/// it is excluded from the list.  e.g. `0x0a 8 1 .5` becomes `4.5`
pub struct Median;

impl Operation for Median {
    fn name(&self) -> &'static str {
        "Median"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Computes the median of a number list. If an item in the string is not a number it is excluded from the list. e.g. 0x0a 8 1 .5 becomes 4.5"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Delimiter",
            description: "Character that separates numbers in the input",
            default_value: "Line feed",
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
        let delim = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let input_str = String::from_utf8_lossy(&input);
        let mut nums = create_num_array(&input_str, delim);

        if nums.is_empty() {
            return Ok(b"NaN".to_vec());
        }

        // Sort ascending (total_cmp is safe for finite f64 values)
        nums.sort_by(|a, b| a.total_cmp(b));

        let len = nums.len();
        let result = if len % 2 == 0 {
            // Even count: mean of the two middle values
            let first = nums[len / 2];
            let second = nums[len / 2 - 1];
            (first + second) / 2.0
        } else {
            nums[len / 2]
        };

        Ok(format_number(result).into_bytes())
    }
}
