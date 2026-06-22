// Tests for the offset_checker operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations offset_checker::

use rxchef::operation::ArgValue;
use rxchef::operations::offset_checker::OffsetChecker;
use rxchef::Operation;

#[test]
fn test_offset_checker_basic() {
    let op = OffsetChecker;
    // Two identical samples -> all chars should match
    let input = b"abc\n\nabc".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains('['));
}
#[test]
fn test_offset_checker_no_match() {
    let op = OffsetChecker;
    let input = b"abc\n\ndef".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    // No matching characters
    assert!(!out.contains('['));
}
#[test]
fn test_offset_checker_too_few_samples() {
    let op = OffsetChecker;
    let input = b"onlyone".to_vec();
    assert!(op.run(input, &[]).is_err());
}
#[test]
fn test_offset_checker_custom_delimiter() {
    let op = OffsetChecker;
    let input = b"abc|abc".to_vec();
    let result = op.run(input, &[ArgValue::Str("|".to_string())]);
    assert!(result.is_ok());
}
