// Tests for the from_hex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_hex::

use rxchef::operations::from_hex::FromHex;
use rxchef::Operation;

#[test]
fn test_from_hex_empty_input() {
    let op = FromHex;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_hex_simple_decode() {
    let op = FromHex;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
    ];
    // Simple hex: "48656c6c6f" -> "Hello"
    let hex_input = "48656c6c6f";
    let result = op.run(hex_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_hex_with_delimiter() {
    let op = FromHex;
    let args = [
        rxchef::operation::ArgValue::Str("Space".to_string()),
    ];
    // Hex with space delimiter: "48 65 6c 6c 6f" -> "Hello"
    let hex_input = "48 65 6c 6c 6f";
    let result = op.run(hex_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_hex_with_prefix() {
    let op = FromHex;
    let args = [
        rxchef::operation::ArgValue::Str("0x".to_string()),
    ];
    // Hex with 0x prefix: "0x480x650x6c0x6c0x6f" -> "Hello"
    let hex_input = "0x480x650x6c0x6c0x6f";
    let result = op.run(hex_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(String::from_utf8_lossy(&decoded), "Hello");
}

#[test]
fn test_from_hex_odd_length() {
    let op = FromHex;
    let args = [
        rxchef::operation::ArgValue::Str("Auto".to_string()),
    ];
    // Hex with odd length - should ignore last nibble
    let hex_input = "48656";
    let result = op.run(hex_input.as_bytes().to_vec(), &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    assert_eq!(decoded, vec![0x48, 0x65]); // "He"
}
