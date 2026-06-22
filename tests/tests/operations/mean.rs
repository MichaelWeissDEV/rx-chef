// Tests for the mean operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations mean::

use rxchef::operation::ArgValue;
use rxchef::operations::mean::Mean;
use rxchef::Operation;

#[test]
fn test_mean_basic() {
    // JS doc example: "0x0a 8 .5 .5" -> mean of [10, 8, 0.5, 0.5] = 19/4 = 4.75
    let op = Mean;
    let result = op
        .run(
            b"0x0a 8 .5 .5".to_vec(),
            &[ArgValue::Str("Space".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "4.75");
}
#[test]
fn test_mean_simple() {
    // mean of [1, 2, 3] = 2
    let op = Mean;
    let result = op
        .run(
            b"1\n2\n3".to_vec(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "2");
}
#[test]
fn test_mean_empty() {
    let op = Mean;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
#[test]
fn test_mean_single() {
    let op = Mean;
    let result = op
        .run(b"42".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "42");
}
