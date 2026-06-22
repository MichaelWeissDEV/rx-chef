// Tests for the unzip operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations unzip::

use rxchef::operations::unzip::Unzip;
use rxchef::Operation;
use std::io::Cursor;
use std::io::Write;
use zip::write::FileOptions;
use zip::write::ZipWriter;

#[test]
fn test_unzip_empty() {
    let op = Unzip;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, b"No files found in zip.");
}
#[test]
fn test_unzip_single_file() {
    let op = Unzip;
    let mut buf = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buf));
        let options: FileOptions<'_, ()> = FileOptions::default();
        zip.start_file("test.txt", options).unwrap();
        zip.write_all(b"hello").unwrap();
        zip.finish().unwrap();
    }
    let result = op.run(buf, &[]).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("File: test.txt"));
    assert!(result_str.contains("hello"));
}
#[test]
fn test_unzip_multiple_files() {
    let op = Unzip;
    let mut buf = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut buf));
        let options: FileOptions<'_, ()> = FileOptions::default();
        zip.start_file("a.txt", options).unwrap();
        zip.write_all(b"aaaaa").unwrap();
        let options: FileOptions<'_, ()> = FileOptions::default();
        zip.start_file("b.txt", options).unwrap();
        zip.write_all(b"bbbbb").unwrap();
        zip.finish().unwrap();
    }
    let result = op.run(buf, &[]).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("File: a.txt"));
    assert!(result_str.contains("aaaaa"));
    assert!(result_str.contains("File: b.txt"));
    assert!(result_str.contains("bbbbb"));
}
