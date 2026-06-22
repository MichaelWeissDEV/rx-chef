// Tests for the count_occurrences operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations count_occurrences::

use rxchef::operation::ArgValue;
use rxchef::operations::count_occurrences::CountOccurrences;
use rxchef::Operation;

#[test]
fn test_count_occurrences_basic() {
    let operation = CountOccurrences;
    let input = b"hello world hello".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("hello".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "2");
}
#[test]
fn test_count_occurrences_empty() {
    let operation = CountOccurrences;
    let input = b"hello world".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0");
}
#[test]
fn test_count_occurrences_not_found() {
    let operation = CountOccurrences;
    let input = b"hello world".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("xyz".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0");
}
#[test]
fn test_count_occurrences_overlapping() {
    let operation = CountOccurrences;
    let input = b"aaaa".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("aa".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "2");
}
