/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Sign operation.
 * -----------------------------------------------------------------------------
 */

use md5::Md5;
use rsa::{pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePrivateKey, Pkcs1v15Sign, RsaPrivateKey};
use sha1::Sha1;
use sha2::{digest::Digest, Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RSA Sign operation
pub struct RSASign;

impl Operation for RSASign {
    fn name(&self) -> &'static str {
        "RSA Sign"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Sign a plaintext message with a PEM encoded RSA key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "RSA Private Key (PEM)",
                description: "RSA Private Key (PEM)",
                default_value: "-----BEGIN RSA PRIVATE KEY-----",
            },
            ArgSchema {
                name: "Key Password",
                description: "Password for the private key (if encrypted)",
                default_value: "",
            },
            ArgSchema {
                name: "Message Digest Algorithm",
                description: "Message Digest Algorithm",
                default_value: "SHA-256",
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
        let pem_key = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let password = args.get(1).and_then(|a| a.as_str()).unwrap_or("");
        let md_algo = args.get(2).and_then(|a| a.as_str()).unwrap_or("SHA-256");

        if pem_key.is_empty()
            || pem_key == "-----BEGIN RSA PRIVATE KEY-----"
            || pem_key == "-----BEGIN PRIVATE KEY-----"
        {
            return Err(OperationError::InvalidArgument {
                name: "RSA Private Key (PEM)".to_string(),
                reason: "Please enter a private key.".to_string(),
            });
        }

        let priv_key = if password.is_empty() {
            RsaPrivateKey::from_pkcs8_pem(pem_key)
                .or_else(|_| RsaPrivateKey::from_pkcs1_pem(pem_key))
                .map_err(|e| {
                    OperationError::InvalidInput(format!("Failed to load private key: {}", e))
                })?
        } else {
            // For encrypted keys, the 'rsa' crate doesn't directly support passwords in from_pem.
            // We would need to decrypt the PKCS#8 or PKCS#1 structure.
            // As a simplified port, we mention it's not fully supported if encrypted.
            return Err(OperationError::InvalidArgument {
                name: "Key Password".to_string(),
                reason: "Encrypted private keys are not yet supported in this port.".to_string(),
            });
        };

        let signature = match md_algo {
            "SHA-1" => {
                let hashed = Sha1::digest(&input);
                priv_key.sign(Pkcs1v15Sign::new::<Sha1>(), &hashed)
            }
            "MD5" => {
                let hashed = Md5::digest(&input);
                priv_key.sign(Pkcs1v15Sign::new::<Md5>(), &hashed)
            }
            "SHA-256" => {
                let hashed = Sha256::digest(&input);
                priv_key.sign(Pkcs1v15Sign::new::<Sha256>(), &hashed)
            }
            "SHA-384" => {
                let hashed = Sha384::digest(&input);
                priv_key.sign(Pkcs1v15Sign::new::<Sha384>(), &hashed)
            }
            "SHA-512" => {
                let hashed = Sha512::digest(&input);
                priv_key.sign(Pkcs1v15Sign::new::<Sha512>(), &hashed)
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Message Digest Algorithm".to_string(),
                    reason: format!("Unsupported algorithm: {}", md_algo),
                })
            }
        }
        .map_err(|e| OperationError::ProcessingError(format!("Signing failed: {}", e)))?;

        Ok(signature)
    }
}
