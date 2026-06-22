// Tests for the text_encoding_brute_force operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations text_encoding_brute_force::

use rxchef::operation::ArgValue;
use rxchef::operations::text_encoding_brute_force::TextEncodingBruteForce;
use rxchef::Operation;
use std::collections::BTreeMap;

#[test]
fn test_text_encoding_brute_force_decode() {
    let op = TextEncodingBruteForce;
    let input = b"Hello".to_vec();
    let args = [ArgValue::Str("Decode".to_string())];
    let result = op.run(input, &args).unwrap();
    let results: BTreeMap<String, String> = serde_json::from_slice(&result).unwrap();
    assert_eq!(results.get("UTF-8").unwrap(), "Hello");
}
#[test]
fn test_text_encoding_brute_force_encode() {
    let op = TextEncodingBruteForce;
    let input = "Hello".as_bytes().to_vec();
    let args = [ArgValue::Str("Encode".to_string())];
    let result = op.run(input, &args).unwrap();
    let results: BTreeMap<String, String> = serde_json::from_slice(&result).unwrap();
    assert_eq!(results.get("UTF-8").unwrap(), "Hello");
}
#[test]
fn test_text_encoding_brute_force_malformed() {
    let op = TextEncodingBruteForce;
    let input = vec![0xFF, 0xFE, 0xFD];
    let args = [ArgValue::Str("Decode".to_string())];
    let result = op.run(input, &args).unwrap();
    let _results: BTreeMap<String, String> = serde_json::from_slice(&result).unwrap();
    // Just check it doesn't crash
}
