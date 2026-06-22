// Tests for the zlib_inflate operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations zlib_inflate::

use flate2::write::ZlibEncoder;
use flate2::Compression;
use rxchef::operations::zlib_inflate::ZlibInflate;
use rxchef::Operation;
use std::io::Write;

fn zlib_compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("write");
    encoder.finish().expect("finish")
}
#[test]
fn test_zlib_inflate_basic() {
    let original = b"Hello zlib!";
    let compressed = zlib_compress(original);
    let op = ZlibInflate;
    let result = op.run(compressed, &[]).expect("should succeed");
    assert_eq!(result, original);
}
#[test]
fn test_zlib_inflate_invalid() {
    let op = ZlibInflate;
    let result = op.run(b"not zlib".to_vec(), &[]);
    assert!(result.is_err());
}
