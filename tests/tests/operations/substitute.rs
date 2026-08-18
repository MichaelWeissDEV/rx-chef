// Tests for the substitute operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations substitute::

use rxchef::operation::ArgValue;
use rxchef::operations::substitute::Substitute;
use rxchef::Operation;

#[test]
fn test_substitute_basic() {
    let op = Substitute;
    let input = b"ABC".to_vec();
    let args = [
        ArgValue::Str("ABC".to_string()),
        ArgValue::Str("XYZ".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "XYZ");
}
#[test]
fn test_substitute_range() {
    let op = Substitute;
    let input = b"123".to_vec();
    let args = [
        ArgValue::Str("0-9".to_string()),
        ArgValue::Str("9-0".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&result),
        "Warning: Plaintext and Ciphertext lengths differ\n\n123"
    );
}
#[test]
fn test_substitute_ignore_case() {
    let op = Substitute;
    let input = b"abcABC".to_vec();
    let args = [
        ArgValue::Str("a-z".to_string()),
        ArgValue::Str("A-Z".to_string()),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    // Since we ignore case, 'a' -> 'A', 'b' -> 'B', etc.
    // And case is preserved.
    assert_eq!(String::from_utf8_lossy(&result), "abcABC");
    let args2 = [
        ArgValue::Str("a".to_string()),
        ArgValue::Str("x".to_string()),
        ArgValue::Bool(true),
    ];
    let result2 = op.run(b"aA".to_vec(), &args2).unwrap();
    assert_eq!(String::from_utf8_lossy(&result2), "xX");
}
#[test]
fn test_substitute_escapes() {
    let op = Substitute;
    let input = b"\n\r".to_vec();
    let args = [
        ArgValue::Str("\\n\\r".to_string()),
        ArgValue::Str("XY".to_string()),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "XY");
}
