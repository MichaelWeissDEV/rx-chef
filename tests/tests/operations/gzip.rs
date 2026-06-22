// Tests for the gzip operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations gzip::

use flate2::read::GzDecoder;
use rxchef::operations::gzip::Gzip;
use rxchef::Operation;
use std::io::Read;

fn gzip_decompress(data: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).expect("decompress");
    out
}
#[test]
fn test_gzip_roundtrip() {
    let op = Gzip;
    let original = b"Hello, World! This is a test of gzip compression.";
    let compressed = op.run(original.to_vec(), &[]).expect("compress");
    let decompressed = gzip_decompress(&compressed);
    assert_eq!(decompressed, original);
}
#[test]
fn test_gzip_empty() {
    let op = Gzip;
    let compressed = op.run(b"".to_vec(), &[]).expect("compress empty");
    let decompressed = gzip_decompress(&compressed);
    assert_eq!(decompressed, b"");
}
#[test]
fn test_gzip_magic_bytes() {
    let op = Gzip;
    let compressed = op.run(b"test".to_vec(), &[]).expect("compress");
    // Gzip magic bytes: 0x1f 0x8b
    assert_eq!(compressed[0], 0x1f);
    assert_eq!(compressed[1], 0x8b);
}
