/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Scrypt operation.
 * -----------------------------------------------------------------------------
 */

use scrypt::{scrypt, Params};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// Scrypt operation
pub struct ScryptOp;

impl Operation for ScryptOp {
    fn name(&self) -> &'static str {
        "Scrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "scrypt is a password-based key derivation function (PBKDF) created by Colin Percival. The algorithm was specifically designed to make it costly to perform large-scale custom hardware attacks by requiring large amounts of memory. Enter the password in the input to generate its hash."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Salt",
                description: "Salt",
                default_value: "",
            },
            ArgSchema {
                name: "Iterations (N)",
                description: "Iterations (N). Must be a power of 2.",
                default_value: "16384",
            },
            ArgSchema {
                name: "Memory factor (r)",
                description: "Memory factor (r)",
                default_value: "8",
            },
            ArgSchema {
                name: "Parallelization factor (p)",
                description: "Parallelization factor (p)",
                default_value: "1",
            },
            ArgSchema {
                name: "Key length",
                description: "Key length",
                default_value: "64",
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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let salt_bytes = args
            .get(0)
            .map(Utils::convert_to_byte_array)
            .transpose()?
            .unwrap_or_default();
        let n = args.get(1).and_then(|v| v.as_f64()).unwrap_or(16384.0) as u32;
        let r = args.get(2).and_then(|v| v.as_f64()).unwrap_or(8.0) as u32;
        let p = args.get(3).and_then(|v| v.as_f64()).unwrap_or(1.0) as u32;
        let key_len = args.get(4).and_then(|v| v.as_f64()).unwrap_or(64.0) as usize;

        if n < 2 || (n & (n - 1)) != 0 {
            return Err(OperationError::InvalidArgument {
                name: "Iterations (N)".to_string(),
                reason: "N must be a power of 2 greater than 1".to_string(),
            });
        }
        let log_n = (n as f32).log2() as u8;

        let params =
            Params::new(log_n, r, p, key_len).map_err(|e| OperationError::InvalidArgument {
                name: "Parameters".to_string(),
                reason: format!("Invalid scrypt parameters: {}", e),
            })?;

        let mut output = vec![0u8; key_len];
        scrypt(&input, &salt_bytes, &params, &mut output)
            .map_err(|e| OperationError::ProcessingError(format!("Scrypt failed: {}", e)))?;

        Ok(hex::encode(output).into_bytes())
    }
}
