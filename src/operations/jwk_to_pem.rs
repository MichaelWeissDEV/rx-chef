/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JWK to PEM operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use openssl::{
    bn::{BigNum, BigNumContext},
    pkey::PKey,
    rsa::Rsa,
};
use p256::pkcs8::{EncodePrivateKey, EncodePublicKey};
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

                    let n = BigNum::from_slice(&URL_SAFE_NO_PAD.decode(n_b64).map_err(|_| {
                        OperationError::ProcessingError("Invalid base64 in n".to_string())
                    })?)
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid RSA modulus: {e}"))
                    })?;
                    let e = BigNum::from_slice(&URL_SAFE_NO_PAD.decode(e_b64).map_err(|_| {
                        OperationError::ProcessingError("Invalid base64 in e".to_string())
                    })?)
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid RSA exponent: {e}"))
                    })?;

                    if let Some(d_b64) = jwk.get("d").and_then(|v| v.as_str()) {
                        // Private key
                        let d =
                            BigNum::from_slice(&URL_SAFE_NO_PAD.decode(d_b64).map_err(|_| {
                                OperationError::ProcessingError("Invalid base64 in d".to_string())
                            })?)
                            .map_err(|e| {
                                OperationError::ProcessingError(format!(
                                    "Invalid RSA private exponent: {e}"
                                ))
                            })?;
                        let p_b64 = jwk.get("p").and_then(|v| v.as_str()).ok_or_else(|| {
                            OperationError::ProcessingError("RSA private JWK missing p".to_string())
                        })?;
                        let q_b64 = jwk.get("q").and_then(|v| v.as_str()).ok_or_else(|| {
                            OperationError::ProcessingError("RSA private JWK missing q".to_string())
                        })?;
                        let p =
                            BigNum::from_slice(&URL_SAFE_NO_PAD.decode(p_b64).map_err(|_| {
                                OperationError::ProcessingError("Invalid base64 in p".to_string())
                            })?)
                            .map_err(|e| {
                                OperationError::ProcessingError(format!("Invalid RSA prime: {e}"))
                            })?;
                        let q =
                            BigNum::from_slice(&URL_SAFE_NO_PAD.decode(q_b64).map_err(|_| {
                                OperationError::ProcessingError("Invalid base64 in q".to_string())
                            })?)
                            .map_err(|e| {
                                OperationError::ProcessingError(format!("Invalid RSA prime: {e}"))
                            })?;

                        let mut context = BigNumContext::new().map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        let mut p_minus_one = p.to_owned().map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        p_minus_one.sub_word(1).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        let mut q_minus_one = q.to_owned().map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        q_minus_one.sub_word(1).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        let mut dmp1 = BigNum::new().map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        dmp1.nnmod(&d, &p_minus_one, &mut context).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        let mut dmq1 = BigNum::new().map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        dmq1.nnmod(&d, &q_minus_one, &mut context).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        let mut iqmp = BigNum::new().map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        iqmp.mod_inverse(&q, &p, &mut context).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        let priv_key = Rsa::from_private_components(
                            n, e, d, p, q, dmp1, dmq1, iqmp,
                        )
                        .map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {e}"
                            ))
                        })?;
                        output.push_str(&String::from_utf8_lossy(
                            &PKey::from_rsa(priv_key)
                                .map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Failed to encode PEM: {e}"
                                    ))
                                })?
                                .private_key_to_pem_pkcs8()
                                .map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Failed to encode PEM: {}",
                                        e
                                    ))
                                })?,
                        ));
                    } else {
                        // Public key
                        let pub_key = Rsa::from_public_components(n, e).map_err(|e| {
                            OperationError::ProcessingError(format!(
                                "Invalid RSA key components: {}",
                                e
                            ))
                        })?;
                        output.push_str(&String::from_utf8_lossy(
                            &PKey::from_rsa(pub_key)
                                .map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Failed to encode PEM: {e}"
                                    ))
                                })?
                                .public_key_to_pem()
                                .map_err(|e| {
                                    OperationError::ProcessingError(format!(
                                        "Failed to encode PEM: {}",
                                        e
                                    ))
                                })?,
                        ));
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
                                    priv_key
                                        .to_pkcs8_pem(p256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .as_ref(),
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
                                    pub_key
                                        .to_public_key_pem(p256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .as_ref(),
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
                                    priv_key
                                        .to_pkcs8_pem(k256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .as_ref(),
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
                                    pub_key
                                        .to_public_key_pem(k256::pkcs8::LineEnding::LF)
                                        .map_err(|e| {
                                            OperationError::ProcessingError(format!(
                                                "Failed to encode PEM: {}",
                                                e
                                            ))
                                        })?
                                        .as_ref(),
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
