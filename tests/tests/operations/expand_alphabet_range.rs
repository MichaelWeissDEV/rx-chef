// Tests for the expand_alphabet_range operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations expand_alphabet_range::

use rxchef::operation::ArgValue;
use rxchef::operations::expand_alphabet_range::ExpandAlphabetRange;
use rxchef::Operation;

#[test]
fn test_expand_alphabet_range() {
    let op = ExpandAlphabetRange;
    let args = [ArgValue::Str("".to_string())];
    let input = b"a-z".to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "abcdefghijklmnopqrstuvwxyz"
    );
}
#[test]
fn test_expand_alphabet_range_with_delimiter() {
    let op = ExpandAlphabetRange;
    let args = [ArgValue::Str(",".to_string())];
    let input = b"0-3".to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "0,1,2,3");
}
#[test]
fn test_expand_alphabet_range_escaped() {
    let op = ExpandAlphabetRange;
    let args = [ArgValue::Str("".to_string())];
    let input = b"a\\-c".to_vec();
    let output = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(output).unwrap(), "a\\]^_`abc");
}
