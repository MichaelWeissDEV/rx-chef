/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Fang URL operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Fang URL operation
///
/// Takes a 'Defanged' Universal Resource Locator (URL) and 'Fangs' it.
/// Meaning, it removes the alterations (defanged) that render it useless
/// so that it can be used again.
pub struct FangURL;

impl Operation for FangURL {
    fn name(&self) -> &'static str {
        "Fang URL"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Takes a 'Defanged' Universal Resource Locator (URL) and 'Fangs' it. Meaning, it removes the alterations (defanged) that render it useless so that it can be used again."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Restore [.]",
                description: "Restore [.] to .",
                default_value: "true",
            },
            ArgSchema {
                name: "Restore hxxp",
                description: "Restore hxxp to http",
                default_value: "true",
            },
            ArgSchema {
                name: "Restore ://",
                description: "Restore [://] to ://",
                default_value: "true",
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
        let restore_dots = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("true")
            .to_lowercase()
            != "false";
        let restore_http = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("true")
            .to_lowercase()
            != "false";
        let restore_slashes = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("true")
            .to_lowercase()
            != "false";

        let input_str = String::from_utf8_lossy(&input);
        let mut result = input_str.into_owned();

        if restore_dots {
            result = result.replace("[.]", ".");
        }
        if restore_http {
            result = result.replace("hxxp", "http").replace("hxxps", "https");
        }
        if restore_slashes {
            result = result.replace("[://]", "://");
        }

        Ok(result.into_bytes())
    }
}
