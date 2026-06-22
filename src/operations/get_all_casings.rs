/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Get All Casings operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Get All Casings operation
///
/// Outputs all possible casing variations of a string.
pub struct GetAllCasings;

impl Operation for GetAllCasings {
    fn name(&self) -> &'static str {
        "Get All Casings"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Outputs all possible casing variations of a string. Limited to inputs with at most 20 alphabetic characters to avoid excessive output."
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
        let s = String::from_utf8_lossy(&input);
        let s_lower = s.to_lowercase();
        let chars: Vec<char> = s_lower.chars().collect();
        let alpha_count = chars.iter().filter(|c| c.is_alphabetic()).count();

        if alpha_count > 20 {
            return Err(OperationError::InvalidInput(
                "Input has more than 20 alphabetic characters; result would be too large"
                    .to_string(),
            ));
        }

        // Build index list of alphabetic positions
        let alpha_indices: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
            .map(|(i, _)| i)
            .collect();

        let combinations = 1usize << alpha_count;
        let mut lines: Vec<String> = Vec::with_capacity(combinations);

        for mask in 0..combinations {
            let mut variant = chars.clone();
            for (bit, &idx) in alpha_indices.iter().enumerate() {
                if (mask >> bit) & 1 == 1 {
                    // uppercase this position
                    let upper: String = variant[idx].to_uppercase().collect();
                    variant[idx] = upper.chars().next().unwrap_or(variant[idx]);
                }
            }
            lines.push(variant.iter().collect());
        }

        Ok(lines.join("\n").into_bytes())
    }
}
