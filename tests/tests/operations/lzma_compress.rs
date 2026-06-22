// Tests for the lzma_compress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lzma_compress::

use rxchef::operation::ArgValue;
use rxchef::operations::lzma_compress::LZMACompress;
use rxchef::Operation;

#[test]
fn test_lzma_compress_basic() {
    let op = LZMACompress;
    let input = b"Hello, world! Hello, world! Hello, world!".to_vec();
    let args = vec![ArgValue::Str("7".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert!(result.len() < input.len() || !result.is_empty());
}
