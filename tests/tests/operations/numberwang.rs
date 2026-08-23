// Tests for the numberwang operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations numberwang::

use rxchef::operations::numberwang::Numberwang;
use rxchef::Operation;

#[test]
fn test_numberwang_empty() {
    let op = Numberwang;
    let result = op.run(b"".to_vec(), &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Let's play Wangernumb!"));
}
#[test]
fn test_numberwang_match() {
    let op = Numberwang;
    let result = op.run(b"42".to_vec(), &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // The fact is intentionally random, while the answer paragraph is the
    // deterministic Numberwang reference behaviour.
    assert_eq!(
        result_str.split("\n\n").next(),
        Some("42! That's Numberwang!")
    );
}
#[test]
fn test_alphanumericwang_match() {
    let op = Numberwang;
    let result = op.run(b"42a".to_vec(), &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("42a! That's AlphaNumericWang!"));
}
