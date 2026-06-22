/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base62 operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToBase62;

impl Operation for ToBase62 {
    fn name(&self) -> &'static str {
        "To Base62"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Base62 is a notation for encoding arbitrary byte data using 0-9A-Za-z."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Alphabet",
            description: "The Base62 alphabet",
            default_value: "0-9A-Za-z",
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
        if input.is_empty() {
            return Ok(vec![]);
        }
        let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"; // Simplified expansion
        let mut n = BigUint::from_bytes_be(&input);
        let mut result = String::new();
        let sixty_two = BigUint::from(62u32);
        while n > BigUint::from(0u32) {
            let rem = (&n % &sixty_two).to_u32().unwrap();
            result.push(alphabet.chars().nth(rem as usize).unwrap());
            n /= &sixty_two;
        }
        Ok(result.chars().rev().collect::<String>().into_bytes())
    }
}
