// Tests for the to_case_insensitive_regex operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_case_insensitive_regex::

use rxchef::operations::to_case_insensitive_regex::ToCaseInsensitiveRegex;
use rxchef::Operation;

#[test]
fn test_basic_string() {
    let op = ToCaseInsensitiveRegex;
    let input = b"Mozilla".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "[mM][oO][zZ][iI][lL][lL][aA]"
    );
}
#[test]
fn test_regex_range() {
    let op = ToCaseInsensitiveRegex;
    let input = b"[a-z]".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "[A-Za-z]");
}
#[test]
fn test_mixed_regex() {
    let op = ToCaseInsensitiveRegex;
    let input = b"abc[0-9]def".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "[aA][bB][cC][0-9][dD][eE][fF]"
    );
}
