// Tests for the from_base58 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base58::

use rxchef::operations::from_base58::FromBase58;
use rxchef::Operation;

#[test]
fn test_from_base58_empty_input() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str(
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string(),
        ),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base58_simple_decode() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str(
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string(),
        ),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // "a" is index 33 in the Bitcoin alphabet, which is the single byte 0x21.
    // Value cross-checked against CyberChef 11.0.0.
    let base58_input = "a";
    let decoded = op.run(base58_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x21]);
}

#[test]
fn test_from_base58_with_cleaning() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str(
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string(),
        ),
        rxchef::operation::ArgValue::Bool(true), // Remove non-alphabet chars
    ];
    // Non-alphabet characters are stripped, leaving "abc".
    // Value cross-checked against CyberChef 11.0.0.
    let base58_input = "a!b@c";
    let decoded = op.run(base58_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x01, 0xb9, 0x7b]);
}

#[test]
fn test_from_base58_invalid_characters_no_cleaning() {
    let op = FromBase58;
    let args = [
        rxchef::operation::ArgValue::Str(
            "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string(),
        ),
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
        rxchef::operation::ArgValue::Str(
            "rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz".to_string(),
        ),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // "a" sits at a different index in the Flickr alphabet, giving 0x05
    // rather than the Bitcoin alphabet's 0x21.
    // Value cross-checked against CyberChef 11.0.0.
    let base58_input = "a";
    let decoded = op.run(base58_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x05]);
}
