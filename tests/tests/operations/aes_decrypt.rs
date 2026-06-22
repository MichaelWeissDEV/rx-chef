// Tests for the aes_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations aes_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::aes_decrypt::AesDecrypt;
use rxchef::Operation;

#[test]
fn test_aes_decrypt_invalid_key_length() {
    let op = AesDecrypt;
    let input = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
    let args = [
        ArgValue::Str("01234".to_string()), // 5 bytes - invalid
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_aes_decrypt_invalid_padding() {
    let op = AesDecrypt;
    // Invalid padding - last byte is 20 but it's not valid PKCS#7
    let input = hex::decode("000102030405060708090a0b0c0d0e0f14").unwrap();
    let key = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
    let args = [
        ArgValue::Bytes(key),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
