/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Conditional Jump operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Conditional Jump operation
pub struct ConditionalJump;

impl Operation for ConditionalJump {
    fn name(&self) -> &'static str {
        "Conditional Jump"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Conditionally jump forwards or backwards to a Label when the current data matches a regular expression. Backwards jumps are bounded by the configured maximum. Interpreted by integration::bake and all CLI recipe frontends."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Match (regex)",
                description: "The regular expression to match against the data",
                default_value: "",
            },
            ArgSchema {
                name: "Invert match",
                description: "If true, jump when the regex does NOT match",
                default_value: "false",
            },
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
        // A standalone operation has no recipe program counter to change.
        Ok(input)
    }
}
