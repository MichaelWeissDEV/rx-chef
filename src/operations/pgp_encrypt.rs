/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PGP Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PGP Encrypt operation
///
/// Encrypts a message using the recipient's PGP public key.
/// This is a stub implementation - full PGP encryption requires key material
/// at runtime and sequoia-openpgp integration.
pub struct PGPEncrypt;

impl Operation for PGPEncrypt {
    fn name(&self) -> &'static str {
        "PGP Encrypt"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Encrypts a message using the recipient's ASCII-armoured PGP public key. \
         Input: plaintext message. Arguments: recipient's public key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Public key of recipient",
            description: "ASCII-armoured PGP public key of the recipient",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let public_key = args.first().and_then(|v| v.as_str()).unwrap_or("");
        if public_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Public key of recipient".to_string(),
                reason: "Enter the public key of the recipient.".to_string(),
            });
        }
        let _ = input;
        Err(OperationError::ProcessingError(
            "PGP Encrypt requires PGP key material at runtime. \
             Full sequoia-openpgp integration not compiled in this build."
                .to_string(),
        ))
    }
}
