/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate ECDSA Key Pair operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate ECDSA Key Pair operation
pub struct GenerateECDSAKeyPairOp;

impl Operation for GenerateECDSAKeyPairOp {
    fn name(&self) -> &'static str {
        "Generate ECDSA Key Pair"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Generate an ECDSA key pair with a given Curve."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Elliptic Curve",
                description: "Curve to use",
                default_value: "P-256",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output Format",
                description: "Format of the output keys",
                default_value: "PEM",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn input_requirement(&self) -> crate::operation::InputRequirement {
        crate::operation::InputRequirement::Ignored
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    /// Key generation draws from a random source.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Random]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
    }

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let curve_name = args.first().and_then(|a| a.as_str()).unwrap_or("P-256");
        let output_format = args.get(1).and_then(|a| a.as_str()).unwrap_or("PEM");

        if curve_name == "P-256" {
            use p256::{
                elliptic_curve::rand_core::OsRng,
                pkcs8::{EncodePrivateKey, EncodePublicKey},
                SecretKey,
            };

            let secret_key = SecretKey::random(&mut OsRng);
            let public_key = secret_key.public_key();

            match output_format {
                "PEM" => {
                    let priv_pem = secret_key.to_pkcs8_pem(Default::default()).map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode private key: {}",
                            e
                        ))
                    })?;
                    let pub_pem =
                        public_key
                            .to_public_key_pem(Default::default())
                            .map_err(|e| {
                                OperationError::ProcessingError(format!(
                                    "Failed to encode public key: {}",
                                    e
                                ))
                            })?;
                    return Ok(format!("{}\n{}", pub_pem, priv_pem.as_str()).into_bytes());
                }
                "DER" => {
                    // For DER, CyberChef returns privKeyHex for some reason in DER mode?
                    // "result = keyPair.prvKeyObj.prvKeyHex;"
                    // Let's return hex of private key
                    return Ok(hex::encode(secret_key.to_bytes()).into_bytes());
                }
                _ => {}
            }
        }

        Err(OperationError::InvalidArgument {
            name: if curve_name != "P-256" {
                "Elliptic Curve".to_string()
            } else {
                "Output Format".to_string()
            },
            reason: "the current implementation supports P-256 with PEM or DER output".to_string(),
        })
    }
}
