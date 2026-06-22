/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Flask Session Verify operation.
 * -----------------------------------------------------------------------------
 */

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine,
};

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::flask_session_sign::{
        b64_to_urlsafe_nopad, sign_itsdangerous_sha1, sign_itsdangerous_sha256,
    },
};

/// Flask Session Verify operation
///
/// Verifies the HMAC signature of a Flask session cookie (itsdangerous).
pub struct FlaskSessionVerify;

/// Add padding to a base64url string that may be missing '=' characters.
fn pad_base64url(s: &str) -> String {
    let remainder = s.len() % 4;
    if remainder == 0 {
        s.to_string()
    } else {
        format!("{}{}", s, "=".repeat(4 - remainder))
    }
}

impl Operation for FlaskSessionVerify {
    fn name(&self) -> &'static str {
        "Flask Session Verify"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Verifies the HMAC signature of a Flask session cookie (itsdangerous) generated."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Secret key (UTF-8)",
                default_value: "",
            },
            ArgSchema {
                name: "Salt",
                description: "Salt string (default: cookie-session)",
                default_value: "cookie-session",
            },
            ArgSchema {
                name: "Algorithm",
                description: "HMAC algorithm: sha1 or sha256",
                default_value: "sha1",
            },
            ArgSchema {
                name: "View Timestamp",
                description: "Include timestamp in output",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .as_bytes()
            .to_vec();

        if key.is_empty() {
            return Err(OperationError::InvalidInput(
                "Secret key required".to_string(),
            ));
        }

        let salt = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("cookie-session");
        let salt = if salt.is_empty() {
            "cookie-session"
        } else {
            salt
        };

        let algorithm = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("sha1")
            .to_ascii_lowercase();

        let view_timestamp = args.get(3).and_then(|a| a.as_bool()).unwrap_or(true);

        let input_str = String::from_utf8_lossy(&input).trim().to_string();
        let parts: Vec<&str> = input_str.splitn(3, '.').collect();
        if parts.len() != 3 {
            return Err(OperationError::InvalidInput(
                "Invalid Flask token format. Expected payload.timestamp.signature".to_string(),
            ));
        }

        let payload_part = parts[0];
        let time_part = parts[1];
        let sig_part = parts[2];

        // Reconstruct the data that was signed
        let data = format!("{}.{}", payload_part, time_part);

        // Compute expected signature
        let expected_sig_bytes = match algorithm.as_str() {
            "sha256" => sign_itsdangerous_sha256(&key, salt.as_bytes(), data.as_bytes())?,
            _ => sign_itsdangerous_sha1(&key, salt.as_bytes(), data.as_bytes())?,
        };
        let expected_sig_b64 = STANDARD.encode(&expected_sig_bytes);
        let expected_sig = b64_to_urlsafe_nopad(&expected_sig_b64);

        if expected_sig != sig_part {
            return Err(OperationError::InvalidInput(
                "Invalid signature!".to_string(),
            ));
        }

        // Decode the timestamp
        let time_padded = pad_base64url(time_part);
        let time_bytes = URL_SAFE.decode(&time_padded).map_err(|e| {
            OperationError::InvalidInput(format!("Invalid timestamp base64: {}", e))
        })?;
        let timestamp = if time_bytes.len() >= 4 {
            i32::from_be_bytes([time_bytes[0], time_bytes[1], time_bytes[2], time_bytes[3]])
        } else {
            return Err(OperationError::InvalidInput(
                "Timestamp bytes too short".to_string(),
            ));
        };

        // Decode the payload
        let payload_padded = pad_base64url(payload_part);
        let payload_bytes = URL_SAFE
            .decode(&payload_padded)
            .map_err(|_| OperationError::InvalidInput("Invalid Base64 payload".to_string()))?;

        let payload_str = String::from_utf8(payload_bytes)
            .map_err(|e| OperationError::InvalidInput(format!("Payload is not UTF-8: {}", e)))?;

        let decoded: serde_json::Value = serde_json::from_str(&payload_str).map_err(|e| {
            OperationError::InvalidInput(format!("Unable to decode JSON payload: {}", e))
        })?;

        let output = if view_timestamp {
            serde_json::json!({
                "valid": true,
                "payload": decoded,
                "timestamp": timestamp
            })
        } else {
            serde_json::json!({
                "valid": true,
                "payload": decoded
            })
        };

        let pretty = serde_json::to_string_pretty(&output)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        Ok(pretty.into_bytes())
    }
}
