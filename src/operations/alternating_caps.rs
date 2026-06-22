/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Alternating Caps operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Alternating caps operation
///
/// Alternating caps, also known as studly caps, sticky caps, or spongecase
/// is a form of text notation in which the capitalization of letters varies
/// by some pattern, or arbitrarily.
pub struct AlternatingCaps;

impl Operation for AlternatingCaps {
    fn name(&self) -> &'static str {
        "Alternating Caps"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Alternating caps, also known as studly caps, sticky caps, or spongecase is a form of text notation in which the capitalization of letters varies by some pattern, or arbitrarily. An example of this would be spelling 'alternative caps' as 'aLtErNaTiNg CaPs'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let mut output = String::new();
        let mut previous_caps = true;

        for c in input_str.chars() {
            if !c.is_alphabetic() {
                output.push(c);
            } else if previous_caps {
                output.push(c.to_lowercase().next().unwrap_or(c));
                previous_caps = false;
            } else {
                output.push(c.to_uppercase().next().unwrap_or(c));
                previous_caps = true;
            }
        }

        Ok(output.into_bytes())
    }
}
