/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Remove whitespace operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Remove whitespace operation
///
/// Optionally removes all spaces, carriage returns, line feeds, tabs and
/// form feeds from the input data. This operation also supports the removal
/// of full stops which are sometimes used to represent non-printable bytes
/// in ASCII output.
pub struct RemoveWhitespace;

impl Operation for RemoveWhitespace {
    fn name(&self) -> &'static str {
        "Remove whitespace"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Optionally removes all spaces, carriage returns, line feeds, tabs and form feeds from the input data. This operation also supports the removal of full stops which are sometimes used to represent non-printable bytes in ASCII output."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Spaces",
                description: "Remove spaces",
                default_value: "true",
            },
            ArgSchema {
                name: "Carriage returns (\\r)",
                description: "Remove carriage returns",
                default_value: "true",
            },
            ArgSchema {
                name: "Line feeds (\\n)",
                description: "Remove line feeds",
                default_value: "true",
            },
            ArgSchema {
                name: "Tabs",
                description: "Remove tabs",
                default_value: "true",
            },
            ArgSchema {
                name: "Form feeds (\\f)",
                description: "Remove form feeds",
                default_value: "true",
            },
            ArgSchema {
                name: "Full stops",
                description: "Remove full stops",
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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let remove_spaces = args.first().and_then(|a| a.as_bool()).unwrap_or(true);
        let remove_cr = args.get(1).and_then(|a| a.as_bool()).unwrap_or(true);
        let remove_lf = args.get(2).and_then(|a| a.as_bool()).unwrap_or(true);
        let remove_tabs = args.get(3).and_then(|a| a.as_bool()).unwrap_or(true);
        let remove_ff = args.get(4).and_then(|a| a.as_bool()).unwrap_or(true);
        let remove_full_stops = args.get(5).and_then(|a| a.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8_lossy(&input);
        let mut result = input_str.to_string();

        if remove_spaces {
            result = result.replace(' ', "");
        }
        if remove_cr {
            result = result.replace('\r', "");
        }
        if remove_lf {
            result = result.replace('\n', "");
        }
        if remove_tabs {
            result = result.replace('\t', "");
        }
        if remove_ff {
            result = result.replace('\x0c', "");
        }
        if remove_full_stops {
            result = result.replace('.', "");
        }

        Ok(result.into_bytes())
    }
}
