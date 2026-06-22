// Tests for the gunzip operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations gunzip::

use flate2::write::GzEncoder;
use flate2::Compression;
use rxchef::operations::gunzip::Gunzip;
use rxchef::Operation;
use std::io::Write;

fn gzip_compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("write");
    encoder.finish().expect("finish")
}
#[test]
fn test_gunzip_basic() {
    let original = b"Hello, World!";
    let compressed = gzip_compress(original);
    let op = Gunzip;
    let result = op.run(compressed, &[]).expect("should succeed");
    assert_eq!(result, original);
}
#[test]
fn test_gunzip_empty() {
    let compressed = gzip_compress(b"");
    let op = Gunzip;
    let result = op.run(compressed, &[]).expect("should succeed");
    assert_eq!(result, b"");
}
#[test]
fn test_gunzip_invalid() {
    let op = Gunzip;
    let result = op.run(b"not gzip data".to_vec(), &[]);
    assert!(result.is_err());
}
