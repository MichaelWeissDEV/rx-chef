// Tests for the decode_netbios_name operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations decode_netbios_name::

use rxchef::operations::decode_netbios_name::DecodeNetBIOSName;
use rxchef::Operation;

fn run(input: &[u8], offset: i64) -> Vec<u8> {
    let op = DecodeNetBIOSName;
    let args = [rxchef::operation::ArgValue::Num(offset as f64)];
    op.run(input.to_vec(), &args).unwrap()
}

#[test]
fn test_decode_netbios_empty_input() {
    let result = run(&[], 65);
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_decode_netbios_basic_encoding() {
    // Test basic NetBIOS name decoding
    // The algorithm: each pair of bytes represents high and low nibbles
    // Each nibble has the offset subtracted, then combined: (high << 4) | (low & 0xf)
    
    // Let's test a simple case with offset 65:
    // Input bytes: (70, 70)
    // high = 70 - 65 = 5, low = 70 - 65 = 5
    // result = (5 << 4) | (5 & 0xf) = 80 | 5 = 85
    let input = vec![70, 70];
    let result = run(&input, 65);
    assert_eq!(result, vec![85]); // Should decode to 85
}

#[test]
fn test_decode_netbios_offset_variation() {
    // Test with different offset values
    let input = vec![70, 70];
    
    // Test offset 64
    let result = run(&input, 64);
    // high = 70 - 64 = 6, low = 70 - 64 = 6
    // result = (6 << 4) | (6 & 0xf) = 96 | 6 = 102
    assert_eq!(result, vec![102]); // Different decoding with offset 64
}

#[test]
fn test_decode_netbios_longer_name() {
    // Test decoding multiple byte pairs
    // Pair 1: (70, 71) -> high=5, low=6 -> (5<<4)|6 = 86
    // Pair 2: (72, 73) -> high=7, low=8 -> (7<<4)|8 = 120
    let input = vec![70, 71, 72, 73];
    let result = run(&input, 65);
    assert_eq!(result, vec![86, 120]); // Two decoded bytes
}

#[test]
fn test_decode_netbios_trailing_spaces() {
    // Test that trailing spaces (value 32) are removed
    // Pair 1: (70, 71) -> 86
    // Pair 2: (68, 67) -> high=3, low=2 -> (3<<4)|2 = 50 (which is not space, so not removed)
    // Let's create a pair that decodes to space (32):
    // To get 32: high=2, low=0 -> encoded: (2+65, 0+65) = (67, 65)
    let input = vec![70, 71, 67, 65]; // Second pair decodes to space
    let result = run(&input, 65);
    assert_eq!(result, vec![86]); // Trailing space should be removed
}

#[test]
fn test_decode_netbios_multiple_trailing_spaces() {
    // Test removal of multiple trailing spaces
    // First pair: (70, 71) -> 86
    // Second pair: (67, 65) -> space (32)
    // Third pair: (67, 65) -> space (32)
    let input = vec![70, 71, 67, 65, 67, 65];
    let result = run(&input, 65);
    assert_eq!(result, vec![86]); // All trailing spaces removed
}

#[test]
fn test_decode_netbios_odd_length_input() {
    // Test with odd length input (should not process)
    let input = vec![71, 70, 72]; // Odd length
    let result = run(&input, 65);
    assert_eq!(result, Vec::<u8>::new()); // Should return empty
}

#[test]
fn test_decode_netbios_long_input() {
    // Test with input longer than 32 bytes (should not process)
    let mut input = Vec::new();
    for i in 0..40 {
        input.push(70 + (i % 10));
    }
    let result = run(&input, 65);
    assert_eq!(result, Vec::<u8>::new()); // Should return empty for >32 bytes
}

#[test]
fn test_decode_netbios_wrapping_arithmetic() {
    // Test wrapping arithmetic with values that would underflow
    let input = vec![10, 10]; // Values less than offset
    let result = run(&input, 65);
    // Should wrap: (10-65) = negative, wrapping to high value
    let expected_high = (10 as u8).wrapping_sub(65) << 4;
    let expected_low = (10 as u8).wrapping_sub(65) & 0xf;
    let expected = expected_high | expected_low;
    assert_eq!(result, vec![expected]);
}

#[test]
fn test_decode_netbios_invalid_offset() {
    let op = DecodeNetBIOSName;
    let input = vec![71, 70];
    let args = [rxchef::operation::ArgValue::Str("invalid".to_string())]; // Invalid offset
    let result = op.run(input, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Offset must be a number"));
}

#[test]
fn test_decode_netbios_zero_offset() {
    // Test with zero offset
    let input = vec![10, 10];
    let result = run(&input, 0);
    let expected_high = 10 << 4;
    let expected_low = 10 & 0xf;
    let expected = expected_high | expected_low;
    assert_eq!(result, vec![expected]);
}

#[test]
fn test_decode_netbios_large_offset() {
    // Test with large offset
    let input = vec![200, 200]; // Values that would wrap
    let result = run(&input, 100);
    let expected_high = (200 as u8).wrapping_sub(100) << 4;
    let expected_low = (200 as u8).wrapping_sub(100) & 0xf;
    let expected = expected_high | expected_low;
    assert_eq!(result, vec![expected]);
}

#[test]
fn test_decode_netbios_realistic_example() {
    // Test with values that might appear in real NetBIOS encoding
    // Using offset 65 (standard NetBIOS offset)
    let input = vec![
        75, 74,  // Some encoded bytes
        80, 79,  // More encoded bytes
    ];
    let result = run(&input, 65);
    assert_eq!(result.len(), 2);
    // Just verify it produces some output without panicking
    assert!(!result.is_empty());
}
