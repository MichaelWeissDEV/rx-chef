/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ROT47 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ROT47;

impl Operation for ROT47 {
    fn name(&self) -> &'static str {
        "ROT47"
    }
    fn module(&self) -> &'static str {
        "Ciphers"
    }
    fn description(&self) -> &'static str {
        "Substitutes characters in the printable ASCII range (0x21-0x7E) by rotating each by 47 positions."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }
    fn input_type(&self) -> DataType {
        DataType::String
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = String::from_utf8_lossy(&input);
        let result: String = text
            .chars()
            .map(|c| {
                if c as u32 >= 0x21 && c as u32 <= 0x7e {
                    let rot = (c as u8 - 0x21 + 47) % 94 + 0x21;
                    rot as char
                } else {
                    c
                }
            })
            .collect();
        Ok(result.into_bytes())
    }
}
