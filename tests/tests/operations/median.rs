// Tests for the median operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations median::

use rxchef::operation::ArgValue;
use rxchef::operations::median::Median;
use rxchef::Operation;

#[test]
fn test_median_even_count() {
    // JS doc example: "0x0a 8 1 .5" -> sorted [0.5, 1, 8, 10] -> (1+8)/2 = 4.5
    let op = Median;
    let result = op
        .run(
            b"0x0a 8 1 .5".to_vec(),
            &[ArgValue::Str("Space".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "4.5");
}
#[test]
fn test_median_odd_count() {
    // sorted [1, 2, 3] -> median = 2
    let op = Median;
    let result = op
        .run(
            b"3\n1\n2".to_vec(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "2");
}
#[test]
fn test_median_single() {
    let op = Median;
    let result = op
        .run(b"7".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "7");
}
#[test]
fn test_median_empty() {
    let op = Median;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
