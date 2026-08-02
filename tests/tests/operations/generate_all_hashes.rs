// Tests for the generate_all_hashes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_all_hashes::

use rxchef::operations::generate_all_hashes::GenerateAllHashes;
use rxchef::Operation;

#[test]
fn test_generate_all_hashes_empty_input() {
    let op = GenerateAllHashes;
    let args = [
        rxchef::operation::ArgValue::Str("All".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Empty input should still generate hashes
    assert!(output_str.contains("MD5"));
    assert!(output_str.contains("SHA1"));
}

#[test]
fn test_generate_all_hashes_simple_input() {
    let op = GenerateAllHashes;
    let args = [
        rxchef::operation::ArgValue::Str("All".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Simple input
    let input = b"hello";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Should contain various hashes
    assert!(output_str.contains("MD5"));
    assert!(output_str.contains("SHA1"));
    assert!(output_str.contains("SHA2 256"));
}

#[test]
fn test_generate_all_hashes_without_names() {
    let op = GenerateAllHashes;
    let args = [
        rxchef::operation::ArgValue::Str("All".to_string()),
        rxchef::operation::ArgValue::Bool(false), // Don't include names
    ];
    // Simple input
    let input = b"test";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Should contain hex values but not algorithm names
    assert!(!output_str.contains("MD5"));
    assert!(output_str.contains("\n"));
}

#[test]
fn test_generate_all_hashes_specific_length() {
    let op = GenerateAllHashes;
    let args = [
        rxchef::operation::ArgValue::Str("256".to_string()), // Only 256-bit hashes
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Simple input
    let input = b"data";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Should contain 256-bit hashes
    assert!(output_str.contains("SHA2 256"));
    assert!(output_str.contains("SHA3 256"));
}

#[test]
fn test_generate_all_hashes_binary_input() {
    let op = GenerateAllHashes;
    let args = [
        rxchef::operation::ArgValue::Str("All".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Binary input with null bytes
    let input = vec![0x00, 0x01, 0x02, 0x03];
    let result = op.run(input, &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
}
