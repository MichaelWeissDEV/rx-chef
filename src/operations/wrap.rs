/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Wrap operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Wrap operation
pub struct WrapOp;

impl Operation for WrapOp {
    fn name(&self) -> &'static str {
        "Wrap"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Wraps the input text at a specified number of characters per line."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Line Width",
            description: "Number of characters per line",
            default_value: "64",
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
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let line_width = args.first().and_then(|a| a.as_usize()).unwrap_or(64);
        if line_width == 0 {
            return Ok(input);
        }

        let input_str = String::from_utf8_lossy(&input);

        // Replicating JS behavior: . match any char EXCEPT newline.
        // Regex in Rust: . does not match \n by default.
        let pattern = format!(".{{1,{}}}", line_width);
        let re =
            Regex::new(&pattern).map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let matches: Vec<&str> = re.find_iter(&input_str).map(|m| m.as_str()).collect();
        let result = matches.join("\n");

        Ok(result.into_bytes())
    }
}
