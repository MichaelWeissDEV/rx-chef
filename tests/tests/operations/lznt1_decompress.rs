// Tests for the lznt1_decompress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lznt1_decompress::

use rxchef::operations::lznt1_decompress::LZNT1Decompress;
use rxchef::Operation;

#[test]
fn test_lznt1_decompress() {
    let op = LZNT1Decompress;
    // A valid uncompressed LZNT1 chunk: signature 0b011 and size-minus-one.
    let text = b"This is a test. This is a test.";
    let header = 0x3000 | (text.len() as u16 - 1);
    let mut input = header.to_le_bytes().to_vec();
    input.extend_from_slice(text);
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "This is a test. This is a test."
    );
}

#[test]
fn test_lznt1_rejects_token_before_any_literal() {
    let op = LZNT1Decompress;
    // Compressed chunk with flag bit 0 set and a back-reference token first.
    let result = op.run(vec![0x02, 0xb0, 0x01, 0x00, 0x00], &[]);
    assert!(result.is_err());
}
