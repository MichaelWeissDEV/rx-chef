// Tests for the lz4_decompress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lz4_decompress::

use lz4_flex::compress_prepend_size;
use rxchef::operations::lz4_decompress::LZ4Decompress;
use rxchef::Operation;

#[test]
fn test_lz4_decompress_basic() {
    let op = LZ4Decompress;
    let original = b"Hello, world! Hello, world! Hello, world!".to_vec();
    let compressed = compress_prepend_size(&original);
    let result = op.run(compressed, &[]).unwrap();
    assert_eq!(result, original);
}
