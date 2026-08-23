// Tests for the parse_colour_code operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_colour_code::

use rxchef::operations::parse_colour_code::ParseColourCode;
use rxchef::Operation;

#[test]
fn test_parse_colour_code_rejects_unsupported_format() {
    assert!(ParseColourCode.run(b"red".to_vec(), &[]).is_err());
}

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

#[test]
fn test_black_channel_minimum_boundary() {
    let output = ParseColourCode.run(b"#000000".to_vec(), &[]).unwrap();
    let text = String::from_utf8(output).unwrap();
    let channels: Vec<_> = text
        .lines()
        .filter(|line| {
            line.starts_with("Hex:") || line.starts_with("RGB:") || line.starts_with("HSL:")
        })
        .collect();
    assert_eq!(
        channels,
        vec![
            "Hex:  #000000",
            "RGB:  rgb(0, 0, 0)",
            "HSL:  hsl(0, 0%, 0%)"
        ]
    );
}
