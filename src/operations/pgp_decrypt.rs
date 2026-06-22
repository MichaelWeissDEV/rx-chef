/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PGP Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PGP Decrypt operation
///
/// Decrypts a PGP-encrypted message using the recipient's private key.
/// This is a stub implementation - full PGP decryption requires key material
/// at runtime and sequoia-openpgp integration.
pub struct PGPDecrypt;

impl Operation for PGPDecrypt {
    fn name(&self) -> &'static str {
        "PGP Decrypt"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Decrypts a PGP-encrypted message using the recipient's ASCII-armoured private key. \
         Input: ASCII-armoured PGP message. Arguments: private key and optional passphrase."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Private key of recipient",
                description: "ASCII-armoured PGP private key",
                default_value: "",
            },
            ArgSchema {
                name: "Private key passphrase",
                description: "Passphrase for the private key (leave blank if none)",
                default_value: "",
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
        let private_key = args.first().and_then(|v| v.as_str()).unwrap_or("");
        if private_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Private key of recipient".to_string(),
                reason: "Enter the private key of the recipient.".to_string(),
            });
        }
        let _ = input;
        Err(OperationError::ProcessingError(
            "PGP Decrypt requires PGP key material at runtime. \
             Full sequoia-openpgp integration not compiled in this build."
                .to_string(),
        ))
    }
}
