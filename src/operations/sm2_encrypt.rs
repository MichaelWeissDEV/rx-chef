/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the SM2 Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// SM2 Encrypt operation
///
/// Encrypts a message utilizing the SM2 standard.
pub struct Sm2Encrypt;

impl Operation for Sm2Encrypt {
    fn name(&self) -> &'static str {
        "SM2 Encrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Encrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Public Key X",
                description: "Public key component X in hex format (32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Public Key Y",
                description: "Public key component Y in hex format (32 bytes)",
                default_value: "",
            },
            ArgSchema {
                name: "Output Format",
                description: "The format of the output ciphertext (C1C3C2 or C1C2C3)",
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
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let public_key_x = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let public_key_y = args.get(1).and_then(|a| a.as_str()).unwrap_or("");

        if public_key_x.is_empty() || public_key_y.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Public Key".to_string(),
                reason: "Both Public Key X and Y are required.".to_string(),
            });
        }

        if public_key_x.len() != 64 || public_key_y.len() != 64 {
            return Err(OperationError::InvalidArgument {
                name: "Public Key".to_string(),
                reason: "Invalid Public Key - Ensure each component is 32 bytes in size (64 hex characters)".to_string(),
            });
        }

        // SM2 is currently not supported as it requires a specialized library (e.g. libsm2 or custom implementation of the SM2 curve).
        // The sm3 and sm4 crates are available in the project, but sm2 is not yet added to Cargo.toml.
        Err(OperationError::ProcessingError("SM2 Encrypt is currently a placeholder and requires a specialized SM2 library not yet present in the dependencies.".to_string()))
    }
}
