/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
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
        "Jump forwards or backwards to the specified Label. In this Rust implementation, it acts as a passthrough since flow control requires recipe-level orchestration."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Label name",
                description: "The name of the label to jump to",
                default_value: "",
            },
            ArgSchema {
                name: "Maximum jumps (if jumping backwards)",
                description:
                    "The maximum number of times to jump backwards to prevent infinite loops",
                default_value: "10",
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
        // Passthrough: full jump flow control requires recipe-level orchestration.
        Ok(input)
    }
}
