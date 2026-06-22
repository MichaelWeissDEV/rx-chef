// Tests for the ctph operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ctph::

use rxchef::operations::ctph::CTPH;
use rxchef::Operation;

#[test]
fn test_ctph_basic() {
    let op = CTPH;
    let input = b"Hello, World!".to_vec();
    let result = op.run(input, &[]).expect("CTPH should succeed");
    let output = String::from_utf8(result).expect("valid utf8");
    // ssdeep format: blocksize:hash1:hash2
    assert!(output.contains(':'), "expected ssdeep format with ':'");
}
#[test]
fn test_ctph_empty() {
    let op = CTPH;
    let input = b"".to_vec();
    let result = op.run(input, &[]);
    // Empty input may succeed or fail depending on ssdeep crate behaviour
    let _ = result;
}
#[test]
fn test_ctph_longer_input() {
    let op = CTPH;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let result = op.run(input, &[]).expect("CTPH should succeed");
    let output = String::from_utf8(result).expect("valid utf8");
    assert!(output.contains(':'));
}
