/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JWK to PEM operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use serde_json::Value;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JWK to PEM operation
pub struct JWKToPem;

impl Operation for JWKToPem {
    fn name(&self) -> &'static str {
        "JWK to PEM"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Converts Keys in JSON Web Key format to PEM format (PKCS#8)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let input_json: Value = serde_json::from_str(&input_str)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON: {}", e)))?;

        let keys = if let Some(keys_array) = input_json.as_array() {
            keys_array.clone()
        } else if let Some(keys_field) = input_json.get("keys").and_then(|v| v.as_array()) {
            keys_field.clone()
        } else if input_json.is_object() {
            vec![input_json]
        } else {
            return Err(OperationError::InvalidInput(
                "Input is not a JSON Web Key".to_string(),
            ));
        };

        let mut output = String::new();
        for jwk in keys {
            let kty = jwk.get("kty").and_then(|v| v.as_str()).ok_or_else(|| {
                OperationError::ProcessingError("Invalid JWK: missing kty".to_string())
            })?;

            match kty {
                "RSA" => {
                    let n_b64 = jwk.get("n").and_then(|v| v.as_str()).ok_or_else(|| {
                        OperationError::ProcessingError("RSA JWK missing n".to_string())
                    })?;
                    let e_b64 = jwk.get("e").and_then(|v| v.as_str()).ok_or_else(|| {
                        OperationError::ProcessingError("RSA JWK missing e".to_string())
                    })?;

                    let n = rsa::BigUint::from_bytes_be(&URL_SAFE_NO_PAD.decode(n_b64).map_err(
                        |_| OperationError::ProcessingError("Invalid base64 in n".to_string()),
                    )?);
                    let e = rsa::BigUint::from_bytes_be(&URL_SAFE_NO_PAD.decode(e_b64).map_err(
                        |_| OperationError::ProcessingError("Invalid base64 in e".to_string()),
                    )?);

                    if let Some(d_b64) = jwk.get("d").and_then(|v| v.as_str()) {
                        // Private key
                        let d = rsa::BigUint::from_bytes_be(
                            &URL_SAFE_NO_PAD.decode(d_b64).map_err(|_| {
                                OperationError::ProcessingError("Invalid base64 in d".to_string())
                            })?,
                        );
                        let p_b64 = jwk.get("p").and_then(|v| v.as_str()).ok_or_else(|| {
                            OperationError::ProcessingError("RSA private JWK missing p".to_string())
                        })?;
                        let q_b64 = jwk.get("q").and_then(|v| v.as_str()).ok_or_else(|| {
                            OperationError::ProcessingError("RSA private JWK missing q".to_string())
                        })?;
                        let p = rsa::BigUint::from_bytes_be(
                            &URL_SAFE_NO_PAD.decode(p_b64).map_err(|_| {
                                OperationError::ProcessingError("Invalid base64 in p".to_string())
                            })?,
                        );
                        let q = rsa::BigUint::from_bytes_be(
                            &URL_SAFE_NO_PAD.decode(q_b64).map_err(|_| {
                                OperationError::ProcessingError("Invalid base64 in q".to_string())
                            })?,
                        );

                        let primes = vec![p, q];
                        let priv_key = rsa::RsaPrivateKey::from_components(n, e, d, primes)
                            .map_err(|e| {
                                OperationError::ProcessingError(format!(
                                    "Invalid RSA key components: {}",
                                    e
                                ))
                            })?;
                        output.push_str(
                            &priv_key
                                .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                                .map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Failed to encode PEM: {}",
                                        e
                                    ))
                                })?
                                .to_string(),
                        );
                    } else {
                        // Public key
                        let pub_key = rsa::RsaPublicKey::new(n, e).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {}",
                                e
                            ))
                        })?;
                        output.push_str(
                            &pub_key
                                .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                                .map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Failed to encode PEM: {}",
                                        e
                                    ))
                                })?
                                .to_string(),
                        );
                    }
                }
                "EC" => {
                    let crv = jwk.get("crv").and_then(|v| v.as_str()).ok_or_else(|| {
                        OperationError::ProcessingError("EC JWK missing crv".to_string())
                    })?;
                    let x_b64 = jwk.get("x").and_then(|v| v.as_str()).ok_or_else(|| {
                        OperationError::ProcessingError("EC JWK missing x".to_string())
                    })?;
                    let y_b64 = jwk.get("y").and_then(|v| v.as_str()).ok_or_else(|| {
                        OperationError::ProcessingError("EC JWK missing y".to_string())
                    })?;

                    let x = URL_SAFE_NO_PAD.decode(x_b64).map_err(|_| {
                        OperationError::ProcessingError("Invalid base64 in x".to_string())
                    })?;
                    let y = URL_SAFE_NO_PAD.decode(y_b64).map_err(|_| {
                        OperationError::ProcessingError("Invalid base64 in y".to_string())
                    })?;

                    match crv {
                        "P-256" => {
                            if let Some(d_b64) = jwk.get("d").and_then(|v| v.as_str()) {
                                let d = URL_SAFE_NO_PAD.decode(d_b64).map_err(|_| {
                                    OperationError::ProcessingError(
                                        "Invalid base64 in d".to_string(),
                                    )
                                })?;
                                let priv_key = p256::SecretKey::from_slice(&d).map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Invalid P-256 secret key: {}",
                                        e
                                    ))
                                })?;
                                output.push_str(
                                    &priv_key
                                        .to_pkcs8_pem(p256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .to_string(),
                                );
                            } else {
                                let mut encoded = vec![0x04];
                                encoded.extend_from_slice(&x);
                                encoded.extend_from_slice(&y);
                                let pub_key =
                                    p256::PublicKey::from_sec1_bytes(&encoded).map_err(|e| {
                                        OperationError::ProcessingError(format!(
                                            "Invalid P-256 public key: {}",
                                            e
                                        ))
                                    })?;
                                output.push_str(
                                    &pub_key
                                        .to_public_key_pem(p256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .to_string(),
                                );
                            }
                        }
                        "secp256k1" => {
                            if let Some(d_b64) = jwk.get("d").and_then(|v| v.as_str()) {
                                let d = URL_SAFE_NO_PAD.decode(d_b64).map_err(|_| {
                                    OperationError::ProcessingError(
                                        "Invalid base64 in d".to_string(),
                                    )
                                })?;
                                let priv_key = k256::SecretKey::from_slice(&d).map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Invalid secp256k1 secret key: {}",
                                        e
                                    ))
                                })?;
                                output.push_str(
                                    &priv_key
                                        .to_pkcs8_pem(k256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .to_string(),
                                );
                            } else {
                                let mut encoded = vec![0x04];
                                encoded.extend_from_slice(&x);
                                encoded.extend_from_slice(&y);
                                let pub_key =
                                    k256::PublicKey::from_sec1_bytes(&encoded).map_err(|e| {
                                        OperationError::ProcessingError(format!(
                                            "Invalid secp256k1 public key: {}",
                                            e
                                        ))
                                    })?;
                                output.push_str(
                                    &pub_key
                                        .to_public_key_pem(k256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .to_string(),
                                );
                            }
                        }
                        _ => {
                            return Err(OperationError::ProcessingError(format!(
                                "Unsupported EC curve: {}",
                                crv
                            )))
                        }
                    }
                }
                _ => {
                    return Err(OperationError::ProcessingError(format!(
                        "Unsupported JWK key type '{}'",
                        kty
                    )))
                }
            }
        }

        Ok(output.into_bytes())
    }
}
