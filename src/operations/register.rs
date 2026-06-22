/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
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
        "Extract data from the input and store it in registers which can then be passed into subsequent operations as arguments. Regular expression capture groups are used to select the data to extract.<br><br>To use registers in arguments, refer to them using the notation <code>$Rn</code> where n is the register number, starting at 0.<br><br>In this implementation, acts as a passthrough (flow control requires recipe-level orchestration)."
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
        // Passthrough: full register flow control requires recipe-level orchestration.
        Ok(input)
    }
}
