// Tests for the triple_des_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations triple_des_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::triple_des_encrypt::TripleDESEncrypt;
use rxchef::Operation;

#[test]
fn test_encrypt_empty_input() {
    let op = TripleDESEncrypt;
    let result = op.run(
        b"".to_vec(),
        &[
            ArgValue::Str("000102030405060708090a0b0c0d0e0f1011121314151617".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("0000000000000000".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("CBC".to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ],
    );
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
#[test]
fn test_encrypt_cbc_produces_output() {
    let op = TripleDESEncrypt;
    let result = op.run(
        b"Hello!!!".to_vec(),
        &[
            ArgValue::Str("000102030405060708090a0b0c0d0e0f1011121314151617".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("0000000000000000".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("CBC".to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ],
    );
    assert!(result.is_ok());
    let out = String::from_utf8(result.unwrap()).unwrap();
    // 8 bytes of plaintext with PKCS7 padding -> 16 bytes ciphertext -> 32 hex chars
    assert_eq!(out.len(), 32);
}
#[test]
fn test_bad_key_length() {
    let op = TripleDESEncrypt;
    let result = op.run(
        b"Hello".to_vec(),
        &[
            ArgValue::Str("0001020304050607".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("0000000000000000".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("CBC".to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ],
    );
    assert!(result.is_err());
}
#[test]
fn test_ecb_encrypt() {
    let op = TripleDESEncrypt;
    let result = op.run(
        b"TestData".to_vec(),
        &[
            ArgValue::Str("000102030405060708090a0b0c0d0e0f1011121314151617".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("".to_string()),
            ArgValue::Str("Hex".to_string()),
            ArgValue::Str("ECB".to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Hex".to_string()),
        ],
    );
    assert!(result.is_ok());
}
