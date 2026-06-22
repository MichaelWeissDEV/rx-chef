// Tests for the jwt_sign operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jwt_sign::

use rxchef::operation::ArgValue;
use rxchef::operations::jwt_sign::JWTSign;
use rxchef::Operation;

#[test]
fn test_jwt_sign_hs256() {
    let op = JWTSign;
    let payload = br#"{"sub":"1234567890","name":"John Doe"}"#;
    let result = op
        .run(
            payload.to_vec(),
            &[
                ArgValue::Str("secret".to_string()),
                ArgValue::Str("HS256".to_string()),
            ],
        )
        .expect("should succeed");
    let token = String::from_utf8(result).expect("valid utf8");
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
    assert!(!parts[2].is_empty());
}
#[test]
fn test_jwt_sign_none() {
    let op = JWTSign;
    let payload = br#"{"sub":"test"}"#;
    let result = op
        .run(
            payload.to_vec(),
            &[
                ArgValue::Str("".to_string()),
                ArgValue::Str("None".to_string()),
            ],
        )
        .expect("should succeed");
    let token = String::from_utf8(result).expect("valid utf8");
    // With None algorithm, signature part should be empty
    assert!(token.ends_with('.'));
}
#[test]
fn test_jwt_sign_invalid_json() {
    let op = JWTSign;
    let result = op.run(b"not json".to_vec(), &[]);
    assert!(result.is_err());
}
#[test]
fn test_jwt_sign_unsupported_algo() {
    let op = JWTSign;
    let result = op.run(
        br#"{"sub":"test"}"#.to_vec(),
        &[
            ArgValue::Str("secret".to_string()),
            ArgValue::Str("RS256".to_string()),
        ],
    );
    assert!(result.is_err());
}
