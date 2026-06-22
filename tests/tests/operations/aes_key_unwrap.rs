// Tests for the aes_key_unwrap operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations aes_key_unwrap::

use rxchef::operation::ArgValue;
use rxchef::operations::aes_key_unwrap::AesKeyUnwrap;
use rxchef::Operation;

#[test]
fn test_aes_key_unwrap_invalid_kek_length() {
    let op = AesKeyUnwrap;
    // Wrapped key with valid length (24 bytes minimum)
    let input = hex::decode("a6a6a6a6a6a6a6a600112233445566778899aabbccddeeff").unwrap();
    let args = [
        ArgValue::Str("01234".to_string()), // 5 bytes - invalid
        ArgValue::Str("a6a6a6a6a6a6a6a6".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_aes_key_unwrap_invalid_iv_length() {
    let op = AesKeyUnwrap;
    let input = hex::decode("a6a6a6a6a6a6a6a600112233445566778899aabbccddeeff").unwrap();
    let args = [
        ArgValue::Str("000102030405060708090a0b0c0d0e0f".to_string()),
        ArgValue::Str("a6a6a6a6a6a6a6".to_string()), // 7 bytes - invalid
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_aes_key_unwrap_invalid_input_length() {
    let op = AesKeyUnwrap;
    let input = hex::decode("a6a6a6a6a6a6a6a60011223344556677").unwrap(); // 16 bytes - too small
    let args = [
        ArgValue::Str("000102030405060708090a0b0c0d0e0f".to_string()),
        ArgValue::Str("a6a6a6a6a6a6a6a6".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_aes_key_unwrap_with_invalid_iv() {
    let op = AesKeyUnwrap;
    // Wrapped key with wrong IV
    let input = hex::decode("001122334455667700112233445566778899aabbccddeeff").unwrap();
    let args = [
        ArgValue::Str("000102030405060708090a0b0c0d0e0f".to_string()),
        ArgValue::Str("a6a6a6a6a6a6a6a6".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_aes_key_unwrap_roundtrip_128bit_kek() {
    let wrap_op = rxchef::operations::aes_key_wrap::AesKeyWrap;
    let unwrap_op = AesKeyUnwrap;
    // 16-byte key to wrap (2 blocks)
    let input_key = hex::decode("00112233445566778899aabbccddeeff").unwrap();
    // 16-byte KEK
    let kek = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
    // Default IV
    let iv = hex::decode("a6a6a6a6a6a6a6a6").unwrap();
    // Wrap
    let wrap_args = [
        ArgValue::Bytes(kek.clone()),
        ArgValue::Bytes(iv.clone()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let wrapped = wrap_op.run(input_key.clone(), &wrap_args).unwrap();
    // Unwrap
    let unwrap_args = [
        ArgValue::Bytes(kek),
        ArgValue::Bytes(iv),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let unwrapped = unwrap_op.run(wrapped, &unwrap_args).unwrap();
    assert_eq!(input_key, unwrapped);
}
#[test]
fn test_aes_key_unwrap_roundtrip_256bit_kek() {
    let wrap_op = rxchef::operations::aes_key_wrap::AesKeyWrap;
    let unwrap_op = AesKeyUnwrap;
    // 24-byte key to wrap (3 blocks)
    let input_key = hex::decode("00112233445566778899aabbccddeeff0011223344556677").unwrap();
    // 32-byte KEK
    let kek =
        hex::decode("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f").unwrap();
    // Default IV
    let iv = hex::decode("a6a6a6a6a6a6a6a6").unwrap();
    // Wrap
    let wrap_args = [
        ArgValue::Bytes(kek.clone()),
        ArgValue::Bytes(iv.clone()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let wrapped = wrap_op.run(input_key.clone(), &wrap_args).unwrap();
    // Unwrap
    let unwrap_args = [
        ArgValue::Bytes(kek),
        ArgValue::Bytes(iv),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let unwrapped = unwrap_op.run(wrapped, &unwrap_args).unwrap();
    assert_eq!(input_key, unwrapped);
}
