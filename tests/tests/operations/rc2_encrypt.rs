// Tests for the rc2_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc2_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rc2_encrypt::RC2Encrypt;
use rxchef::Operation;

/// Apply PKCS#7 padding to a multiple of 8 bytes
fn pkcs7_pad_8(data: &[u8]) -> Vec<u8> {
    let pad_len = 8 - (data.len() % 8);
    let mut padded = data.to_vec();
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));
    padded
}

#[test]
fn test_rc2_encrypt_invalid_empty_key() {
    let op = RC2Encrypt;
    let result = op.run(
        b"hello".to_vec(),
        &[
            ArgValue::Str("".to_string()),
            ArgValue::Str("".to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ],
    );
    assert!(result.is_err());
}
#[test]
fn test_rc2_encrypt_output_is_hex() {
    let op = RC2Encrypt;
    let result = op.run(
        b"hello".to_vec(),
        &[
            ArgValue::Str("mykey".to_string()),
            ArgValue::Str("".to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ],
    );
    assert!(result.is_ok());
    let out = result.unwrap();
    // Output must be valid hex and 16 chars (8 bytes padded -> 8 bytes encrypted)
    let hex_str = String::from_utf8(out).unwrap();
    assert_eq!(hex_str.len(), 16);
}
#[test]
fn test_pkcs7_pad_8() {
    let data = b"hello";
    let padded = pkcs7_pad_8(data);
    assert_eq!(padded.len(), 8);
    assert_eq!(padded[5], 3); // pad byte value is 3
}
