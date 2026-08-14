/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Jump operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Jump operation
pub struct Jump;

impl Operation for Jump {
    fn name(&self) -> &'static str {
        "Jump"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Jump forwards or backwards to a Label in a recipe. Backwards jumps are bounded by the configured maximum. Interpreted by integration::bake and all CLI recipe frontends."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Label name",
                description: "The name of the label to jump to",
                default_value: "",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Maximum jumps (if jumping backwards)",
                description:
                    "The maximum number of times to jump backwards to prevent infinite loops",
                default_value: "10",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
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

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        // A standalone operation has no recipe program counter to change.
        Ok(input)
    }
}
