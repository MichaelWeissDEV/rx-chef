// Tests for the x_salsa20 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations x_salsa20::

use rxchef::operation::ArgValue;
use rxchef::operations::x_salsa20::XSalsa20Op;
use rxchef::Operation;

#[test]
fn test_xsalsa20_encryption() {
    let op = XSalsa20Op;
    // Key: 32 bytes of 0x01
    let key = vec![0x01; 32];
    // Nonce: 24 bytes of 0x02
    let nonce = vec![0x02; 24];
    let input = b"Hello World!".to_vec();
    let args = [
        ArgValue::Str(hex::encode(&key)),
        ArgValue::Str(hex::encode(&nonce)),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    assert_ne!(result, input);
    // Decrypt
    let decrypted = op.run(result, &args).unwrap();
    assert_eq!(decrypted, input);
}
#[test]
fn test_xsalsa20_invalid_key_length() {
    let op = XSalsa20Op;
    let input = b"test".to_vec();
    let args = [
        ArgValue::Str("01020304".to_string()), // Too short
        ArgValue::Str("010203040506070809101112131415161718192021222324".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_xsalsa20_invalid_nonce_length() {
    let op = XSalsa20Op;
    let input = b"test".to_vec();
    let args = [
        ArgValue::Str(
            "0102030405060708091011121314151617181920212223242526272829303132".to_string(),
        ),
        ArgValue::Str("0102".to_string()), // Too short
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
