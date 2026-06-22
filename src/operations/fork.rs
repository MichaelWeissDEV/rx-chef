/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Fork operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Fork operation - flow control. In this Rust context, implemented as a
/// passthrough since true fork/merge flow control requires recipe-level
/// orchestration that is beyond a single-operation scope.
pub struct Fork;

impl Operation for Fork {
    fn name(&self) -> &'static str {
        "Fork"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Split the input data up based on the specified delimiter and run all subsequent operations on each branch separately. In this implementation, acts as a passthrough (flow control requires recipe-level orchestration)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Split delimiter",
                description: "The delimiter to split the input on",
                default_value: "\\n",
            },
            ArgSchema {
                name: "Merge delimiter",
                description: "The delimiter to join outputs with",
                default_value: "\\n",
            },
            ArgSchema {
                name: "Ignore errors",
                description: "Continue processing even if a branch fails",
                default_value: "false",
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
        // Passthrough: full fork/merge requires recipe-level orchestration.
        Ok(input)
    }
}
