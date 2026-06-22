/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PGP Encrypt and Sign operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PGP Encrypt and Sign operation
///
/// Encrypts a message to a recipient and signs it with the sender's private key.
/// This is a stub implementation - full PGP operation requires key material
/// at runtime and sequoia-openpgp integration.
pub struct PGPEncryptAndSign;

impl Operation for PGPEncryptAndSign {
    fn name(&self) -> &'static str {
        "PGP Encrypt and Sign"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Encrypts a message to the recipient and signs it with the signer's private key. \
         Input: cleartext to sign. \
         Arguments: private key of signer, optional passphrase, public key of recipient."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Private key of signer",
                description: "ASCII-armoured PGP private key of the signer",
                default_value: "",
            },
            ArgSchema {
                name: "Private key passphrase",
                description: "Passphrase for the private key (leave blank if none)",
                default_value: "",
            },
            ArgSchema {
                name: "Public key of recipient",
                description: "ASCII-armoured PGP public key of the recipient",
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
        let public_key = args.get(2).and_then(|v| v.as_str()).unwrap_or("");
        if private_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Private key of signer".to_string(),
                reason: "Enter the private key of the signer.".to_string(),
            });
        }
        if public_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Public key of recipient".to_string(),
                reason: "Enter the public key of the recipient.".to_string(),
            });
        }
        let _ = input;
        Err(OperationError::ProcessingError(
            "PGP Encrypt and Sign requires PGP key material at runtime. \
             Full sequoia-openpgp integration not compiled in this build."
                .to_string(),
        ))
    }
}
