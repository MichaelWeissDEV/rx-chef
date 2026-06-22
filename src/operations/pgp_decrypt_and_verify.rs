/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PGP Decrypt and Verify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PGP Decrypt and Verify operation
///
/// Decrypts and verifies a PGP signed+encrypted message.
/// This is a stub implementation - full PGP operation requires key material
/// at runtime and sequoia-openpgp integration.
pub struct PGPDecryptAndVerify;

impl Operation for PGPDecryptAndVerify {
    fn name(&self) -> &'static str {
        "PGP Decrypt and Verify"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Decrypts and verifies a PGP signed+encrypted message. \
         Input: ASCII-armoured encrypted PGP message. \
         Arguments: public key of signer, private key of recipient, optional passphrase."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Public key of signer",
                description: "ASCII-armoured PGP public key of the signer",
                default_value: "",
            },
            ArgSchema {
                name: "Private key of recipient",
                description: "ASCII-armoured PGP private key of the recipient",
                default_value: "",
            },
            ArgSchema {
                name: "Private key password",
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
        let public_key = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let private_key = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        if public_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Public key of signer".to_string(),
                reason: "Enter the public key of the signer.".to_string(),
            });
        }
        if private_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Private key of recipient".to_string(),
                reason: "Enter the private key of the recipient.".to_string(),
            });
        }
        let _ = input;
        Err(OperationError::ProcessingError(
            "PGP Decrypt and Verify requires PGP key material at runtime. \
             Full sequoia-openpgp integration not compiled in this build."
                .to_string(),
        ))
    }
}
