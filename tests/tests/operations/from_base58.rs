// Tests for the from_base58 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base58::

use rxchef::operations::from_base58::FromBase58;
use rxchef::Operation;

#[test]
fn test_from_base58_empty_input() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base58_simple_decode() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Simple Base58 encoding: "a" -> should decode to some bytes
    let base58_input = "a";
    let result = op.run(base58_input.as_bytes().to_vec(), &args);
    // Should successfully decode
    assert!(result.is_ok());
}

#[test]
fn test_from_base58_with_cleaning() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Remove non-alphabet chars
    ];
    // Base58 with invalid characters
    let base58_input = "a!b@c";
    let result = op.run(base58_input.as_bytes().to_vec(), &args);
    // Should successfully decode after removing invalid chars
    assert!(result.is_ok());
}

#[test]
fn test_from_base58_invalid_characters_no_cleaning() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string()),
        rxchef::operation::ArgValue::Bool(false), // Don't remove non-alphabet chars
    ];
    // Base58 with invalid characters
    let base58_input = "a!b";
    let result = op.run(base58_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid characters
    assert!(result.is_err());
}

#[test]
fn test_from_base58_different_alphabet() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str("rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Using Flickr alphabet
    let base58_input = "a";
    let result = op.run(base58_input.as_bytes().to_vec(), &args);
    // Should decode successfully with the different alphabet
    assert!(result.is_ok());
}
