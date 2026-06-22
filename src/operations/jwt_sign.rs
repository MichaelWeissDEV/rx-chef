/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JWT Sign operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

type HmacSha256 = Hmac<Sha256>;
type HmacSha384 = Hmac<Sha384>;
type HmacSha512 = Hmac<Sha512>;

/// JWT Sign operation
///
/// Signs a JSON object as a JSON Web Token using a provided secret key.
/// Supports HS256, HS384, HS512, and None algorithms.
pub struct JWTSign;

fn b64url(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

impl Operation for JWTSign {
    fn name(&self) -> &'static str {
        "JWT Sign"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Signs a JSON object as a JSON Web Token using a provided secret key. Supports HMAC algorithms (HS256, HS384, HS512) and None."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Private/Secret Key",
                description: "The secret key for HMAC signing",
                default_value: "secret",
            },
            ArgSchema {
                name: "Signing algorithm",
                description: "Algorithm: HS256, HS384, HS512, None",
                default_value: "HS256",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key = args.first().and_then(|a| a.as_str()).unwrap_or("secret");
        let algorithm = args.get(1).and_then(|a| a.as_str()).unwrap_or("HS256");

        // Validate input is JSON
        let _payload: serde_json::Value = serde_json::from_slice(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Input is not valid JSON: {}", e)))?;

        let (alg_name, alg_upper) = match algorithm.to_uppercase().as_str() {
            "HS256" => ("HS256", "HS256"),
            "HS384" => ("HS384", "HS384"),
            "HS512" => ("HS512", "HS512"),
            "NONE" | "None" => ("none", "None"),
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Signing algorithm".to_string(),
                    reason: format!(
                        "Unsupported algorithm: {}. Supported: HS256, HS384, HS512, None",
                        other
                    ),
                })
            }
        };

        let header = serde_json::json!({"alg": alg_upper, "typ": "JWT"});
        let header_b64 = b64url(
            serde_json::to_string(&header)
                .map_err(|e| OperationError::ProcessingError(e.to_string()))?
                .as_bytes(),
        );

        // Compact payload (strip whitespace)
        let payload_str = serde_json::to_string(&_payload)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        let payload_b64 = b64url(payload_str.as_bytes());

        let signing_input = format!("{}.{}", header_b64, payload_b64);

        let signature_b64 = match alg_name {
            "HS256" => {
                let mut mac = HmacSha256::new_from_slice(key.as_bytes())
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                mac.update(signing_input.as_bytes());
                b64url(&mac.finalize().into_bytes())
            }
            "HS384" => {
                let mut mac = HmacSha384::new_from_slice(key.as_bytes())
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                mac.update(signing_input.as_bytes());
                b64url(&mac.finalize().into_bytes())
            }
            "HS512" => {
                let mut mac = HmacSha512::new_from_slice(key.as_bytes())
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
                mac.update(signing_input.as_bytes());
                b64url(&mac.finalize().into_bytes())
            }
            "none" => String::new(),
            _ => unreachable!(),
        };

        let token = format!("{}.{}", signing_input, signature_b64);
        Ok(token.into_bytes())
    }
}
