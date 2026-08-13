// Tests for the sm2_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sm2_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::sm2_decrypt::Sm2Decrypt;
use rxchef::Operation;

#[test]
fn test_sm2_decrypt_invalid_key() {
    let op = Sm2Decrypt;
    let args = [
        ArgValue::Str("SHORT".to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}
#[test]
fn test_sm2_decrypt_no_key() {
    let op = Sm2Decrypt;
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str("C1C3C2".to_string()),
        ArgValue::Str("sm2p256v1".to_string()),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}
