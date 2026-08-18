// Tests for the to_braille operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_braille::

use rxchef::operations::from_braille::FromBraille;
use rxchef::operations::to_braille::ToBraille;
use rxchef::Operation;

#[test]
fn test_to_braille_space() {
    let op = ToBraille;
    let input = b" ".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "\u{2800}");
}
#[test]
fn test_to_braille_a() {
    let op = ToBraille;
    let input = b"a".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "\u{2801}");
}
#[test]
fn test_roundtrip() {
    let to = ToBraille;
    let from = FromBraille;
    let input = b"HELLO".to_vec();
    let braille = to.run(input.clone(), &[]).unwrap();
    let back = from.run(braille, &[]).unwrap();
    assert_eq!(String::from_utf8(back).unwrap(), "HELLO");
}

#[test]
fn test_to_braille_invalid_utf8() {
    let op = FromBraille;
    // Operations that take String input should fail on invalid UTF-8
    let result = rxchef::Operation::run(&op, vec![0xFF, 0xFE], &[]);
    assert!(result.is_err());
}
