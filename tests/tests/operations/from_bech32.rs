// Tests for the from_bech32 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_bech32::

use rxchef::operations::from_bech32::FromBech32;
use rxchef::Operation;

#[test]
fn test_from_bech32_empty_input() {
    let op = FromBech32;
    let args = [
        rxchef::operation::ArgValue::Str("Auto-detect".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_bech32_valid_address() {
    let op = FromBech32;
    let args = [
        rxchef::operation::ArgValue::Str("Auto-detect".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    // Valid Bech32 address
    let bech32_input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
    let result = op.run(bech32_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert!(!decoded.is_empty());
}

#[test]
fn test_from_bech32_hex_output() {
    let op = FromBech32;
    let args = [
        rxchef::operation::ArgValue::Str("Auto-detect".to_string()),
        rxchef::operation::ArgValue::Str("Hex".to_string()),
    ];
    // Valid Bech32 address
    let bech32_input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
    let result = op.run(bech32_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let hex_output = result.unwrap();
    assert!(!hex_output.is_empty());
    // Should be valid hex
    assert!(String::from_utf8_lossy(&hex_output).chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_from_bech32_invalid_checksum() {
    let op = FromBech32;
    let args = [
        rxchef::operation::ArgValue::Str("Auto-detect".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    // Invalid Bech32 address with bad checksum
    let bech32_input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t5"; // Changed last char
    let result = op.run(bech32_input.as_bytes().to_vec(), &args);
    // Should fail due to invalid checksum
    assert!(result.is_err());
}

#[test]
fn test_from_bech32_mixed_case() {
    let op = FromBech32;
    let args = [
        rxchef::operation::ArgValue::Str("Auto-detect".to_string()),
        rxchef::operation::ArgValue::Str("Raw".to_string()),
    ];
    // Bech32 with mixed case (should be invalid)
    let bech32_input = "BC1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
    let result = op.run(bech32_input.as_bytes().to_vec(), &args);
    // Should fail due to mixed case
    assert!(result.is_err());
}
