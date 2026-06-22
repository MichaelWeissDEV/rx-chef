// Tests for the parse_colour_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_colour_code::

use rxchef::operations::parse_colour_code::ParseColourCode;
use rxchef::Operation;

#[test]
fn test_hex() {
    let op = ParseColourCode;
    let input = b"#d9edf7".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Hex:  #d9edf7"));
    assert!(result_str.contains("RGB:  rgb(217, 237, 247)"));
}
#[test]
fn test_rgba() {
    let op = ParseColourCode;
    let input = b"rgba(217,237,247,1)".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Hex:  #d9edf7"));
    assert!(result_str.contains("RGBA: rgba(217, 237, 247, 1)"));
}
#[test]
fn test_hsla() {
    let op = ParseColourCode;
    let input = b"hsla(200,65%,91%,1)".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Hex:  #d9edf7"));
    assert!(result_str.contains("HSL:  hsl(200, 65%, 91%)"));
}
#[test]
fn test_cmyk() {
    let op = ParseColourCode;
    let input = b"cmyk(0.12, 0.04, 0.00, 0.03)".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Hex:  #daedf7"));
}
