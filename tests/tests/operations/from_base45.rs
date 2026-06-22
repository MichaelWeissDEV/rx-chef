// Tests for the from_base45 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_base45::

use rxchef::operations::from_base45::FromBase45;
use rxchef::Operation;

#[test]
fn test_from_base45_empty_input() {
    let op = FromBase45;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Z $%*+-./:".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_base45_simple_decode() {
    let op = FromBase45;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Z $%*+-./:".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Simple Base45 encoding: "AB" -> should decode to some bytes
    let base45_input = "AB";
    let result = op.run(base45_input.as_bytes().to_vec(), &args);
    // Should successfully decode
    assert!(result.is_ok());
}

#[test]
fn test_from_base45_with_cleaning() {
    let op = FromBase45;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Z $%*+-./:".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Remove non-alphabet chars
    ];
    // Base45 with invalid characters
    let base45_input = "A!B@C";
    let result = op.run(base45_input.as_bytes().to_vec(), &args);
    // Should successfully decode after removing invalid chars
    assert!(result.is_ok());
}

#[test]
fn test_from_base45_invalid_characters_no_cleaning() {
    let op = FromBase45;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Z $%*+-./:".to_string()),
        rxchef::operation::ArgValue::Bool(false), // Don't remove non-alphabet chars
    ];
    // Base45 with invalid characters
    let base45_input = "A!B";
    let result = op.run(base45_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid characters
    assert!(result.is_err());
}

#[test]
fn test_from_base45_triplet_decode() {
    let op = FromBase45;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Z $%*+-./:".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Base45 triplet that should decode to some bytes
    let base45_input = "AAA"; // 0*45^2 + 0*45 + 0 = 0
    let result = op.run(base45_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert!(!decoded.is_empty());
}
