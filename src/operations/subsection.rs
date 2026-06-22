/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Subsection operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Subsection operation - flow control.
/// In this Rust context, implemented as a passthrough since true flow control
/// requires recipe-level orchestration that is beyond a single-operation scope.
pub struct Subsection;

impl Operation for Subsection {
    fn name(&self) -> &'static str {
        "Subsection"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Select a part of the input data using a regular expression (regex), and run all subsequent operations on each match separately. In this implementation, acts as a passthrough (flow control requires recipe-level orchestration)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Section (regex)",
                description: "The regex to select the subsection with",
                default_value: "",
            },
            ArgSchema {
                name: "Case sensitive matching",
                description: "Whether the regex match should be case sensitive",
                default_value: "true",
            },
            ArgSchema {
                name: "Global matching",
                description: "Whether to match all occurrences",
                default_value: "true",
            },
            ArgSchema {
                name: "Ignore errors",
                description: "Whether to ignore errors in subsequent operations",
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
        // Passthrough: full subsection flow control requires recipe-level orchestration.
        Ok(input)
    }
}
