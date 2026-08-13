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
use libsm::sm2::{encrypt::DecryptCtx, signature::Seckey};

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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
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

        let format = args.get(1).and_then(ArgValue::as_str).unwrap_or("C1C3C2");
        if format != "C1C3C2" && format != "C1C2C3" {
            return Err(OperationError::InvalidArgument {
                name: "Input Format".into(),
                reason: "Expected C1C3C2 or C1C2C3".into(),
            });
        }
        if args
            .get(2)
            .and_then(ArgValue::as_str)
            .unwrap_or("sm2p256v1")
            != "sm2p256v1"
        {
            return Err(OperationError::InvalidArgument {
                name: "Curve".into(),
                reason: "Only sm2p256v1 is supported".into(),
            });
        }
        let encoded = std::str::from_utf8(&input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        let mut ciphertext = hex::decode(encoded.trim())
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        if ciphertext.len() < 97 {
            return Err(OperationError::InvalidInput(
                "SM2 ciphertext must contain C1, C2, and C3".into(),
            ));
        }
        if format == "C1C3C2" {
            let c2 = ciphertext.split_off(97);
            let c3 = ciphertext.split_off(65);
            ciphertext.extend_from_slice(&c2);
            ciphertext.extend_from_slice(&c3);
        }
        let message_length = ciphertext.len() - 97;
        let private_key =
            Seckey::from_bytes_be(&hex::decode(private_key_hex).map_err(|error| {
                OperationError::InvalidArgument {
                    name: "Private Key".into(),
                    reason: error.to_string(),
                }
            })?);
        DecryptCtx::new(message_length, private_key)
            .decrypt(&ciphertext)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))
    }
}
