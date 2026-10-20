/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use openssl::{encrypt::Decrypter, rsa::Padding};

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    rsa_backend,
};

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
                kind: crate::operation::ArgKind::Bytes,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Key Password",
                description: "Key Password",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
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
            rsa_backend::private_key_from_pem(pem_key)
        } else {
            openssl::pkey::PKey::private_key_from_pem_passphrase(
                pem_key.as_bytes(),
                password.as_bytes(),
            )
        }
        .map_err(|e| OperationError::InvalidInput(format!("Failed to load private key: {e}")))?;

        let decrypted = match scheme {
            "RSA-OAEP" => {
                let digest = rsa_backend::digest(md).ok_or_else(|| {
                    OperationError::InvalidInput(format!("Unsupported digest: {md}"))
                })?;
                let mut decrypter = Decrypter::new(&priv_key)
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
                decrypter
                    .set_rsa_padding(Padding::PKCS1_OAEP)
                    .and_then(|_| decrypter.set_rsa_oaep_md(digest))
                    .and_then(|_| decrypter.set_rsa_mgf1_md(digest))
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
                let mut output = vec![
                    0;
                    decrypter.decrypt_len(&input).map_err(|e| {
                        OperationError::InvalidInput(format!("Decryption failed: {e}"))
                    })?
                ];
                let len = decrypter
                    .decrypt(&input, &mut output)
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
                output.truncate(len);
                Ok(output)
            }
            "RSAES-PKCS1-V1_5" => {
                let mut decrypter = Decrypter::new(&priv_key)
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
                decrypter
                    .set_rsa_padding(Padding::PKCS1)
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
                let mut output = vec![
                    0;
                    decrypter.decrypt_len(&input).map_err(|e| {
                        OperationError::InvalidInput(format!("Decryption failed: {e}"))
                    })?
                ];
                let len = decrypter
                    .decrypt(&input, &mut output)
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
                output.truncate(len);
                Ok(output)
            }
            "RAW" => {
                let rsa = priv_key.rsa().map_err(|e| {
                    OperationError::InvalidInput(format!("Failed to load private key: {e}"))
                })?;
                let mut output = vec![0; rsa.size() as usize];
                let len = rsa
                    .private_decrypt(&input, &mut output, Padding::NONE)
                    .map_err(|e| OperationError::InvalidInput(format!("Decryption failed: {e}")))?;
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

        Ok(decrypted)
    }
}
