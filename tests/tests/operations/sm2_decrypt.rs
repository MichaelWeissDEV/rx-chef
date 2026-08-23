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

#[test]
fn test_sm2_decrypt_matches_pinned_upstream_vector() {
    let ciphertext = b"9a31bc0adb4677cdc4141479e3949572a55c3e6fb52094721f741c2bd2e179aaa87be6263bc1be602e473be3d5de5dce97f8248948b3a7e15f9f67f64aef21575e0c05e6171870a10ff9ab778dbef24267ad90e1a9d47d68f757d57c4816612e9829f804025dea05a511cda39371c22a2828f976f72e";
    let args = [
        ArgValue::Str("e74a72505084c3269aa9b696d603e3e08c74c6740212c11a31e26cdfe08bdf6a".into()),
        ArgValue::Str("C1C3C2".into()),
        ArgValue::Str("sm2p256v1".into()),
    ];
    assert_eq!(Sm2Decrypt.run(ciphertext.to_vec(), &args).unwrap(), b"I am a small plaintext");
}
