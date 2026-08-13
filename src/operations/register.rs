/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Register operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Register operation
pub struct Register;

impl Operation for Register {
    fn name(&self) -> &'static str {
        "Register"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extract data from the input into recipe registers using regular expression capture groups. Refer to captures in later operation arguments as $R0, $R1, and so on. Register expansion is implemented by integration::bake and all CLI recipe frontends."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Extractor",
                description: "Regular expression capture groups",
                default_value: "([\\s\\S]*)",
            },
            ArgSchema {
                name: "Case insensitive",
                description: "Case insensitive matching",
                default_value: "true",
            },
            ArgSchema {
                name: "Multiline matching",
                description: "Multiline matching",
                default_value: "false",
            },
            ArgSchema {
                name: "Dot matches all",
                description: "Dot matches all",
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
        // A standalone operation has no later recipe arguments to expand.
        Ok(input)
    }
}
