/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Mean operation.
 * -----------------------------------------------------------------------------
 */

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::divide::{create_num_array, format_number},
};

/// Mean operation
///
/// Computes the mean (average) of a number list. If an item in the string is not
/// a number it is excluded from the list.  e.g. `0x0a 8 .5 .5` becomes `4.75`
pub struct Mean;

impl Operation for Mean {
    fn name(&self) -> &'static str {
        "Mean"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Computes the mean (average) of a number list. If an item in the string is not a number it is excluded from the list. e.g. 0x0a 8 .5 .5 becomes 4.75"
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

        let total: f64 = nums.iter().sum();
        let result = total / nums.len() as f64;
        Ok(format_number(result).into_bytes())
    }
}
