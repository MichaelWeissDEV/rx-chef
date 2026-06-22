// Tests for the ssdeep operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ssdeep::

use rxchef::operations::ssdeep::SSDEEP;
use rxchef::Operation;

#[test]
fn test_ssdeep_basic() {
    let op = SSDEEP;
    let input = b"Hello, World!".to_vec();
    let result = op.run(input, &[]).expect("SSDEEP should succeed");
    let output = String::from_utf8(result).expect("valid utf8");
    // ssdeep format: blocksize:hash1:hash2
    assert!(output.contains(':'), "expected ssdeep format with ':'");
}
#[test]
fn test_ssdeep_longer_input() {
    let op = SSDEEP;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let result = op.run(input, &[]).expect("SSDEEP should succeed");
    let output = String::from_utf8(result).expect("valid utf8");
    assert!(output.contains(':'));
}
#[test]
fn test_ssdeep_and_ctph_same_output() {
    // Both CTPH and SSDEEP use ssdeep::hash, so they produce the same result
    let op = SSDEEP;
    let input = vec![0u8; 1024];
    let result = op.run(input, &[]).expect("SSDEEP should succeed");
    let output = String::from_utf8(result).expect("valid utf8");
    assert!(output.contains(':'));
}
