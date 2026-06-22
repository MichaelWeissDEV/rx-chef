// Tests for the standard_deviation operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations standard_deviation::

use rxchef::operation::ArgValue;
use rxchef::operations::standard_deviation::StandardDeviation;
use rxchef::Operation;

#[test]
fn test_std_dev_basic() {
    // JS doc example: "0x0a 8 .5" -> stdDev([10, 8, 0.5])  4.089281382128433
    // avg = 18.5/3  6.1667
    // devSum = (10-6.1667)^2 + (8-6.1667)^2 + (0.5-6.1667)^2
    //         14.694 + 3.361 + 32.178  50.233
    // stddev = sqrt(50.233/3)  4.0893
    let op = StandardDeviation;
    let result = op
        .run(b"0x0a 8 .5".to_vec(), &[ArgValue::Str("Space".to_string())])
        .unwrap();
    let s = String::from_utf8(result).unwrap();
    let v: f64 = s.parse().unwrap();
    assert!((v - 4.089281382128433).abs() < 1e-9);
}
#[test]
fn test_std_dev_identical() {
    // All same values -> stddev = 0
    let op = StandardDeviation;
    let result = op
        .run(
            b"5\n5\n5".to_vec(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "0");
}
#[test]
fn test_std_dev_empty() {
    let op = StandardDeviation;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "NaN");
}
#[test]
fn test_std_dev_two_values() {
    // [0, 2]: avg=1, devSum=(0-1)^2 + (2-1)^2 = 2, stddev = sqrt(2/2) = 1
    let op = StandardDeviation;
    let result = op
        .run(b"0\n2".to_vec(), &[ArgValue::Str("Line feed".to_string())])
        .unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "1");
}
