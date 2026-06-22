// Tests for the sm2_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sm2_encrypt::

use rxchef::operation::{ArgValue, OperationError};
use rxchef::operations::sm2_encrypt::Sm2Encrypt;
use rxchef::Operation;

#[test]
fn test_sm2_encrypt_invalid_key() {
    let op = Sm2Encrypt;
    let args = [
        ArgValue::Str("SHORT".to_string()),
        ArgValue::Str("DEADBEEF".to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}
#[test]
fn test_sm2_encrypt_missing_key() {
    let op = Sm2Encrypt;
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}
#[test]
fn test_sm2_encrypt_placeholder_error() {
    let op = Sm2Encrypt;
    let key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let args = [
        ArgValue::Str(key.to_string()),
        ArgValue::Str(key.to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
    if let Err(OperationError::ProcessingError(msg)) = result {
        assert!(msg.contains("placeholder"));
    } else {
        panic!("Expected ProcessingError");
    }
}
