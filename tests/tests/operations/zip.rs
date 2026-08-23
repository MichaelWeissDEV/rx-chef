// Tests for the zip operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations zip::

use rxchef::operation::ArgValue;
use rxchef::operations::zip::ZipOp;
use rxchef::Operation;
use std::io::{Cursor, Read};
use zip::CompressionMethod;
use zip::ZipArchive;

#[test]
fn test_zip_deflate() {
    let op = ZipOp;
    let input = b"Hello Zip World!".to_vec();
    let args = [
        ArgValue::Str("test.txt".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("Deflate".to_string()),
        ArgValue::Str("Unix".to_string()),
        ArgValue::Str("Dynamic".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(result)).unwrap();
    assert_eq!(archive.len(), 1);
    let mut file = archive.by_name("test.txt").unwrap();
    let mut decompressed = Vec::new();
    file.read_to_end(&mut decompressed).unwrap();
    assert_eq!(decompressed, input);
}
#[test]
fn test_zip_store() {
    let op = ZipOp;
    let input = b"Hello Zip World!".to_vec();
    let args = [
        ArgValue::Str("test.txt".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("None (Store)".to_string()),
        ArgValue::Str("Unix".to_string()),
        ArgValue::Str("Dynamic".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(result)).unwrap();
    let file = archive.by_name("test.txt").unwrap();
    assert_eq!(file.compression(), CompressionMethod::Stored);
}
#[test]
fn test_zip_password() {
    let op = ZipOp;
    let input = b"Secret data".to_vec();
    let args = [
        ArgValue::Str("secret.txt".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("password123".to_string()),
        ArgValue::Str("Deflate".to_string()),
        ArgValue::Str("Unix".to_string()),
        ArgValue::Str("Dynamic".to_string()),
    ];
    let result = op.run(input.clone(), &args).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(result)).unwrap();
    let mut file = archive
        .by_name_decrypt("secret.txt", b"password123")
        .unwrap();
    let mut decompressed = Vec::new();
    file.read_to_end(&mut decompressed).unwrap();
    assert_eq!(decompressed, input);
}

#[test]
fn test_zip_empty_file_boundary() {
    let args = [
        ArgValue::Str("empty.bin".into()),
        ArgValue::Str(String::new()),
        ArgValue::Str(String::new()),
        ArgValue::Str("None (Store)".into()),
        ArgValue::Str("Unix".into()),
        ArgValue::Str("Dynamic".into()),
    ];
    let output = ZipOp.run(Vec::new(), &args).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(output)).unwrap();
    let mut contents = Vec::new();
    archive.by_name("empty.bin").unwrap().read_to_end(&mut contents).unwrap();
    assert_eq!(contents, Vec::<u8>::new());
}

#[test]
fn test_zip_rejects_unknown_compression_method() {
    let args = [
        ArgValue::Str("file.bin".into()),
        ArgValue::Str(String::new()),
        ArgValue::Str(String::new()),
        ArgValue::Str("Brotli".into()),
    ];
    assert!(ZipOp.run(b"data".to_vec(), &args).is_err());
}
