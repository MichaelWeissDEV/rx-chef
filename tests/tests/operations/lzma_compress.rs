// Tests for the lzma_compress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lzma_compress::

use rxchef::operation::ArgValue;
use rxchef::operations::lzma_compress::LZMACompress;
use rxchef::Operation;
use std::io::Read;

#[test]
fn test_lzma_compress_basic() {
    let op = LZMACompress;
    let input = b"Hello, world! Hello, world! Hello, world!".to_vec();
    let args = vec![ArgValue::Str("7".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    assert_eq!(&result[..6], b"\xfd7zXZ\0");
    let mut decoded = Vec::new();
    xz2::read::XzDecoder::new(&result[..])
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_lzma_compress_empty_input_boundary() {
    let output = LZMACompress
        .run(Vec::new(), &[ArgValue::Str("1".into())])
        .unwrap();
    let mut decoded = Vec::new();
    xz2::read::XzDecoder::new(&output[..])
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, Vec::<u8>::new());
}

#[test]
fn test_lzma_compress_rejects_out_of_range_mode() {
    assert!(LZMACompress
        .run(b"data".to_vec(), &[ArgValue::Str("10".into())])
        .is_err());
}
