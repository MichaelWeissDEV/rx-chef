/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Remove Diacritics operation.
 * -----------------------------------------------------------------------------
 */

use unicode_normalization::UnicodeNormalization;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Remove Diacritics operation
pub struct RemoveDiacritics;

impl Operation for RemoveDiacritics {
    fn name(&self) -> &'static str {
        "Remove Diacritics"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Replaces accented characters with their latin character equivalent. Accented characters are made up of Unicode combining characters, so unicode text formatting such as strikethroughs and underlines will also be removed."
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        // Normalize to NFD (Normalization Form Decomposition)
        // and filter out characters in the combining diacritical marks range (U+0300 to U+036F)
        let output: String = input_str
            .nfd()
            .filter(|c| !('\u{0300}'..='\u{036f}').contains(c))
            .collect();

        Ok(output.into_bytes())
    }
}
