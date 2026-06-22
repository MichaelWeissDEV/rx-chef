/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use md5::Md5;
use rsa::{
    pkcs1::DecodeRsaPrivateKey,
    pkcs8::DecodePrivateKey,
    traits::{PrivateKeyParts, PublicKeyParts},
    Oaep, Pkcs1v15Encrypt, RsaPrivateKey,
};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RSA Decrypt operation
pub struct RSADecrypt;

impl Operation for RSADecrypt {
    fn name(&self) -> &'static str {
        "RSA Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Decrypt an RSA encrypted message with a PEM encoded private key."
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
                description: "Key Password",
                default_value: "",
            },
            ArgSchema {
                name: "Encryption Scheme",
                description: "Encryption Scheme",
                default_value: "RSA-OAEP",
            },
            ArgSchema {
                name: "Message Digest Algorithm",
                description: "Message Digest Algorithm (for OAEP)",
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
        let scheme = args.get(2).and_then(|a| a.as_str()).unwrap_or("RSA-OAEP");
        let md = args.get(3).and_then(|a| a.as_str()).unwrap_or("SHA-256");

        if pem_key.is_empty() || pem_key == "-----BEGIN RSA PRIVATE KEY-----" {
            return Err(OperationError::InvalidInput(
                "Please enter a private key.".to_string(),
            ));
        }

        let priv_key = if password.is_empty() {
            RsaPrivateKey::from_pkcs8_pem(pem_key)
                .or_else(|_| RsaPrivateKey::from_pkcs1_pem(pem_key))
                .map_err(|e| {
                    OperationError::InvalidInput(format!("Failed to load private key: {}", e))
                })?
        } else {
            RsaPrivateKey::from_pkcs8_pem(pem_key)
                .or_else(|_| RsaPrivateKey::from_pkcs1_pem(pem_key))
                .map_err(|e| {
                    OperationError::InvalidInput(format!(
                        "Failed to load private key (password not supported): {}",
                        e
                    ))
                })?
        };

        let decrypted = match scheme {
            "RSA-OAEP" => {
                let padding = match md {
                    "SHA-1" => Oaep::new::<Sha1>(),
                    "SHA-224" => Oaep::new::<Sha224>(),
                    "SHA-256" => Oaep::new::<Sha256>(),
                    "SHA-384" => Oaep::new::<Sha384>(),
                    "SHA-512" => Oaep::new::<Sha512>(),
                    "MD5" => Oaep::new::<Md5>(),
                    _ => {
                        return Err(OperationError::InvalidInput(format!(
                            "Unsupported digest: {}",
                            md
                        )))
                    }
                };
                priv_key.decrypt(padding, &input)
            }
            "RSAES-PKCS1-V1_5" => priv_key.decrypt(Pkcs1v15Encrypt, &input),
            "RAW" => {
                let n = priv_key.n();
                let c = rsa::BigUint::from_bytes_be(&input);
                if &c >= n {
                    return Err(OperationError::InvalidInput(
                        "Ciphertext too large for key size".to_string(),
                    ));
                }
                let d = priv_key.d();
                let m = c.modpow(d, n);
                let m_bytes = m.to_bytes_be();
                Ok(m_bytes)
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported encryption scheme: {}",
                    scheme
                )))
            }
        }
        .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {}", e)))?;

        Ok(decrypted)
    }
}
