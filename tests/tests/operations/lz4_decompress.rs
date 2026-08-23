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
    // `lz4_flex` is an independent implementation of the LZ4 block format.
    // Its size-prepended stream is used as the external encoder oracle; the
    // rx-chef operation must interoperate with it byte-for-byte on decode.
    let compressed = compress_prepend_size(&original);
    let result = op.run(compressed, &[]).unwrap();
    assert_eq!(result, original);
}
