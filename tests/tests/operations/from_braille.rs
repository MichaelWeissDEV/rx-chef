// Tests for the from_braille operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_braille::

use rxchef::operations::from_braille::FromBraille;
use rxchef::Operation;

#[test]
fn test_from_braille_space() {
    let op = FromBraille;
    // U+2800 = Braille blank = space
    let input = "\u{2800}".as_bytes().to_vec();
    let args = [];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), " ");
}
#[test]
fn test_from_braille_a() {
    let op = FromBraille;
    // U+2801 = Braille A
    let input = "\u{2801}".as_bytes().to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "A");
}
#[test]
fn test_from_braille_passthrough_non_braille() {
    let op = FromBraille;
    let input = b"hello".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello");
}
