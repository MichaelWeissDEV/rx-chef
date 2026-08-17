/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Base32 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ToBase32;

impl Operation for ToBase32 {
    fn name(&self) -> &'static str {
        "To Base32"
    }
    fn module(&self) -> &'static str {
        "Default"
    }
    fn description(&self) -> &'static str {
        "Base32 is a notation for encoding arbitrary byte data using a restricted set of symbols."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Alphabet",
            description: "The Base32 alphabet",
            default_value: "A-Z2-7=",
            kind: crate::operation::ArgKind::String,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }
    fn input_type(&self) -> DataType {
        DataType::Bytes
    }
    fn output_type(&self) -> DataType {
        DataType::String
    }
    /// Verified against upstream CyberChef by the differential harness.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let alphabet = args.first().and_then(|v| v.as_str()).unwrap_or("A-Z2-7=");
        // The argument is written in range notation ("A-Z2-7="), which
        // `data_encoding` does not understand. Without expanding it first the
        // operation rejected its own declared default and could not run at all.
        let symbols = crate::operations::from_base32::expand_base32_alphabet(alphabet);
        let mut spec = data_encoding::Specification::new();
        spec.symbols = symbols;
        spec.padding = Some('=');
        let encoding = spec
            .encoding()
            .map_err(|e| OperationError::InvalidArgument {
                name: "Alphabet".to_string(),
                reason: e.to_string(),
            })?;
        Ok(encoding.encode(&input).into_bytes())
    }
}
