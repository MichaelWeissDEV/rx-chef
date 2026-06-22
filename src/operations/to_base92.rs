/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base92 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Base92 operation
pub struct ToBase92;

impl Operation for ToBase92 {
    fn name(&self) -> &'static str {
        "To Base92"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Base92 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let mut res = Vec::new();
        let mut bit_string = String::new();
        let mut input_iter = input.into_iter();

        loop {
            while bit_string.len() < 13 {
                if let Some(byte) = input_iter.next() {
                    bit_string.push_str(&format!("{:08b}", byte));
                } else {
                    break;
                }
            }

            if bit_string.len() < 13 {
                break;
            }

            let i = u32::from_str_radix(&bit_string[..13], 2).unwrap();
            res.push(base92_chr((i / 91) as u8)?);
            res.push(base92_chr((i % 91) as u8)?);
            bit_string = bit_string[13..].to_string();
        }

        if !bit_string.is_empty() {
            if bit_string.len() < 7 {
                while bit_string.len() < 6 {
                    bit_string.push('0');
                }
                let i = u8::from_str_radix(&bit_string[..6], 2).unwrap();
                res.push(base92_chr(i)?);
            } else {
                while bit_string.len() < 13 {
                    bit_string.push('0');
                }
                let i = u32::from_str_radix(&bit_string[..13], 2).unwrap();
                res.push(base92_chr((i / 91) as u8)?);
                res.push(base92_chr((i % 91) as u8)?);
            }
        }

        Ok(res)
    }
}

fn base92_chr(val: u8) -> Result<u8, OperationError> {
    if val >= 91 {
        return Err(OperationError::ProcessingError(
            "Invalid base92 value".to_string(),
        ));
    }
    if val == 0 {
        Ok(b'!')
    } else if val <= 61 {
        Ok(b'#' + val - 1)
    } else {
        Ok(b'a' + val - 62)
    }
}
