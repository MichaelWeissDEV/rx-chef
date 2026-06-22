// Tests for the flask_session_sign operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations flask_session_sign::

use rxchef::operation::ArgValue;
use rxchef::operations::flask_session_sign::FlaskSessionSign;
use rxchef::Operation;

#[test]
fn test_flask_sign_produces_valid_token() {
    let op = FlaskSessionSign;
    let input = b"{\"user\":\"admin\",\"role\":\"superuser\"}".to_vec();
    let result = op
        .run(
            input,
            &[
                ArgValue::Str("mysecretkey".to_string()),
                ArgValue::Str("cookie-session".to_string()),
                ArgValue::Str("sha1".to_string()),
            ],
        )
        .expect("should sign");
    let token = String::from_utf8(result).expect("valid utf8");
    // Token should have 3 parts separated by dots
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
}
#[test]
fn test_flask_sign_no_key_error() {
    let op = FlaskSessionSign;
    let result = op.run(
        b"{\"user\":\"admin\"}".to_vec(),
        &[ArgValue::Str("".to_string())],
    );
    assert!(result.is_err());
}
#[test]
fn test_flask_sign_sha256_produces_valid_token() {
    let op = FlaskSessionSign;
    let input = b"{\"user\":\"admin\",\"role\":\"superuser\"}".to_vec();
    let result = op
        .run(
            input,
            &[
                ArgValue::Str("mysecretkey".to_string()),
                ArgValue::Str("cookie-session".to_string()),
                ArgValue::Str("sha256".to_string()),
            ],
        )
        .expect("should sign");
    let token = String::from_utf8(result).expect("valid utf8");
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
}
