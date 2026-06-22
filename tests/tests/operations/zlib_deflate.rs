// Tests for the zlib_deflate operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations zlib_deflate::

use flate2::read::ZlibDecoder;
use rxchef::operations::zlib_deflate::ZlibDeflate;
use rxchef::Operation;
use std::io::Read;

fn zlib_decompress(data: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(data);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out).expect("decompress");
    out
}
#[test]
fn test_zlib_deflate_roundtrip() {
    let op = ZlibDeflate;
    let original = b"Hello zlib compression!";
    let compressed = op.run(original.to_vec(), &[]).expect("compress");
    let decompressed = zlib_decompress(&compressed);
    assert_eq!(decompressed, original);
}
#[test]
fn test_zlib_deflate_empty() {
    let op = ZlibDeflate;
    let compressed = op.run(b"".to_vec(), &[]).expect("compress empty");
    let decompressed = zlib_decompress(&compressed);
    assert_eq!(decompressed, b"");
}
