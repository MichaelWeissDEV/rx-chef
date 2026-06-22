// Tests for the from_case_insensitive_regex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_case_insensitive_regex::

use rxchef::operations::from_case_insensitive_regex::FromCaseInsensitiveRegex;
use rxchef::Operation;

#[test]
fn test_from_case_insensitive_regex_basic() {
    let op = FromCaseInsensitiveRegex;
    let input = b"[mM][oO][zZ]".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "MOZ");
}
#[test]
fn test_from_case_insensitive_regex_mixed() {
    let op = FromCaseInsensitiveRegex;
    // [aA] should collapse, [0-9] should remain
    let input = b"[aA][0-9]".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "A[0-9]");
}
#[test]
fn test_from_case_insensitive_regex_no_pairs() {
    let op = FromCaseInsensitiveRegex;
    let input = b"Mozilla/[0-9].[0-9] .*".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Mozilla/[0-9].[0-9] .*");
}
