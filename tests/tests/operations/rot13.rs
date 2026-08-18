// Tests for the rot13 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rot13::

use rxchef::operation::ArgValue;
use rxchef::operations::rot13::ROT13;
use rxchef::Operation;

fn rot13(s: &str) -> String {
    let op = ROT13;
    let args = [
        ArgValue::Str("true".into()),
        ArgValue::Str("true".into()),
        ArgValue::Str("false".into()),
        ArgValue::Str("13".into()),
    ];
    String::from_utf8(op.run(s.as_bytes().to_vec(), &args).unwrap()).unwrap()
}
#[test]
fn test_basic() {
    assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
}
#[test]
fn test_double_rot13() {
    assert_eq!(rot13(&rot13("The Quick Brown Fox")), "The Quick Brown Fox");
}
#[test]
fn test_only_letters() {
    assert_eq!(rot13("ABC xyz 123"), "NOP klm 123");
}
