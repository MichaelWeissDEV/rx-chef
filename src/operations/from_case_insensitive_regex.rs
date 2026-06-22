/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Case Insensitive Regex operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Case Insensitive Regex operation.
///
/// Converts a case-insensitive regex (with [xX] style character classes) back
/// to a simpler case-sensitive form by collapsing [xX] pairs into a single
/// character.
///
/// e.g. `[mM][oO][zZ][iI][lL][lL][aA]/[0-9].[0-9] .*`
///   -> `Mozilla/[0-9].[0-9] .*`
pub struct FromCaseInsensitiveRegex;

impl Operation for FromCaseInsensitiveRegex {
    fn name(&self) -> &'static str {
        "From Case Insensitive Regex"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a case-insensitive regex string to a case sensitive regex string by collapsing [xX] character class pairs into single characters."
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        // Match [xX] patterns where both chars are the same letter (different case)
        let re = Regex::new(r"\[([a-zA-Z])([a-zA-Z])\]")
            .map_err(|e| OperationError::ProcessingError(format!("Regex error: {}", e)))?;

        let result = re.replace_all(&input_str, |caps: &regex::Captures| {
            let c1 = caps[1].chars().next().unwrap_or(' ');
            let c2 = caps[2].chars().next().unwrap_or(' ');
            if c1.to_uppercase().to_string() == c2.to_uppercase().to_string() {
                // Same letter different case - collapse to upper case version
                c1.to_uppercase().to_string()
            } else {
                // Different letters - leave as-is
                caps[0].to_string()
            }
        });

        Ok(result.into_owned().into_bytes())
    }
}
