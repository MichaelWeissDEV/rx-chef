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
use libsm::sm2::{encrypt::EncryptCtx, signature::SigCtx};

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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Err(OperationError::InvalidInput(
                "SM2 cannot encrypt an empty message".into(),
            ));
        }
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

        let format = args.get(2).and_then(ArgValue::as_str).unwrap_or("C1C3C2");
        if format != "C1C3C2" && format != "C1C2C3" {
            return Err(OperationError::InvalidArgument {
                name: "Output Format".into(),
                reason: "Expected C1C3C2 or C1C2C3".into(),
            });
        }
        if args
            .get(3)
            .and_then(ArgValue::as_str)
            .unwrap_or("sm2p256v1")
            != "sm2p256v1"
        {
            return Err(OperationError::InvalidArgument {
                name: "Curve".into(),
                reason: "Only sm2p256v1 is supported".into(),
            });
        }
        let encoded = hex::decode(format!("04{public_key_x}{public_key_y}")).map_err(|error| {
            OperationError::InvalidArgument {
                name: "Public Key".into(),
                reason: error.to_string(),
            }
        })?;
        let public_key = SigCtx::new().load_pubkey(&encoded).map_err(|error| {
            OperationError::InvalidArgument {
                name: "Public Key".into(),
                reason: error.to_string(),
            }
        })?;
        let mut ciphertext = EncryptCtx::new(input.len(), public_key)
            .encrypt(&input)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
        if format == "C1C3C2" {
            let c3 = ciphertext.split_off(ciphertext.len() - 32);
            let c2 = ciphertext.split_off(65);
            ciphertext.extend_from_slice(&c3);
            ciphertext.extend_from_slice(&c2);
        }
        Ok(hex::encode(ciphertext).into_bytes())
    }
}
