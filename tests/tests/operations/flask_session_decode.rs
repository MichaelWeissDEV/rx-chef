// Tests for the flask_session_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations flask_session_decode::

use rxchef::operation::ArgValue;
use rxchef::operations::flask_session_decode::FlaskSessionDecode;
use rxchef::Operation;

// Token from CyberChef test suite:
// validTokenSha1 = "eyJyb2xlIjoic3VwZXJ1c2VyIiwidXNlciI6ImFkbWluIn0.aZ-KEw.E_x6bOhA4GU9t72pMinJUjN-O3I"
// payload decodes to: {"role":"superuser","user":"admin"}
const VALID_TOKEN_SHA1: &str =
    "eyJyb2xlIjoic3VwZXJ1c2VyIiwidXNlciI6ImFkbWluIn0.aZ-KEw.E_x6bOhA4GU9t72pMinJUjN-O3I";
#[test]
fn test_flask_decode_payload() {
    let op = FlaskSessionDecode;
    let result = op
        .run(
            VALID_TOKEN_SHA1.as_bytes().to_vec(),
            &[ArgValue::Bool(false)],
        )
        .expect("should decode");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert_eq!(v["user"], "admin");
    assert_eq!(v["role"], "superuser");
}
#[test]
fn test_flask_decode_with_timestamp() {
    let op = FlaskSessionDecode;
    let result = op
        .run(
            VALID_TOKEN_SHA1.as_bytes().to_vec(),
            &[ArgValue::Bool(true)],
        )
        .expect("should decode with timestamp");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert!(v["payload"]["user"] == "admin");
    assert!(v["timestamp"].is_number());
}
#[test]
fn test_flask_decode_invalid_format() {
    let op = FlaskSessionDecode;
    let result = op.run(b"not.valid".to_vec(), &[ArgValue::Bool(false)]);
    assert!(result.is_err());
}
