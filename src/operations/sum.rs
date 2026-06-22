/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Sum operation.
 * -----------------------------------------------------------------------------
 */

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::divide::{create_num_array, format_number},
};

/// Sum operation
///
/// Adds together a list of numbers. If an item in the string is not a number it
/// is excluded from the list.  e.g. `0x0a 8 .5` becomes `18.5`
pub struct Sum;

impl Operation for Sum {
    fn name(&self) -> &'static str {
        "Sum"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Adds together a list of numbers. If an item in the string is not a number it is excluded from the list. e.g. 0x0a 8 .5 becomes 18.5"
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

        let result: f64 = nums.iter().sum();
        Ok(format_number(result).into_bytes())
    }
}
