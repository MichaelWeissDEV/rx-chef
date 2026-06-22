// Tests for the raw_inflate operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations raw_inflate::

use flate2::write::DeflateEncoder;
use flate2::Compression;
use rxchef::operations::raw_inflate::RawInflate;
use rxchef::Operation;
use std::io::Write;

fn raw_compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("write");
    encoder.finish().expect("finish")
}
#[test]
fn test_raw_inflate_basic() {
    let original = b"Hello raw inflate!";
    let compressed = raw_compress(original);
    let op = RawInflate;
    let result = op.run(compressed, &[]).expect("should succeed");
    assert_eq!(result, original);
}
#[test]
fn test_raw_inflate_invalid() {
    let op = RawInflate;
    let result = op.run(b"not deflated".to_vec(), &[]);
    assert!(result.is_err());
}
