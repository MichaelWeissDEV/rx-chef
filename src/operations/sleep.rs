/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Sleep operation.
 * -----------------------------------------------------------------------------
 */

use std::{thread, time::Duration};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Sleep operation
pub struct Sleep;

impl Operation for Sleep {
    fn name(&self) -> &'static str {
        "Sleep"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Sleep causes the recipe to wait for a specified number of milliseconds before continuing execution."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Time (ms)",
            description: "Number of milliseconds to sleep",
            default_value: "1000",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let ms = args.first().and_then(|a| a.as_f64()).unwrap_or(1000.0) as u64;
        thread::sleep(Duration::from_millis(ms));
        Ok(input)
    }
}
