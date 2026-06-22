/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Remove line numbers operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Remove line numbers operation
pub struct RemoveLineNumbers;

impl Operation for RemoveLineNumbers {
    fn name(&self) -> &'static str {
        "Remove line numbers"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Removes line numbers from the output if they can be trivially detected."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // CyberChef regex: /^[ \t]{0,5}\d+[\s:|\-,.)\]]/gm
        // In Rust regex, we use (?m) for multiline mode.
        let re = Regex::new(r"(?m)^[ \t]{0,5}\d+[\s:|\-,.)\]]").unwrap();
        let result = re.replace_all(&input_str, "");

        Ok(result.into_owned().into_bytes())
    }
}
