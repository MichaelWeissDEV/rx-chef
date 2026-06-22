// Tests for the raw_deflate operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations raw_deflate::

use flate2::read::DeflateDecoder;
use rxchef::operations::raw_deflate::RawDeflate;
use rxchef::Operation;
use std::io::Read;

fn raw_inflate(data: &[u8]) -> Vec<u8> {
    let mut decoder = DeflateDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).expect("decompress");
    out
}
#[test]
fn test_raw_deflate_roundtrip() {
    let op = RawDeflate;
    let original = b"Hello raw deflate!";
    let compressed = op.run(original.to_vec(), &[]).expect("compress");
    let decompressed = raw_inflate(&compressed);
    assert_eq!(decompressed, original);
}
#[test]
fn test_raw_deflate_empty() {
    let op = RawDeflate;
    let compressed = op.run(b"".to_vec(), &[]).expect("compress empty");
    let decompressed = raw_inflate(&compressed);
    assert_eq!(decompressed, b"");
}
