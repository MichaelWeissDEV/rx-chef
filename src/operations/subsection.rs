/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Subsection operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Subsection operation - interpreted by the shared recipe engine.
pub struct Subsection;

impl Operation for Subsection {
    fn name(&self) -> &'static str {
        "Subsection"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Select input sections using a regular expression and run subsequent recipe operations on every match until Merge. Non-matching bytes remain unchanged. Interpreted by integration::bake and all CLI recipe frontends."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Section (regex)",
                description: "The regex to select the subsection with",
                default_value: "",
                kind: crate::operation::ArgKind::Regex,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Case sensitive matching",
                description: "Whether the regex match should be case sensitive",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Global matching",
                description: "Whether to match all occurrences",
                default_value: "true",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Ignore errors",
                description: "Whether to ignore errors in subsequent operations",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
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
        // A standalone operation has no following recipe to apply to matches.
        Ok(input)
    }
}
