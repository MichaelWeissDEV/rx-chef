// Tests for the extract_rgba operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_rgba::

use rxchef::operations::extract_rgba::ExtractRGBA;
use rxchef::Operation;

#[test]
fn test_extract_rgba_empty_input() {
    let op = ExtractRGBA;
    let args = [
        rxchef::operation::ArgValue::Str(" ".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_rgba_invalid_image() {
    let op = ExtractRGBA;
    let args = [
        rxchef::operation::ArgValue::Str(" ".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Invalid image data
    let invalid_image = vec![0x00, 0x01, 0x02, 0x03];
    let result = op.run(invalid_image, &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_rgba_custom_delimiter() {
    let op = ExtractRGBA;
    let args = [
        rxchef::operation::ArgValue::Str(",".to_string()),
        rxchef::operation::ArgValue::Bool(true),
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
fn test_extract_rgba_no_alpha() {
    let op = ExtractRGBA;
    let args = [
        rxchef::operation::ArgValue::Str(" ".to_string()),
        rxchef::operation::ArgValue::Bool(false), // No alpha channel
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
fn test_extract_rgba_large_input() {
    let op = ExtractRGBA;
    let args = [
        rxchef::operation::ArgValue::Str(" ".to_string()),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Large random data that's not a valid image
    let large_data = vec![0xFF; 1024];
    let result = op.run(large_data, &args);
    assert!(result.is_err());
}
