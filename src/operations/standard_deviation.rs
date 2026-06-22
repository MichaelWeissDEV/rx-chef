/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Standard Deviation operation.
 * -----------------------------------------------------------------------------
 */

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::divide::{create_num_array, format_number},
};

/// Standard Deviation operation
///
/// Computes the standard deviation of a number list. If an item in the string is
/// not a number it is excluded from the list.
/// e.g. `0x0a 8 .5` becomes `4.089281382128433`
pub struct StandardDeviation;

impl Operation for StandardDeviation {
    fn name(&self) -> &'static str {
        "Standard Deviation"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Computes the standard deviation of a number list. If an item in the string is not a number it is excluded from the list. e.g. 0x0a 8 .5 becomes 4.089281382128433"
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
        let nums = create_num_array(&input_str, delim);

        if nums.is_empty() {
            return Ok(b"NaN".to_vec());
        }

        let n = nums.len() as f64;
        let avg: f64 = nums.iter().sum::<f64>() / n;
        let dev_sum: f64 = nums.iter().map(|x| (x - avg).powi(2)).sum();
        let result = (dev_sum / n).sqrt();

        Ok(format_number(result).into_bytes())
    }
}
