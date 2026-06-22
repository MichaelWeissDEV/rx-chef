// Tests for the scrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations scrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::scrypt::ScryptOp;
use rxchef::Operation;

#[test]
fn test_scrypt_basic() {
    let op = ScryptOp;
    let input = b"password".to_vec();
    let args = [
        ArgValue::Str("salt".to_string()),
        ArgValue::Num(16.0), // Small N for fast test
        ArgValue::Num(8.0),
        ArgValue::Num(1.0),
        ArgValue::Num(32.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result.len(), 64); // 32 bytes in hex = 64 chars
}
#[test]
fn test_scrypt_invalid_n() {
    let op = ScryptOp;
    let input = b"password".to_vec();
    let args = [
        ArgValue::Str("salt".to_string()),
        ArgValue::Num(15.0), // Not a power of 2
        ArgValue::Num(8.0),
        ArgValue::Num(1.0),
        ArgValue::Num(32.0),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_scrypt_empty_salt() {
    let op = ScryptOp;
    let input = b"password".to_vec();
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Num(16.0),
        ArgValue::Num(8.0),
        ArgValue::Num(1.0),
        ArgValue::Num(32.0),
    ];
    let result = op.run(input, &args);
    assert!(result.is_ok());
}
