/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate RSA Key Pair operation.
 * -----------------------------------------------------------------------------
 */

use openssl::rsa::Rsa;
use serde_json::json;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate RSA Key Pair operation
pub struct GenerateRSAKeyPair;

impl Operation for GenerateRSAKeyPair {
    fn name(&self) -> &'static str {
        "Generate RSA Key Pair"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Generate an RSA key pair with a given number of bits."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "RSA Key Length",
                description: "RSA Key Length",
                default_value: "2048",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output Format",
                description: "Output Format",
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
        let key_length_str = args.first().and_then(|v| v.as_str()).unwrap_or("2048");
        let key_length =
            key_length_str
                .parse::<usize>()
                .map_err(|_| OperationError::InvalidArgument {
                    name: "RSA Key Length".to_string(),
                    reason: "Invalid number".to_string(),
                })?;

        let output_format = args.get(1).and_then(|v| v.as_str()).unwrap_or("PEM");

        let private_key = Rsa::generate(key_length as u32).map_err(|e| {
            OperationError::ProcessingError(format!("Key generation failed: {}", e))
        })?;
        match output_format {
            "PEM" => {
                let priv_pem = private_key.private_key_to_pem().map_err(|e| {
                    OperationError::ProcessingError(format!(
                        "Failed to encode private key to PEM: {}",
                        e
                    ))
                })?;
                let pub_pem = private_key.public_key_to_pem_pkcs1().map_err(|e| {
                    OperationError::ProcessingError(format!(
                        "Failed to encode public key to PEM: {}",
                        e
                    ))
                })?;

                let result = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&pub_pem),
                    String::from_utf8_lossy(&priv_pem)
                );
                Ok(result.into_bytes())
            }
            "JSON" => {
                // node-forge like JSON
                let n = private_key
                    .n()
                    .to_hex_str()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode RSA component: {e}"
                        ))
                    })?
                    .to_string();
                let e = private_key
                    .e()
                    .to_hex_str()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode RSA component: {e}"
                        ))
                    })?
                    .to_string();
                let d = private_key
                    .d()
                    .to_hex_str()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode RSA component: {e}"
                        ))
                    })?
                    .to_string();
                let p = private_key
                    .p()
                    .ok_or_else(|| {
                        OperationError::ProcessingError(
                            "Generated RSA key is missing p".to_string(),
                        )
                    })?
                    .to_hex_str()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode RSA component: {e}"
                        ))
                    })?
                    .to_string();
                let q = private_key
                    .q()
                    .ok_or_else(|| {
                        OperationError::ProcessingError(
                            "Generated RSA key is missing q".to_string(),
                        )
                    })?
                    .to_hex_str()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode RSA component: {e}"
                        ))
                    })?
                    .to_string();

                let json_res = json!({
                    "publicKey": {
                        "n": n,
                        "e": e
                    },
                    "privateKey": {
                        "n": n,
                        "e": e,
                        "d": d,
                        "p": p,
                        "q": q
                    }
                });
                Ok(serde_json::to_string_pretty(&json_res)
                    .unwrap()
                    .into_bytes())
            }
            "DER" => {
                let priv_der = private_key.private_key_to_der().map_err(|e| {
                    OperationError::ProcessingError(format!(
                        "Failed to encode private key to DER: {}",
                        e
                    ))
                })?;
                Ok(priv_der)
            }
            _ => Err(OperationError::InvalidArgument {
                name: "Output Format".to_string(),
                reason: "Invalid format".to_string(),
            }),
        }
    }
}
