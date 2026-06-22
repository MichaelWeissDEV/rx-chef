// Tests for the remove_diacritics operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations remove_diacritics::

use rxchef::operations::remove_diacritics::RemoveDiacritics;
use rxchef::Operation;

#[test]
fn test_remove_diacritics() {
    let op = RemoveDiacritics;
    let input = "Caf".as_bytes().to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Caf");
}
#[test]
fn test_remove_diacritics_complex() {
    let op = RemoveDiacritics;
    let input = "Mrt".as_bytes().to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Mrt");
}
#[test]
fn test_remove_strikethrough() {
    let op = RemoveDiacritics;
    // "strike" uses U+0336 (Combining Long Stroke Overlay)
    let input = "strike".as_bytes().to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "strike");
}
