/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Index of Coincidence operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Index of Coincidence operation
///
/// Index of Coincidence (IC) is the probability of two randomly selected
/// characters being the same.
pub struct IndexOfCoincidence;

impl Operation for IndexOfCoincidence {
    fn name(&self) -> &'static str {
        "Index of Coincidence"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Index of Coincidence (IC) is the probability of two randomly selected characters being the same."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Number
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let text: String = input_str
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let mut frequencies = [0u32; 26];
        let _alphabet: Vec<char> = ('a'..='z').collect();

        for c in text.chars() {
            let idx = (c as u8 - b'a') as usize;
            frequencies[idx] += 1;
        }

        let mut coincidence = 0.0;
        let mut density = 0.0;

        for &count in &frequencies {
            coincidence += (count as f64) * ((count as f64) - 1.0);
            density += count as f64;
        }

        // Ensure that we don't divide by 0
        if density < 2.0 {
            density = 2.0;
        }

        let result = coincidence / (density * (density - 1.0));

        Ok(result.to_le_bytes().to_vec())
    }
}
