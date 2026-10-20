/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RSA Sign operation.
 * -----------------------------------------------------------------------------
 */

use openssl::{rsa::Padding, sign::Signer};

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    rsa_backend,
};

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
                kind: crate::operation::ArgKind::Bytes,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Key Password",
                description: "Password for the private key (if encrypted)",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Message Digest Algorithm",
                description: "Message Digest Algorithm",
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
            rsa_backend::private_key_from_pem(pem_key)
        } else {
            openssl::pkey::PKey::private_key_from_pem_passphrase(
                pem_key.as_bytes(),
                password.as_bytes(),
            )
        }
        .map_err(|e| OperationError::InvalidInput(format!("Failed to load private key: {e}")))?;
        let digest =
            rsa_backend::digest(md_algo).ok_or_else(|| OperationError::InvalidArgument {
                name: "Message Digest Algorithm".to_string(),
                reason: format!("Unsupported algorithm: {md_algo}"),
            })?;
        let mut signer = Signer::new(digest, &priv_key)
            .map_err(|e| OperationError::ProcessingError(format!("Signing failed: {e}")))?;
        signer
            .set_rsa_padding(Padding::PKCS1)
            .map_err(|e| OperationError::ProcessingError(format!("Signing failed: {e}")))?;
        signer
            .update(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Signing failed: {e}")))?;
        let signature = signer
            .sign_to_vec()
            .map_err(|e| OperationError::ProcessingError(format!("Signing failed: {e}")))?;

        Ok(signature)
    }
}
