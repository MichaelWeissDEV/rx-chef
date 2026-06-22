// Tests for the lzma_decompress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lzma_decompress::

use rxchef::operations::lzma_decompress::LZMADecompress;
use rxchef::Operation;
use std::io::Read;
use xz2::read::XzEncoder;

#[test]
fn test_lzma_decompress_basic() {
    let op = LZMADecompress;
    let original = b"Hello, world! Hello, world! Hello, world!".to_vec();
    let mut encoder = XzEncoder::new(&original[..], 6);
    let mut compressed = Vec::new();
    encoder.read_to_end(&mut compressed).unwrap();
    let result = op.run(compressed, &[]).unwrap();
    assert_eq!(result, original);
}
