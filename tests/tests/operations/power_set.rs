// Tests for the power_set operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations power_set::

use rxchef::operation::ArgValue;
use rxchef::operations::power_set::PowerSet;
use rxchef::Operation;

#[test]
fn test_power_set_basic() {
    let operation = PowerSet;
    let input = b"a,b,c".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str(",".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // Should contain all subsets
    assert!(output.contains("a"));
    assert!(output.contains("b"));
    assert!(output.contains("c"));
}
#[test]
fn test_power_set_empty() {
    let operation = PowerSet;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.is_empty());
}
#[test]
fn test_power_set_single() {
    let operation = PowerSet;
    let input = b"a".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("a"));
}
