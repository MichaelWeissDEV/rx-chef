/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Verify operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose, Engine as _};
use md5::Md5;
use rsa::{pkcs1::DecodeRsaPublicKey, pkcs8::DecodePublicKey, Pkcs1v15Sign, RsaPublicKey};
use sha1::Sha1;
use sha2::{digest::Digest, Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RSA Verify operation
pub struct RSAVerify;

impl Operation for RSAVerify {
    fn name(&self) -> &'static str {
        "RSA Verify"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Verify a message against a signature and a public PEM encoded RSA key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "RSA Public Key (PEM)",
                description: "RSA Public Key (PEM)",
                default_value: "-----BEGIN RSA PUBLIC KEY-----",
            },
            ArgSchema {
                name: "Message",
                description: "Message to verify",
                default_value: "",
            },
            ArgSchema {
                name: "Message format",
                description: "Format of the message",
                default_value: "Raw",
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
        let message = args.get(1).and_then(|a| a.as_str()).unwrap_or("");
        let format = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw");
        let md_algo = args.get(3).and_then(|a| a.as_str()).unwrap_or("SHA-256");

        if pem_key.is_empty()
            || pem_key == "-----BEGIN RSA PUBLIC KEY-----"
            || pem_key == "-----BEGIN PUBLIC KEY-----"
        {
            return Err(OperationError::InvalidArgument {
                name: "RSA Public Key (PEM)".to_string(),
                reason: "Please enter a public key.".to_string(),
            });
        }

        let pub_key = RsaPublicKey::from_public_key_pem(pem_key)
            .or_else(|_| RsaPublicKey::from_pkcs1_pem(pem_key))
            .map_err(|e| {
                OperationError::InvalidInput(format!("Failed to load public key: {}", e))
            })?;

        let message_bytes = match format {
            "Raw" => message.as_bytes().to_vec(),
            "Hex" => hex::decode(
                message
                    .trim()
                    .replace(' ', "")
                    .replace('\n', "")
                    .replace('\r', ""),
            )
            .map_err(|e| OperationError::InvalidArgument {
                name: "Message".to_string(),
                reason: format!("Invalid hex: {}", e),
            })?,
            "Base64" => general_purpose::STANDARD
                .decode(message.trim())
                .map_err(|e| OperationError::InvalidArgument {
                    name: "Message".to_string(),
                    reason: format!("Invalid base64: {}", e),
                })?,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Message format".to_string(),
                    reason: format!("Unsupported format: {}", format),
                })
            }
        };

        let result = match md_algo {
            "SHA-1" => {
                let hashed = Sha1::digest(&message_bytes);
                pub_key.verify(Pkcs1v15Sign::new::<Sha1>(), &hashed, &input)
            }
            "MD5" => {
                let hashed = Md5::digest(&message_bytes);
                pub_key.verify(Pkcs1v15Sign::new::<Md5>(), &hashed, &input)
            }
            "SHA-256" => {
                let hashed = Sha256::digest(&message_bytes);
                pub_key.verify(Pkcs1v15Sign::new::<Sha256>(), &hashed, &input)
            }
            "SHA-384" => {
                let hashed = Sha384::digest(&message_bytes);
                pub_key.verify(Pkcs1v15Sign::new::<Sha384>(), &hashed, &input)
            }
            "SHA-512" => {
                let hashed = Sha512::digest(&message_bytes);
                pub_key.verify(Pkcs1v15Sign::new::<Sha512>(), &hashed, &input)
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Message Digest Algorithm".to_string(),
                    reason: format!("Unsupported algorithm: {}", md_algo),
                })
            }
        };

        match result {
            Ok(_) => Ok("Verified OK".as_bytes().to_vec()),
            Err(_) => Ok("Verification Failure".as_bytes().to_vec()),
        }
    }
}
