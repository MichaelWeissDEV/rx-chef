// Tests for the scan_for_embedded_files operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations scan_for_embedded_files::

use rxchef::operation::ArgValue;
use rxchef::operations::scan_for_embedded_files::ScanForEmbeddedFiles;
use rxchef::Operation;

#[test]
fn test_scan_png() {
    let op = ScanForEmbeddedFiles;
    let mut input = vec![0; 10];
    input.extend_from_slice(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
    input.extend_from_slice(&[0; 10]);
    let args = [
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    assert!(result.contains("Offset 10"));
    assert!(result.contains("File type:   PNG"));
}
#[test]
fn test_scan_pdf() {
    let op = ScanForEmbeddedFiles;
    let input = b"some data %PDF-1.4 more data".to_vec();
    let args = [
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    assert!(result.contains("File type:   PDF"));
}
#[test]
fn test_scan_none() {
    let op = ScanForEmbeddedFiles;
    let input = vec![0, 1, 2, 3, 4, 5];
    let args = [
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    assert!(result.contains("No embedded files were found."));
}
