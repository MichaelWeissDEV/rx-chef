// Tests for the to_base62 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base62::

use rxchef::operations::to_base62::ToBase62;
use rxchef::Operation;

fn run(input: Vec<u8>, args: &[rxchef::operation::ArgValue]) -> Vec<u8> {
    let op = ToBase62;
    op.run(input, args).unwrap()
}

#[test]
fn test_to_base62_empty_input() {
    let op = ToBase62;
    let args = [];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_to_base62_zero() {
    let input = vec![0x00];
    let args = [];
    let result = run(input.to_vec(), &args);
    // [0x00] is treated as zero, which results in empty output (correct behavior)
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_to_base62_single_byte() {
    let input = vec![0x01];
    let args = [];
    let result = run(input.to_vec(), &args);
    assert_eq!(result, "1".as_bytes());
}

#[test]
fn test_to_base62_small_number() {
    let input = vec![0x0A]; // 10 in decimal
    let args = [];
    let result = run(input.to_vec(), &args);
    assert_eq!(result, "A".as_bytes());
}

#[test]
fn test_to_base62_large_number() {
    let input = vec![0x3E]; // 62 in decimal (should be "10" in base62)
    let args = [];
    let result = run(input.to_vec(), &args);
    assert_eq!(result, "10".as_bytes());
}

#[test]
fn test_to_base62_two_bytes() {
    let input = vec![0x00, 0x40]; // 64 in decimal (should be "12" in base62)
    let args = [];
    let result = run(input.to_vec(), &args);
    assert_eq!(result, "12".as_bytes());
}

#[test]
fn test_to_base62_text() {
    let input = "Hello".as_bytes();
    let args = [];
    let result = run(input.to_vec(), &args);
    // "Hello" in ASCII is [72, 101, 108, 108, 111]
    // Actual calculated base62 value
    let expected = "5TP3P3v"; // Actual calculated value
    assert_eq!(result, expected.as_bytes());
}

#[test]
fn test_to_base62_large_input() {
    let input = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Large number
    let args = [];
    let result = run(input.to_vec(), &args);
    // Should produce a valid base62 string
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.len() > 0);
    // Should only contain base62 characters (0-9, A-Z, a-z)
    for ch in result_str.chars() {
        assert!(ch.is_ascii_alphanumeric());
    }
}

#[test]
fn test_to_base62_max_byte() {
    let input = vec![0xFF]; // 255 in decimal
    let args = [];
    let result = run(input.to_vec(), &args);
    assert_eq!(result, "47".as_bytes()); // 255 in base62
}

#[test]
fn test_to_base62_multiple_bytes() {
    let input = vec![0x01, 0x00, 0x00]; // 65536 in decimal
    let args = [];
    let result = run(input.to_vec(), &args);
    // 65536 in base62 should be "H34" (actual calculated value)
    assert_eq!(result, "H32".as_bytes());
}

#[test]
fn test_to_base62_binary_data() {
    let input = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD];
    let args = [];
    let result = run(input.to_vec(), &args);
    // Should produce a valid base62 string
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.len() > 0);
    // Should only contain base62 characters
    for ch in result_str.chars() {
        assert!(ch.is_ascii_alphanumeric());
    }
}

#[test]
fn test_to_base62_unicode_bytes() {
    let input = "Hello 世界".as_bytes(); // UTF-8 encoded
    let args = [];
    let result = run(input.to_vec(), &args);
    // Should produce a valid base62 string
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.len() > 0);
    // Should only contain base62 characters
    for ch in result_str.chars() {
        assert!(ch.is_ascii_alphanumeric());
    }
}
