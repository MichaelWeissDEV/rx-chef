/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use openssl::{encrypt::Encrypter, rsa::Padding};

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    rsa_backend,
};

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
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Encryption Scheme",
                description: "Encryption Scheme",
                default_value: "RSA-OAEP",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Message Digest Algorithm",
                description: "Message Digest Algorithm (for OAEP)",
                default_value: "SHA-256",
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

    fn output_type(&self) -> DataType {
        DataType::String
    }

    /// PKCS#1 and OAEP padding are randomised.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Random]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
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

        let pub_key = rsa_backend::public_key_from_pem(pem_key).map_err(|e| {
            OperationError::InvalidInput(format!("Failed to load public key: {}", e))
        })?;

        let encrypted = match scheme {
            "RSA-OAEP" => {
                let digest = rsa_backend::digest(md).ok_or_else(|| {
                    OperationError::InvalidInput(format!("Unsupported digest: {}", md))
                })?;
                let mut encrypter = Encrypter::new(&pub_key)
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                encrypter
                    .set_rsa_padding(Padding::PKCS1_OAEP)
                    .and_then(|_| encrypter.set_rsa_oaep_md(digest))
                    .and_then(|_| encrypter.set_rsa_mgf1_md(digest))
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                let mut output = vec![
                    0;
                    encrypter.encrypt_len(&input).map_err(|e| {
                        OperationError::InvalidInput(format!("Encryption failed: {e}"))
                    })?
                ];
                let len = encrypter
                    .encrypt(&input, &mut output)
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                output.truncate(len);
                Ok(output)
            }
            "RSAES-PKCS1-V1_5" => {
                let mut encrypter = Encrypter::new(&pub_key)
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                encrypter
                    .set_rsa_padding(Padding::PKCS1)
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                let mut output = vec![
                    0;
                    encrypter.encrypt_len(&input).map_err(|e| {
                        OperationError::InvalidInput(format!("Encryption failed: {e}"))
                    })?
                ];
                let len = encrypter
                    .encrypt(&input, &mut output)
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                output.truncate(len);
                Ok(output)
            }
            "RAW" => {
                let rsa = pub_key.rsa().map_err(|e| {
                    OperationError::InvalidInput(format!("Failed to load public key: {e}"))
                })?;
                if input.len() > rsa.size() as usize {
                    return Err(OperationError::InvalidInput(
                        "Message too large for key size".to_string(),
                    ));
                }
                let mut output = vec![0; rsa.size() as usize];
                let mut raw_input = vec![0; rsa.size() as usize - input.len()];
                raw_input.extend_from_slice(&input);
                let len = rsa
                    .public_encrypt(&raw_input, &mut output, Padding::NONE)
                    .map_err(|e| OperationError::InvalidInput(format!("Encryption failed: {e}")))?;
                output.truncate(len);
                Ok(output)
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported encryption scheme: {}",
                    scheme
                )))
            }
        }?;

        Ok(encrypted)
    }
}
