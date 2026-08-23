// Tests for the lz4_compress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lz4_compress::

use rxchef::operations::lz4_compress::LZ4Compress;
use rxchef::Operation;

#[test]
fn test_lz4_compress_basic() {
    let op = LZ4Compress;
    let input = b"Hello, world! Hello, world! Hello, world!".to_vec();
    let result = op.run(input.clone(), &[]).unwrap();
    assert!(result.len() < input.len());
}

#[test]
fn test_lz4_compress_empty_input_boundary() {
    // The block wrapper prepends the uncompressed length as little-endian u32.
    assert_eq!(LZ4Compress.run(Vec::new(), &[]).unwrap(), vec![0, 0, 0, 0]);
}
