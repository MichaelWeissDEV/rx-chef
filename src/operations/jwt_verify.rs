/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JWT Verify operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;
type HmacSha512 = Hmac<Sha512>;

/// JWT Verify operation
///
/// Verifies a JSON Web Token and returns the decoded payload if valid.
/// Supports HS256, HS384, HS512, and None algorithms.
pub struct JWTVerify;

fn b64url(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

impl Operation for JWTVerify {
    fn name(&self) -> &'static str {
        "JWT Verify"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Verifies that a JSON Web Token is valid and has been signed with the provided secret key. Supports HS256, HS384, HS512, and None."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Public/Secret Key",
            description: "The secret key used to verify the HMAC signature",
            default_value: "secret",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key = args.first().and_then(|a| a.as_str()).unwrap_or("secret");
        let token = String::from_utf8_lossy(&input).trim().to_string();

        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(OperationError::InvalidInput(
                "JWT must have exactly 3 parts (header.payload.signature)".to_string(),
            ));
        }

        let header_bytes = URL_SAFE_NO_PAD.decode(parts[0]).map_err(|e| {
            OperationError::InvalidInput(format!("Invalid JWT header encoding: {}", e))
        })?;
        let header: serde_json::Value = serde_json::from_slice(&header_bytes).map_err(|e| {
            OperationError::InvalidInput(format!("JWT header is not valid JSON: {}", e))
        })?;

        let alg = header
            .get("alg")
            .and_then(|v| v.as_str())
            .unwrap_or("none")
            .to_uppercase();

        let signing_input = format!("{}.{}", parts[0], parts[1]);

        match alg.as_str() {
            "HS256" => {
                let mut mac = HmacSha256::new_from_slice(key.as_bytes())
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                mac.update(signing_input.as_bytes());
                let expected_sig = b64url(&mac.finalize().into_bytes());
                if expected_sig != parts[2] {
                    return Err(OperationError::ProcessingError(
                        "JWT signature verification failed".to_string(),
                    ));
                }
            }
            "HS384" => {
                let mut mac = HmacSha384::new_from_slice(key.as_bytes())
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                mac.update(signing_input.as_bytes());
                let expected_sig = b64url(&mac.finalize().into_bytes());
                if expected_sig != parts[2] {
                    return Err(OperationError::ProcessingError(
                        "JWT signature verification failed".to_string(),
                    ));
                }
            }
            "HS512" => {
                let mut mac = HmacSha512::new_from_slice(key.as_bytes())
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                mac.update(signing_input.as_bytes());
                let expected_sig = b64url(&mac.finalize().into_bytes());
                if expected_sig != parts[2] {
                    return Err(OperationError::ProcessingError(
                        "JWT signature verification failed".to_string(),
                    ));
                }
            }
            "NONE" => {
                // No signature to verify
            }
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "alg".to_string(),
                    reason: format!(
                        "Unsupported algorithm: {}. Supported: HS256, HS384, HS512, None",
                        other
                    ),
                })
            }
        }

        // Decode and return payload
        let payload_bytes = URL_SAFE_NO_PAD.decode(parts[1]).map_err(|e| {
            OperationError::InvalidInput(format!("Invalid JWT payload encoding: {}", e))
        })?;
        let payload: serde_json::Value = serde_json::from_slice(&payload_bytes).map_err(|e| {
            OperationError::InvalidInput(format!("JWT payload is not valid JSON: {}", e))
        })?;

        let pretty = serde_json::to_string_pretty(&payload)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        Ok(pretty.into_bytes())
    }
}
