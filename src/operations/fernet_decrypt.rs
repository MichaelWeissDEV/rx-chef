/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Fernet Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use fernet::Fernet;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Fernet Decrypt operation
///
/// Decrypts a Fernet-encoded message using a base64url-encoded 32-byte key.
/// Fernet uses 128-bit AES-CBC with PKCS7 padding and HMAC-SHA256 for authentication.
pub struct FernetDecrypt;

impl Operation for FernetDecrypt {
    fn name(&self) -> &'static str {
        "Fernet Decrypt"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Fernet is a symmetric encryption method which makes sure that the message encrypted cannot be manipulated/read without the key. It uses URL safe encoding for the keys. Fernet uses 128-bit AES in CBC mode and PKCS7 padding, with HMAC using SHA256 for authentication. The IV is created from os.random(). Key: The key must be 32 bytes (256 bits) encoded with Base64."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Key",
            description: "Base64url-encoded 32-byte Fernet key",
            default_value: "",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_str = args.first().and_then(|a| a.as_str()).unwrap_or("");

        if key_str.is_empty() {
            return Err(OperationError::InvalidInput(
                "Secret must be 32 url-safe base64-encoded bytes.".to_string(),
            ));
        }

        let f = Fernet::new(key_str).ok_or_else(|| {
            OperationError::InvalidInput(
                "Secret must be 32 url-safe base64-encoded bytes.".to_string(),
            )
        })?;

        let token = String::from_utf8_lossy(&input).trim().to_string();

        if token.is_empty() {
            return Err(OperationError::InvalidInput("Invalid version".to_string()));
        }

        let plaintext = f.decrypt(&token).map_err(|e| {
            OperationError::ProcessingError(format!("Fernet decryption failed: {}", e))
        })?;

        Ok(plaintext)
    }
}
