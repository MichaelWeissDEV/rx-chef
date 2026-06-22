/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Pseudo-Random Number Generator operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::BigUint;
use rand::RngCore;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Pseudo-Random Number Generator operation
pub struct PseudoRandomNumberGenerator;

impl Operation for PseudoRandomNumberGenerator {
    fn name(&self) -> &'static str {
        "Pseudo-Random Number Generator"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A cryptographically-secure pseudo-random number generator (PRNG). This operation uses a cryptographically secure RNG."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Number of bytes",
                description: "How many bytes to generate",
                default_value: "32",
            },
            ArgSchema {
                name: "Output as",
                description: "Output format (Hex, Integer, Byte array, Raw)",
                default_value: "Hex",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let num_bytes = args.first().and_then(|a| a.as_usize()).unwrap_or(32);
        let output_as = args.get(1).and_then(|a| a.as_str()).unwrap_or("Hex");

        let mut bytes = vec![0u8; num_bytes];
        rand::thread_rng().fill_bytes(&mut bytes);

        match output_as {
            "Hex" => Ok(hex::encode(bytes).into_bytes()),
            "Integer" => {
                let val = BigUint::from_bytes_le(&bytes);
                Ok(val.to_string().into_bytes())
            }
            "Byte array" => {
                let json = serde_json::to_string(&bytes)
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                Ok(json.into_bytes())
            }
            "Raw" => Ok(bytes),
            _ => Ok(hex::encode(bytes).into_bytes()),
        }
    }
}
