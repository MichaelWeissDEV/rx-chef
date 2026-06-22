// Tests for the rot47 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rot47::

use rxchef::operations::rot47::ROT47;
use rxchef::Operation;

fn rot47(s: &str) -> String {
    let op = ROT47;
    String::from_utf8(op.run(s.as_bytes().to_vec(), &[]).unwrap()).unwrap()
}
#[test]
fn test_basic() {
    assert_eq!(rot47("Hello World"), "w6==@ (@C=5");
}
#[test]
fn test_double_rot47() {
    let s = "The Quick Brown Fox";
    assert_eq!(rot47(&rot47(s)), s);
}
#[test]
fn test_non_printable_unchanged() {
    assert_eq!(rot47(" "), " ");
    assert_eq!(rot47("\n"), "\n");
}
