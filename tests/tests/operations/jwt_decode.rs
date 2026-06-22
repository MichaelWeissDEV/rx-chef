// Tests for the jwt_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations jwt_decode::

use rxchef::operations::jwt_decode::JWTDecode;
use rxchef::Operation;

#[test]
fn test_jwt_decode_basic() {
    let op = JWTDecode;
    // A simple JWT: header={"alg":"HS256","typ":"JWT"} payload={"sub":"1234567890"}
    let token = b"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U".to_vec();
    let result = op.run(token, &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let v: serde_json::Value = serde_json::from_str(&s).expect("valid json");
    assert_eq!(v["payload"]["sub"], "1234567890");
    assert_eq!(v["header"]["alg"], "HS256");
}
#[test]
fn test_jwt_decode_invalid_format() {
    let op = JWTDecode;
    let result = op.run(b"not.a.jwt.with.too.many.parts".to_vec(), &[]);
    assert!(result.is_err());
}
