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
    // RFC 9285: a two-character group decodes to one byte.
    // 'A'=10, 'B'=11 -> 10 + 11*45 = 505 -> 0xF9.
    // Value cross-checked against CyberChef 11.0.0.
    let base45_input = "AB";
    let decoded = op.run(base45_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0xf9]);
}

#[test]
fn test_from_base45_with_cleaning() {
    let op = FromBase45;
    let args = [
        rxchef::operation::ArgValue::Str("0-9A-Z $%*+-./:".to_string()),
        rxchef::operation::ArgValue::Bool(true), // Remove non-alphabet chars
    ];
    // Non-alphabet characters are stripped, leaving "ABC", which is the
    // same three-character group as the triplet test below.
    let base45_input = "A!B@C";
    let stripped = op.run(b"ABC".to_vec(), &args).unwrap();
    let decoded = op.run(base45_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, stripped);
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
    // RFC 9285: a three-character group decodes to two bytes.
    // 'A'=10 -> 10 + 10*45 + 10*45^2 = 20710 = 0x50E6.
    // Value cross-checked against CyberChef 11.0.0.
    let base45_input = "AAA";
    let decoded = op.run(base45_input.as_bytes().to_vec(), &args).unwrap();
    assert_eq!(decoded, [0x50, 0xe6]);
}
