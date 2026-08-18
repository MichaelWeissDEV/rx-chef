// Tests for the wrap operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations wrap::

use rxchef::operation::ArgValue;
use rxchef::operations::wrap::WrapOp;
use rxchef::Operation;

#[test]
fn test_wrap_basic() {
    let op = WrapOp;
    let input = b"1234567890".to_vec();
    let args = [ArgValue::Num(3.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "123\n456\n789\n0");
}
#[test]
fn test_wrap_with_newlines() {
    let op = WrapOp;
    let input = b"12345\n67890".to_vec();
    let args = [ArgValue::Num(3.0)];
    let result = op.run(input, &args).unwrap();
    // JS behavior: . skips newlines
    assert_eq!(String::from_utf8_lossy(&result), "123\n45\n678\n90");
}
#[test]
fn test_wrap_empty() {
    let op = WrapOp;
    let input = b"".to_vec();
    let args = [ArgValue::Num(10.0)];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, b"");
}
