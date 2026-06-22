/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CTPH operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CTPH (Context Triggered Piecewise Hashing) operation
///
/// Context Triggered Piecewise Hashing, also called Fuzzy Hashing, can match
/// inputs that have homologies. Such inputs have sequences of identical bytes
/// in the same order, although bytes in between these sequences may be
/// different in both content and length.
pub struct CTPH;

impl Operation for CTPH {
    fn name(&self) -> &'static str {
        "CTPH"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Context Triggered Piecewise Hashing, also called Fuzzy Hashing, can match inputs that have homologies. Such inputs have sequences of identical bytes in the same order, although bytes in between these sequences may be different in both content and length."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let hash = ssdeep::hash(&input)
            .map_err(|e| OperationError::InvalidInput(format!("CTPH hash error: {}", e)))?;
        Ok(hash.into_bytes())
    }
}
