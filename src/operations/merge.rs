/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Merge operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Merge operation
pub struct Merge;

impl Operation for Merge {
    fn name(&self) -> &'static str {
        "Merge"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Consolidate all branches back into a single trunk. The opposite of Fork. Unticking the Merge All checkbox will only consolidate all branches up to the nearest Fork/Subsection."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Merge All",
            description: "Consolidate all branches back into a single trunk.",
            default_value: "true",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        // No need to actually do anything here. The fork operation will
        // merge when it sees this operation.
        Ok(input)
    }
}
