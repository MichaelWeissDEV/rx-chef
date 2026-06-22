// Tests for the generate_all_checksums operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_all_checksums::

use rxchef::operations::generate_all_checksums::GenerateAllChecksums;
use rxchef::Operation;

#[test]
fn test_generate_all_checksums_empty_input() {
    let op = GenerateAllChecksums;
    let args = [
        rxchef::operation::ArgValue::Str("All".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Empty input should still generate checksums (for empty data)
    assert!(output_str.contains("Fletcher-8"));
    assert!(output_str.contains("00"));
}

#[test]
fn test_generate_all_checksums_simple_input() {
    let op = GenerateAllChecksums;
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
    // Should contain some checksums
    assert!(output_str.contains("Fletcher-8"));
    assert!(output_str.contains("Adler-32"));
}

#[test]
fn test_generate_all_checksums_without_names() {
    let op = GenerateAllChecksums;
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
    assert!(!output_str.contains("Fletcher"));
    assert!(output_str.contains("\n"));
}

#[test]
fn test_generate_all_checksums_specific_length() {
    let op = GenerateAllChecksums;
    let args = [
        rxchef::operation::ArgValue::Str("32".to_string()), // Only 32-bit checksums
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Simple input
    let input = b"data";
    let result = op.run(input.to_vec(), &args);
    assert!(result.is_ok());
    let output = result.unwrap();
    let output_str = String::from_utf8_lossy(&output);
    // Should contain 32-bit checksums
    assert!(output_str.contains("Adler-32"));
    assert!(output_str.contains("Fletcher-32"));
}

#[test]
fn test_generate_all_checksums_binary_input() {
    let op = GenerateAllChecksums;
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
