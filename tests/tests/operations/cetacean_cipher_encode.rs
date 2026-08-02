// Tests for the cetacean_cipher_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cetacean_cipher_encode::

use rxchef::operations::cetacean_cipher_encode::CetaceanCipherEncode;
use rxchef::Operation;

fn run(input: &str) -> String {
    let op = CetaceanCipherEncode;
    String::from_utf8(op.run(input.as_bytes().to_vec(), &[]).unwrap()).unwrap()
}

#[test]
fn test_cetacean_encode_empty() {
    let result = run("");
    assert_eq!(result, "");
}

#[test]
fn test_cetacean_encode_single_char() {
    // 'h' = ASCII 104 = binary 01101000
    // Should be encoded as EEEEEEEEeeEeEEEE (16 bits)
    let result = run("h");
    assert_eq!(result.len(), 16); // 16 bits per character
    assert!(result.contains('E'));
    assert!(result.contains('e'));
}

#[test]
fn test_cetacean_encode_space() {
    // Space should be encoded as a single space character
    let result = run(" ");
    assert_eq!(result, " ");
}

#[test]
fn test_cetacean_encode_simple_word() {
    // Test encoding a simple word
    let result = run("hi");
    // Should be 32 characters (16 bits per character)
    assert_eq!(result.len(), 32);
    assert!(result.contains('E'));
    assert!(result.contains('e'));
}

#[test]
fn test_cetacean_encode_mixed_case() {
    // Test with mixed case letters
    let result = run("HeLLo");
    assert_eq!(result.len(), 80); // 5 characters * 16 bits
}

#[test]
fn test_cetacean_encode_special_characters() {
    // Test with special characters
    let result = run("!@#$%");
    assert_eq!(result.len(), 80); // 5 characters * 16 bits
}

#[test]
fn test_cetacean_encode_unicode() {
    // Test with Unicode characters
    let result = run("世界");
    // Each Unicode character will be encoded as 16 bits
    assert_eq!(result.len(), 32); // 2 characters * 16 bits
}

#[test]
fn test_cetacean_encode_invalid_utf8() {
    let op = CetaceanCipherEncode;
    // Create invalid UTF-8 input
    let input = vec![0xFF, 0xFE]; // Invalid UTF-8 sequence
    let result = op.run(input, &[]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid UTF-8"));
}

#[test]
fn test_cetacean_encode_roundtrip() {
    // Test that encoding and then decoding gives us back the original
    let op_encode = CetaceanCipherEncode;
    let op_decode = rxchef::operations::cetacean_cipher_decode::CetaceanCipherDecode;
    
    let original = "hello";
    let encoded = op_encode.run(original.as_bytes().to_vec(), &[]).unwrap();
    let decoded = op_decode.run(encoded, &[]).unwrap();
    
    assert_eq!(String::from_utf8(decoded).unwrap(), original);
}

#[test]
fn test_cetacean_encode_all_zeros() {
    // Test encoding null character (all zeros)
    let result = run("\0");
    // Should be 16 'E's (all zeros)
    assert_eq!(result, "EEEEEEEEEEEEEEEE");
}

#[test]
fn test_cetacean_encode_all_ones() {
    // Test encoding character with all ones in lower 16 bits
    let result = run("\u{FFFF}");
    // Should be 16 'e's (all ones)
    assert_eq!(result, "eeeeeeeeeeeeeeee");
}
