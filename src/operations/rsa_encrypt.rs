/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use md5::Md5;
use num_bigint::BigUint;
use rsa::{
    pkcs1::DecodeRsaPublicKey, pkcs8::DecodePublicKey, traits::PublicKeyParts, Oaep,
    Pkcs1v15Encrypt, RsaPublicKey,
};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RSA Encrypt operation
pub struct RSAEncrypt;

impl Operation for RSAEncrypt {
    fn name(&self) -> &'static str {
        "RSA Encrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Encrypt a message with a PEM encoded RSA public key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "RSA Public Key (PEM)",
                description: "RSA Public Key (PEM)",
                default_value: "-----BEGIN RSA PUBLIC KEY-----",
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
        let scheme = args.get(1).and_then(|a| a.as_str()).unwrap_or("RSA-OAEP");
        let md = args.get(2).and_then(|a| a.as_str()).unwrap_or("SHA-256");

        if pem_key.is_empty() || pem_key == "-----BEGIN RSA PUBLIC KEY-----" {
            return Err(OperationError::InvalidInput(
                "Please enter a public key.".to_string(),
            ));
        }

        let pub_key = RsaPublicKey::from_public_key_pem(pem_key)
            .or_else(|_| RsaPublicKey::from_pkcs1_pem(pem_key))
            .map_err(|e| {
                OperationError::InvalidInput(format!("Failed to load public key: {}", e))
            })?;

        let mut rng = rand::thread_rng();

        let encrypted = match scheme {
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
                pub_key.encrypt(&mut rng, padding, &input)
            }
            "RSAES-PKCS1-V1_5" => pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &input),
            "RAW" => {
                let n_bytes = pub_key.n().to_bytes_be();
                let e_bytes = pub_key.e().to_bytes_be();
                let n_big = BigUint::from_bytes_be(&n_bytes);
                let e_big = BigUint::from_bytes_be(&e_bytes);
                let m = BigUint::from_bytes_be(&input);
                if &m >= &n_big {
                    return Err(OperationError::InvalidInput(
                        "Message too large for key size".to_string(),
                    ));
                }
                let c = m.modpow(&e_big, &n_big);
                let mut c_bytes = c.to_bytes_be();
                let key_size = (pub_key.size() + 7) / 8;
                if c_bytes.len() < key_size {
                    let mut padded = vec![0u8; key_size - c_bytes.len()];
                    padded.extend(c_bytes);
                    c_bytes = padded;
                }
                Ok(c_bytes)
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported encryption scheme: {}",
                    scheme
                )))
            }
        }
        .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {}", e)))?;

        Ok(encrypted)
    }
}
