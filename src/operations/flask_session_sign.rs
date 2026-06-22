/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Flask Session Sign operation.
 * -----------------------------------------------------------------------------
 */

use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::Sha256;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

type HmacSha1 = Hmac<Sha1>;
type HmacSha256 = Hmac<Sha256>;

/// Flask Session Sign operation
///
/// Signs a JSON payload to produce a Flask session cookie (itsdangerous HMAC).
/// itsdangerous signing algorithm:
///   derived_key = HMAC_algo(secret_key, salt)
///   signature   = HMAC_algo(derived_key, payload_b64url + "." + timestamp_b64url)
pub struct FlaskSessionSign;

impl Operation for FlaskSessionSign {
    fn name(&self) -> &'static str {
        "Flask Session Sign"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Signs a JSON payload to produce a Flask session cookie (itsdangerous HMAC)."
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

        // Parse JSON input
        let input_str = String::from_utf8_lossy(&input).trim().to_string();
        // Validate it is valid JSON
        let _: serde_json::Value = serde_json::from_str(&input_str)
            .map_err(|e| OperationError::InvalidInput(format!("Input is not valid JSON: {}", e)))?;

        // Encode payload as standard base64, then convert to url-safe without padding
        let payload_b64 = STANDARD.encode(input_str.as_bytes());
        let payload = b64_to_urlsafe_nopad(&payload_b64);

        // Build timestamp: current time as big-endian 32-bit int, base64url-encoded
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?
            .as_secs() as u32;
        let ts_bytes = now.to_be_bytes();
        let ts_b64 = STANDARD.encode(ts_bytes);
        let time = b64_to_urlsafe_nopad(&ts_b64);

        // Data to sign: "payload.timestamp"
        let data = format!("{}.{}", payload, time);

        let sig_bytes = match algorithm.as_str() {
            "sha256" => sign_itsdangerous_sha256(&key, salt.as_bytes(), data.as_bytes())?,
            _ => sign_itsdangerous_sha1(&key, salt.as_bytes(), data.as_bytes())?,
        };

        let sig_b64 = STANDARD.encode(&sig_bytes);
        let sig = b64_to_urlsafe_nopad(&sig_b64);

        let token = format!("{}.{}.{}", payload, time, sig);
        Ok(token.into_bytes())
    }
}

/// Convert standard base64 to url-safe base64 with no padding
pub(crate) fn b64_to_urlsafe_nopad(s: &str) -> String {
    s.replace('+', "-").replace('/', "_").replace('=', "")
}

/// itsdangerous SHA1 signing:
///   derived = HMAC-SHA1(key, salt)
///   sig     = HMAC-SHA1(derived, data)
pub(crate) fn sign_itsdangerous_sha1(
    key: &[u8],
    salt: &[u8],
    data: &[u8],
) -> Result<Vec<u8>, OperationError> {
    let mut mac = HmacSha1::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("HMAC init error: {}", e)))?;
    mac.update(salt);
    let derived = mac.finalize().into_bytes();

    let mut mac2 = HmacSha1::new_from_slice(&derived)
        .map_err(|e| OperationError::ProcessingError(format!("HMAC init error: {}", e)))?;
    mac2.update(data);
    Ok(mac2.finalize().into_bytes().to_vec())
}

/// itsdangerous SHA256 signing:
///   derived = HMAC-SHA256(key, salt)
///   sig     = HMAC-SHA256(derived, data)
pub(crate) fn sign_itsdangerous_sha256(
    key: &[u8],
    salt: &[u8],
    data: &[u8],
) -> Result<Vec<u8>, OperationError> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| OperationError::ProcessingError(format!("HMAC init error: {}", e)))?;
    mac.update(salt);
    let derived = mac.finalize().into_bytes();

    let mut mac2 = HmacSha256::new_from_slice(&derived)
        .map_err(|e| OperationError::ProcessingError(format!("HMAC init error: {}", e)))?;
    mac2.update(data);
    Ok(mac2.finalize().into_bytes().to_vec())
}
