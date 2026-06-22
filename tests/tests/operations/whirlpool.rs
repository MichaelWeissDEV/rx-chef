// Tests for the whirlpool operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations whirlpool::

use rxchef::operation::ArgValue;
use rxchef::operations::whirlpool::WHIRLPOOL;
use rxchef::Operation;

#[test]
fn test_whirlpool_basic() {
    let operation = WHIRLPOOL;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(
            input,
            &[ArgValue::Str("Whirlpool".to_string()), ArgValue::Num(10.0)],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // Whirlpool hash of "Hello, World!"
    assert_eq!(output.len(), 128); // 512 bits = 128 hex chars
}
#[test]
fn test_whirlpool_empty() {
    let operation = WHIRLPOOL;
    let input = b"".to_vec();
    let result = operation
        .run(
            input,
            &[ArgValue::Str("Whirlpool".to_string()), ArgValue::Num(10.0)],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // Whirlpool hash of empty string
    assert_eq!(output.len(), 128);
}
#[test]
fn test_whirlpool_invalid_variant() {
    let operation = WHIRLPOOL;
    let input = b"test".to_vec();
    let args = [ArgValue::Str("Invalid".to_string()), ArgValue::Num(10.0)];
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_whirlpool_invalid_rounds() {
    let operation = WHIRLPOOL;
    let input = b"test".to_vec();
    let args = [ArgValue::Str("Whirlpool".to_string()), ArgValue::Num(15.0)]; // Invalid - > 10
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
