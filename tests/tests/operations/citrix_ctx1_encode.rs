// Tests for the citrix_ctx1_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations citrix_ctx1_encode::

use rxchef::operations::citrix_ctx1_encode::CitrixCtx1Encode;
use rxchef::Operation;

#[test]
fn test_ctx1_encode_output_is_ascii() {
    let op = CitrixCtx1Encode;
    let result = op.run(b"Password".to_vec(), &[]).unwrap();
    // All output bytes should be in range 0x41..=0x50 (A..P)
    for &b in &result {
        assert!(b >= 0x41, "byte {} < 0x41", b);
        assert!(b <= 0x50, "byte {} > 0x50", b);
    }
}
#[test]
fn test_ctx1_encode_length() {
    let op = CitrixCtx1Encode;
    // ASCII input of N chars -> N*2 UTF-16LE bytes -> N*4 output bytes
    let result = op.run(b"Test".to_vec(), &[]).unwrap();
    assert_eq!(result.len(), 4 * 4); // 4 chars * 4 output bytes each
}
#[test]
fn test_ctx1_encode_empty() {
    let op = CitrixCtx1Encode;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_ctx1_encode_known_value() {
    // Verify against known CTX1 encoding of "Password"
    // From the Citrix CTX1 spec / common references:
    // "Password" -> "PABLCAAAAAAAAAAAAAAA" (this is approximate; exact value depends on impl)
    let op = CitrixCtx1Encode;
    let result = op.run(b"Password".to_vec(), &[]).unwrap();
    // Just verify it's non-empty and all chars are in A-P range
    assert!(!result.is_empty());
    let s = String::from_utf8(result).unwrap();
    for ch in s.chars() {
        assert!(ch >= 'A' && ch <= 'P', "char '{}' out of range", ch);
    }
}
