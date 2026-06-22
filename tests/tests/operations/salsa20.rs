// Tests for the salsa20 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations salsa20::

use rxchef::operation::ArgValue;
use rxchef::operations::salsa20::Salsa20Op;
use rxchef::Operation;

#[test]
fn test_salsa20_basic() {
    let op = Salsa20Op;
    let input = b"Hello World".to_vec();
    let key = "000102030405060708090a0b0c0d0e0f".to_string(); // 16 bytes
    let nonce = "0011223344556677".to_string(); // 8 bytes
    let args = [
        ArgValue::Str(key.clone()),
        ArgValue::Str(nonce.clone()),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let encrypted = op.run(input.clone(), &args).unwrap();
    assert_ne!(input, encrypted);
    let decrypted = op.run(encrypted, &args).unwrap();
    assert_eq!(input, decrypted);
}
#[test]
fn test_salsa20_hex_output() {
    let op = Salsa20Op;
    let input = b"Hello".to_vec();
    let key = "000102030405060708090a0b0c0d0e0f".to_string();
    let nonce = "0011223344556677".to_string();
    let args = [
        ArgValue::Str(key),
        ArgValue::Str(nonce),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(result.iter().all(|&b| (b as char).is_ascii_hexdigit()));
}
#[test]
fn test_salsa20_invalid_key() {
    let op = Salsa20Op;
    let input = b"Hello".to_vec();
    let args = [
        ArgValue::Str("010203".to_string()), // Too short
        ArgValue::Str("0011223344556677".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("20".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
