// Tests for the flask_session_verify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations flask_session_verify::

use rxchef::operation::ArgValue;
use rxchef::operations::flask_session_verify::FlaskSessionVerify;
use rxchef::Operation;

// From CyberChef test suite
const VALID_TOKEN_SHA1: &str =
    "eyJyb2xlIjoic3VwZXJ1c2VyIiwidXNlciI6ImFkbWluIn0.aZ-KEw.E_x6bOhA4GU9t72pMinJUjN-O3I";
const VALID_TOKEN_SHA256: &str =
    "eyJyb2xlIjoic3VwZXJ1c2VyIiwidXNlciI6ImFkbWluIn0.aab3Ew.Jsx2DOx_H9anZg0YcvhsASxQ11897EFHeQfS2oja4y8";
#[test]
fn test_verify_sha1_valid() {
    let op = FlaskSessionVerify;
    let result = op
        .run(
            VALID_TOKEN_SHA1.as_bytes().to_vec(),
            &[
                ArgValue::Str("mysecretkey".to_string()),
                ArgValue::Str("cookie-session".to_string()),
                ArgValue::Str("sha1".to_string()),
                ArgValue::Bool(false),
            ],
        )
        .expect("should verify");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert_eq!(v["valid"], true);
    assert_eq!(v["payload"]["user"], "admin");
    assert_eq!(v["payload"]["role"], "superuser");
}
#[test]
fn test_verify_sha256_valid() {
    let op = FlaskSessionVerify;
    let result = op
        .run(
            VALID_TOKEN_SHA256.as_bytes().to_vec(),
            &[
                ArgValue::Str("mysecretkey".to_string()),
                ArgValue::Str("cookie-session".to_string()),
                ArgValue::Str("sha256".to_string()),
                ArgValue::Bool(false),
            ],
        )
        .expect("should verify sha256");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert_eq!(v["valid"], true);
    assert_eq!(v["payload"]["user"], "admin");
}
#[test]
fn test_verify_wrong_key() {
    let op = FlaskSessionVerify;
    let result = op.run(
        VALID_TOKEN_SHA1.as_bytes().to_vec(),
        &[
            ArgValue::Str("notTheKey".to_string()),
            ArgValue::Str("cookie-session".to_string()),
            ArgValue::Str("sha1".to_string()),
            ArgValue::Bool(false),
        ],
    );
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("Invalid signature"));
}
#[test]
fn test_verify_wrong_salt() {
    let op = FlaskSessionVerify;
    let result = op.run(
        VALID_TOKEN_SHA1.as_bytes().to_vec(),
        &[
            ArgValue::Str("mysecretkey".to_string()),
            ArgValue::Str("notTheSalt".to_string()),
            ArgValue::Str("sha1".to_string()),
            ArgValue::Bool(false),
        ],
    );
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("Invalid signature"));
}
#[test]
fn test_verify_with_timestamp() {
    let op = FlaskSessionVerify;
    let result = op
        .run(
            VALID_TOKEN_SHA1.as_bytes().to_vec(),
            &[
                ArgValue::Str("mysecretkey".to_string()),
                ArgValue::Str("cookie-session".to_string()),
                ArgValue::Str("sha1".to_string()),
                ArgValue::Bool(true),
            ],
        )
        .expect("should verify with ts");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert_eq!(v["valid"], true);
    assert!(v["timestamp"].is_number());
}
