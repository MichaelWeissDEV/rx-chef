// Tests for the lz_string_decompress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations lz_string_decompress::

use rxchef::operation::ArgValue;
use rxchef::operations::lz_string_decompress::LZStringDecompress;
use rxchef::Operation;

#[test]
#[ignore]
fn test_lzstring_decompress_standard() {
    let op = LZStringDecompress;
    let input = "㘶㳳㜶㜶㠶".to_string().into_bytes();
    let args = [ArgValue::Str("Standard".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello");
}
#[test]
#[ignore]
fn test_lzstring_decompress_base64() {
    let op = LZStringDecompress;
    let input = b"CoCwpgHgvA==".to_vec();
    let args = [ArgValue::Str("Base64".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello");
}
#[test]
#[ignore]
fn test_lzstring_decompress_uri() {
    let op = LZStringDecompress;
    let input = b"CoCwpgHgvA".to_vec();
    let args = [ArgValue::Str("EncodedURIComponent".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "hello");
}
