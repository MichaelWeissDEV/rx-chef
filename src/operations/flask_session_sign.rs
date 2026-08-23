/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
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
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Salt",
                description: "Salt string (default: cookie-session)",
                default_value: "cookie-session",
                kind: crate::operation::ArgKind::Bytes,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Algorithm",
                description: "HMAC algorithm: sha1 or sha256",
                default_value: "sha1",
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
        DataType::Json
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    /// Embeds the current timestamp in the signed session.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Time]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
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
        let input_str = String::from_utf8(input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;
        // CyberChef receives JSON and serialises it compactly before signing.
        let value: serde_json::Value = serde_json::from_str(input_str.trim())
            .map_err(|e| OperationError::InvalidInput(format!("Input is not valid JSON: {}", e)))?;
        let input_str = serde_json::to_string(&value)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;

        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        // Upstream uses Math.ceil(Date.now()/1000), not a truncating floor.
        let now = elapsed.as_secs() + u64::from(elapsed.subsec_nanos() != 0);
        sign_session_at(&input_str, &key, salt, &algorithm, now as u32)
    }
}

fn sign_session_at(
    compact_json: &str,
    key: &[u8],
    salt: &str,
    algorithm: &str,
    timestamp: u32,
) -> Result<Vec<u8>, OperationError> {
        let payload_b64 = STANDARD.encode(compact_json.as_bytes());
        let payload = b64_to_urlsafe_nopad(&payload_b64);

        // Build timestamp as an unsigned big-endian 32-bit integer.
        let now = timestamp;
        let ts_bytes = now.to_be_bytes();
        let ts_b64 = STANDARD.encode(ts_bytes);
        let time = b64_to_urlsafe_nopad(&ts_b64);

        // Data to sign: "payload.timestamp"
        let data = format!("{}.{}", payload, time);

        let sig_bytes = match algorithm {
            "sha256" => sign_itsdangerous_sha256(&key, salt.as_bytes(), data.as_bytes())?,
            "sha1" => sign_itsdangerous_sha1(&key, salt.as_bytes(), data.as_bytes())?,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Algorithm".into(),
                    reason: format!("expected sha1 or sha256, got {algorithm:?}"),
                })
            }
        };

        let sig_b64 = STANDARD.encode(&sig_bytes);
        let sig = b64_to_urlsafe_nopad(&sig_b64);

        let token = format!("{}.{}.{}", payload, time, sig);
        Ok(token.into_bytes())
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

#[cfg(test)]
mod tests {
    use super::{sign_itsdangerous_sha1, sign_session_at};

    #[test]
    fn hmac_sha1_matches_openssl_known_answer() {
        // Independently evaluated with OpenSSL 3's HMAC implementation.
        let signature = sign_itsdangerous_sha1(
            b"secret",
            b"cookie-session",
            b"eyJ1c2VyIjoiYWRtaW4ifQ.AAAAAA",
        )
        .unwrap();
        assert_eq!(
            hex::encode(signature),
            "becbd06cd6068ec2e4db671a37175986847385ae"
        );
    }

    #[test]
    fn unix_epoch_and_empty_object_boundary() {
        let token = sign_session_at("{}", b"secret", "cookie-session", "sha1", 0).unwrap();
        let token = String::from_utf8(token).unwrap();
        assert!(token.starts_with("e30.AAAAAA."));
        assert_eq!(token.split('.').count(), 3);
    }
}
