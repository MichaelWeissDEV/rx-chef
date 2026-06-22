// Tests for the xor_brute_force operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations xor_brute_force::

use rxchef::operation::ArgValue;
use rxchef::operations::xor_brute_force::XORBruteForce;
use rxchef::Operation;

#[test]
fn test_xor_bf_finds_key() {
    // XOR "hello" with key 0x42 then brute-force to recover it
    let key = 0x42u8;
    let plaintext = b"hello";
    let ciphertext: Vec<u8> = plaintext.iter().map(|&b| b ^ key).collect();
    let op = XORBruteForce;
    let args = [
        ArgValue::Num(1.0),   // key_length
        ArgValue::Num(100.0), // sample_length
        ArgValue::Num(0.0),   // sample_offset
        ArgValue::Str("Standard".to_string()),
        ArgValue::Bool(false),              // null_preserving
        ArgValue::Bool(true),               // print_key
        ArgValue::Bool(false),              // output_hex
        ArgValue::Str("hello".to_string()), // crib
    ];
    let result = String::from_utf8(op.run(ciphertext, &args).unwrap()).unwrap();
    // Should find at least one line containing "Key = 42:"
    assert!(
        result.contains("Key = 42:"),
        "Expected key 42 in: {}",
        result
    );
    assert!(result.to_lowercase().contains("hello"));
}
#[test]
fn test_xor_bf_empty_input() {
    let op = XORBruteForce;
    let result = op.run(vec![], &[]).unwrap();
    // All 255 XOR keys applied to empty input produce empty strings
    let s = String::from_utf8(result).unwrap();
    // Should have 255 lines (key 1..255), each with empty result
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 255);
}
#[test]
fn test_xor_bf_crib_filters() {
    let op = XORBruteForce;
    // XOR "AAAA" with 0x01
    let input: Vec<u8> = b"AAAA".iter().map(|&b| b ^ 0x01u8).collect();
    let args = [
        ArgValue::Num(1.0),
        ArgValue::Num(100.0),
        ArgValue::Num(0.0),
        ArgValue::Str("Standard".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Str("AAAA".to_string()),
    ];
    let result = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    // Only the line for key=01 should be present
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("01"));
}
#[test]
fn test_xor_bf_hex_output() {
    let op = XORBruteForce;
    let input = vec![0xff]; // 0xff ^ 0x01 = 0xfe
    let args = [
        ArgValue::Num(1.0),
        ArgValue::Num(100.0),
        ArgValue::Num(0.0),
        ArgValue::Str("Standard".to_string()),
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(true), // output_hex = true
        ArgValue::Str("".to_string()),
    ];
    let result = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    // Line for key=01 should show "fe"
    assert!(result.lines().any(|l| l.contains("01") && l.contains("fe")));
}
