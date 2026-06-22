/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base45 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToBase45;

impl Operation for ToBase45 {
    fn name(&self) -> &'static str {
        "To Base45"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Base45 is optimized for usage with QR codes."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Alphabet",
            description: "The Base45 alphabet",
            default_value: "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:",
        }];
        SCHEMA
    }
    fn input_type(&self) -> DataType {
        DataType::Bytes
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }
    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:".as_bytes();
        let mut output = String::new();
        for chunk in input.chunks(2) {
            let val = if chunk.len() == 2 {
                (chunk[0] as u32) << 8 | chunk[1] as u32
            } else {
                chunk[0] as u32
            };
            let c = val % 45;
            let d = (val / 45) % 45;
            let e = val / (45 * 45);
            output.push(alphabet[c as usize] as char);
            output.push(alphabet[d as usize] as char);
            if chunk.len() == 2 {
                output.push(alphabet[e as usize] as char);
            }
        }
        Ok(output.into_bytes())
    }
}
