/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SM2 Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SM2 Decrypt operation
///
/// Decrypts a message utilizing the SM2 standard.
pub struct Sm2Decrypt;

impl Operation for Sm2Decrypt {
    fn name(&self) -> &'static str {
        "SM2 Decrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Decrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Private Key",
                description: "The private key in hex format (32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Input Format",
                description: "The format of the input ciphertext (C1C3C2 or C1C2C3)",
                default_value: "C1C3C2",
            },
            ArgSchema {
                name: "Curve",
                description: "The elliptic curve to use (sm2p256v1)",
                default_value: "sm2p256v1",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let private_key_hex = args.first().and_then(|a| a.as_str()).unwrap_or("");

        if private_key_hex.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Private Key".to_string(),
                reason: "Private key is required.".to_string(),
            });
        }

        if private_key_hex.len() != 64 {
            return Err(OperationError::InvalidArgument {
                name: "Private Key".to_string(),
                reason:
                    "Input private key must be in hex; and should be 32 bytes (64 hex characters)"
                        .to_string(),
            });
        }

        // SM2 is currently not supported as it requires a specialized library (e.g. libsm2 or custom implementation of the SM2 curve).
        // The sm3 and sm4 crates are available in the project, but sm2 is not yet added to Cargo.toml.
        Err(OperationError::ProcessingError("SM2 Decrypt is currently a placeholder and requires a specialized SM2 library not yet present in the dependencies.".to_string()))
    }
}
