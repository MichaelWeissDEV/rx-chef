/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Base92 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Base92 operation
pub struct FromBase92;

impl Operation for FromBase92 {
    fn name(&self) -> &'static str {
        "From Base92"
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
        let input_str = String::from_utf8_lossy(&input);

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let mut res = Vec::new();
        let mut bit_string = String::new();

        let chars: Vec<char> = input_str.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if i + 1 < chars.len() {
                let x1 = base92_ord(chars[i])?;
                let x2 = base92_ord(chars[i + 1])?;
                let x = x1 as u32 * 91 + x2 as u32;
                bit_string.push_str(&format!("{:013b}", x));
                i += 2;
            } else {
                let x = base92_ord(chars[i])?;
                bit_string.push_str(&format!("{:06b}", x));
                i += 1;
            }
            while bit_string.len() >= 8 {
                let byte = u8::from_str_radix(&bit_string[..8], 2).unwrap();
                res.push(byte);
                bit_string = bit_string[8..].to_string();
            }
        }

        Ok(res)
    }
}

fn base92_ord(c: char) -> Result<u8, OperationError> {
    if c == '!' {
        Ok(0)
    } else if c >= '#' && c <= '_' {
        Ok((c as u32 - '#' as u32 + 1) as u8)
    } else if c >= 'a' && c <= '}' {
        Ok((c as u32 - 'a' as u32 + 62) as u8)
    } else {
        Err(OperationError::InvalidInput(format!(
            "{} is not a base92 character",
            c
        )))
    }
}
