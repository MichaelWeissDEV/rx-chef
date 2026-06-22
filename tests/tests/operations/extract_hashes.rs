// Tests for the extract_hashes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_hashes::

use rxchef::operation::ArgValue;
use rxchef::operations::extract_hashes::ExtractHashes;
use rxchef::Operation;

#[test]
fn test_extract_hashes_basic() {
    let op = ExtractHashes;
    let input = b"Here is a hash: 7cf3273e84e55e513066a2a537021d96009ed945 and another one: 1234567890abcdef1234567890abcdef12345678".to_vec();
    let args = &[
        ArgValue::Num(40.0),   // Hash character length
        ArgValue::Bool(false), // All hashes
        ArgValue::Bool(false), // Display Total
    ];
    let result = op.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(
        output,
        "7cf3273e84e55e513066a2a537021d96009ed945\n1234567890abcdef1234567890abcdef12345678"
    );
}
#[test]
fn test_extract_hashes_all() {
    let op = ExtractHashes;
    let input =
        b"md5: 81dc9bdb52d04dc20036dbd8313ed055, sha1: 7cf3273e84e55e513066a2a537021d96009ed945"
            .to_vec();
    let args = &[
        ArgValue::Num(0.0),    // Ignored
        ArgValue::Bool(true),  // All hashes
        ArgValue::Bool(false), // Display Total
    ];
    let result = op.run(input, args).unwrap();
    let output = String::from_utf8(result).unwrap();
    // md5 (32 chars) and sha1 (40 chars) should be found.
    assert!(output.contains("81dc9bdb52d04dc20036dbd8313ed055"));
    assert!(output.contains("7cf3273e84e55e513066a2a537021d96009ed945"));
}
