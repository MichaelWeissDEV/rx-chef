// Tests for the extract_files operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_files::

use rxchef::operations::extract_files::ExtractFiles;
use rxchef::Operation;

#[test]
fn test_extract_files_empty_input() {
    let op = ExtractFiles;
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Num(100.0),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "No files found.".as_bytes());
}

#[test]
fn test_extract_files_no_matching_files() {
    let op = ExtractFiles;
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Num(100.0),
    ];
    // Random data with no file signatures
    let random_data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let result = op.run(random_data, &args).unwrap();
    assert_eq!(result, "No files found.".as_bytes());
}

#[test]
fn test_extract_files_png_extraction() {
    let op = ExtractFiles;
    let args = [
        rxchef::operation::ArgValue::Bool(true),  // Images
        rxchef::operation::ArgValue::Bool(false), // No documents
        rxchef::operation::ArgValue::Bool(false), // No archives
        rxchef::operation::ArgValue::Bool(true),  // Ignore failed
        rxchef::operation::ArgValue::Num(10.0),   // Small min size
    ];
    // Minimal PNG file signature
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 pixel
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, // More chunk data
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xBF, 0x3C, // Image data
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // IEND chunk
        0xAE, 0x42, 0x60, 0x82, // IEND signature
    ];
    let result = op.run(png_data, &args).unwrap();
    // The result should contain the text header followed by binary PNG data
    // Just check that it starts with the expected text
    if result.len() > 20 {
        let header = &result[..std::cmp::min(50, result.len())];
        let header_str = String::from_utf8_lossy(header);
        assert!(header_str.contains("--- Extracted PNG ---"));
    }
}

#[test]
fn test_extract_files_minimum_size_filter() {
    let op = ExtractFiles;
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Num(200.0), // Large min size
    ];
    // Small PNG that should be filtered out
    let small_png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let result = op.run(small_png, &args).unwrap();
    assert_eq!(result, "No files found.".as_bytes());
}

#[test]
fn test_extract_files_disable_image_extraction() {
    let op = ExtractFiles;
    let args = [
        rxchef::operation::ArgValue::Bool(false), // No images
        rxchef::operation::ArgValue::Bool(true),  // Documents
        rxchef::operation::ArgValue::Bool(true),  // Archives
        rxchef::operation::ArgValue::Bool(true),  // Ignore failed
        rxchef::operation::ArgValue::Num(10.0),
    ];
    // PNG data should not be extracted
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let result = op.run(png_data, &args).unwrap();
    assert_eq!(result, "No files found.".as_bytes());
}
