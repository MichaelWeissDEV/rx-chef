/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate RSA Key Pair operation.
 * -----------------------------------------------------------------------------
 */

use rand::thread_rng;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    traits::{PrivateKeyParts, PublicKeyParts},
    RsaPrivateKey, RsaPublicKey,
};
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
            },
            ArgSchema {
                name: "Output Format",
                description: "Output Format",
                default_value: "PEM",
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

        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, key_length).map_err(|e| {
            OperationError::ProcessingError(format!("Key generation failed: {}", e))
        })?;
        let public_key = RsaPublicKey::from(&private_key);

        match output_format {
            "PEM" => {
                let priv_pem = private_key
                    .to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode private key to PEM: {}",
                            e
                        ))
                    })?;
                let pub_pem = public_key
                    .to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode public key to PEM: {}",
                            e
                        ))
                    })?;

                let result = format!("{}\n{}", pub_pem, &*priv_pem);
                Ok(result.into_bytes())
            }
            "JSON" => {
                // node-forge like JSON
                let n = private_key.n().to_str_radix(16);
                let e = private_key.e().to_str_radix(16);
                let d = private_key.d().to_str_radix(16);
                let p = private_key.primes()[0].to_str_radix(16);
                let q = private_key.primes()[1].to_str_radix(16);

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
                let priv_der = private_key.to_pkcs1_der().map_err(|e| {
                    OperationError::ProcessingError(format!(
                        "Failed to encode private key to DER: {}",
                        e
                    ))
                })?;
                Ok(priv_der.to_bytes().to_vec())
            }
            _ => Err(OperationError::InvalidArgument {
                name: "Output Format".to_string(),
                reason: "Invalid format".to_string(),
            }),
        }
    }
}
