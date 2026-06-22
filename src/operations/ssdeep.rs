/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SSDEEP operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SSDEEP fuzzy hash operation
///
/// SSDEEP is a program for computing context triggered piecewise hashes (CTPH).
/// Also called fuzzy hashes, CTPH can match inputs that have homologies.
/// SSDEEP hashes are widely used for simple identification purposes (e.g. in VirusTotal).
pub struct SSDEEP;

impl Operation for SSDEEP {
    fn name(&self) -> &'static str {
        "SSDEEP"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "SSDEEP is a program for computing context triggered piecewise hashes (CTPH). Also called fuzzy hashes, CTPH can match inputs that have homologies. SSDEEP hashes are now widely used for simple identification purposes (e.g. the 'Basic Properties' section in VirusTotal)."
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
            .map_err(|e| OperationError::InvalidInput(format!("SSDEEP hash error: {}", e)))?;
        Ok(hash.into_bytes())
    }
}
