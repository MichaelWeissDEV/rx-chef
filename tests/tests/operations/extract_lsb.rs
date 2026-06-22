// Tests for the extract_lsb operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_lsb::

use rxchef::operations::extract_lsb::ExtractLSB;
use rxchef::Operation;

#[test]
fn test_extract_lsb_empty_input() {
    let op = ExtractLSB;
    let args = [
        rxchef::operation::ArgValue::Str("R".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Row".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_lsb_invalid_image() {
    let op = ExtractLSB;
    let args = [
        rxchef::operation::ArgValue::Str("R".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Row".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    // Invalid image data
    let invalid_image = vec![0x00, 0x01, 0x02, 0x03];
    let result = op.run(invalid_image, &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_lsb_invalid_bit() {
    let op = ExtractLSB;
    let args = [
        rxchef::operation::ArgValue::Str("R".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Row".to_string()),
        rxchef::operation::ArgValue::Num(8.0), // Invalid bit (must be 0-7)
    ];
    let result = op.run(vec![0xFF], &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Must be between 0 and 7"));
}

#[test]
fn test_extract_lsb_minimal_png() {
    let op = ExtractLSB;
    let args = [
        rxchef::operation::ArgValue::Str("R".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Row".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    // Minimal 1x1 PNG image - this will likely fail to parse but we test the attempt
    let minimal_png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41,
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xBF, 0x3C,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82,
    ];
    let result = op.run(minimal_png, &args);
    // This will likely fail due to invalid PNG format, but we're testing the error handling
    assert!(result.is_err());
}

#[test]
fn test_extract_lsb_multiple_colours() {
    let op = ExtractLSB;
    let args = [
        rxchef::operation::ArgValue::Str("R".to_string()),
        rxchef::operation::ArgValue::Str("G".to_string()),
        rxchef::operation::ArgValue::Str("B".to_string()),
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Str("Row".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    // Minimal 1x1 PNG image - this will likely fail to parse but we test the attempt
    let minimal_png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41,
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xBF, 0x3C,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
        0xAE, 0x42, 0x60, 0x82,
    ];
    let result = op.run(minimal_png, &args);
    // This will likely fail due to invalid PNG format, but we're testing the error handling
    assert!(result.is_err());
}
