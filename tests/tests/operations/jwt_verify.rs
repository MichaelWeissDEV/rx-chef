// Tests for the jwt_verify operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jwt_verify::

use rxchef::operation::ArgValue;
use rxchef::operations::jwt_sign::JWTSign;
use rxchef::operations::jwt_verify::JWTVerify;
use rxchef::Operation;

#[test]
fn test_jwt_verify_hs256_valid() {
    let sign_op = JWTSign;
    let payload = br#"{"sub":"test","name":"Alice"}"#;
    let token = sign_op
        .run(
            payload.to_vec(),
            &[
                ArgValue::Str("mysecret".to_string()),
                ArgValue::Str("HS256".to_string()),
            ],
        )
        .expect("sign should succeed");
    let verify_op = JWTVerify;
    let result = verify_op
        .run(token, &[ArgValue::Str("mysecret".to_string())])
        .expect("verify should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert_eq!(v["sub"], "test");
}
#[test]
fn test_jwt_verify_hs256_wrong_key() {
    let sign_op = JWTSign;
    let payload = br#"{"sub":"test"}"#;
    let token = sign_op
        .run(
            payload.to_vec(),
            &[
                ArgValue::Str("secret1".to_string()),
                ArgValue::Str("HS256".to_string()),
            ],
        )
        .expect("sign");
    let verify_op = JWTVerify;
    let result = verify_op.run(token, &[ArgValue::Str("secret2".to_string())]);
    assert!(result.is_err());
}
#[test]
fn test_jwt_verify_invalid_format() {
    let op = JWTVerify;
    let result = op.run(b"bad.token".to_vec(), &[ArgValue::Str("key".to_string())]);
    assert!(result.is_err());
}
