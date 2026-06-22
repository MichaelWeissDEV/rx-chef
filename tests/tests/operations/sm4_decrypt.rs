// Tests for the sm4_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sm4_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::sm4_decrypt::Sm4Decrypt;
use rxchef::Operation;

// Test vectors from IETF draft-ribose-cfrg-sm4-09 (reversed from Encrypt)
const KEY_1: &str = "01234567 89abcdef fedcba98 76543210";
const IV: &str = "00010203 04050607 08090a0b 0c0d0e0f";
fn make_args(key: &str, iv: &str, mode: &str) -> Vec<ArgValue> {
    vec![
        ArgValue::Str(key.to_string()),
        ArgValue::Str(iv.to_string()),
        ArgValue::Str(mode.to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ]
}
fn run_decrypt(input: &str, key: &str, iv: &str, mode: &str) -> String {
    let op = Sm4Decrypt;
    let args = make_args(key, iv, mode);
    let result = op
        .run(input.as_bytes().to_vec(), &args)
        .expect("decrypt failed");
    String::from_utf8(result).unwrap()
}
fn normalize_hex(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
#[test]
fn test_ecb_no_padding_decrypt() {
    let input = "5ec8143d e509cff7 b5179f8f 474b8619 2f1d305a 7fb17df9 85f81c84 82192304";
    let expected = "aaaaaaaa bbbbbbbb cccccccc dddddddd eeeeeeee ffffffff aaaaaaaa bbbbbbbb";
    let out = run_decrypt(input, KEY_1, "", "ECB/NoPadding");
    assert_eq!(out, normalize_hex(expected));
}
#[test]
fn test_cbc_no_padding_decrypt() {
    let input = "78ebb11c c40b0a48 312aaeb2 040244cb 4cb70169 51909226 979b0d15 dc6a8f6d";
    let expected = "aaaaaaaa bbbbbbbb cccccccc dddddddd eeeeeeee ffffffff aaaaaaaa bbbbbbbb";
    let out = run_decrypt(input, KEY_1, IV, "CBC/NoPadding");
    assert_eq!(out, normalize_hex(expected));
}
#[test]
fn test_ctr_decrypt() {
    let input = "ac3236cb 970cc207 91364c39 5a1342d1 a3cbc187 8c6f30cd 074cce38 5cdd70c7 f234bc0e 24c11980 fd128631 0ce37b92 6e02fcd0 faa0baf3 8b293385 1d824514";
    let expected = "aaaaaaaa aaaaaaaa bbbbbbbb bbbbbbbb cccccccc cccccccc dddddddd dddddddd eeeeeeee eeeeeeee ffffffff ffffffff aaaaaaaa aaaaaaaa bbbbbbbb bbbbbbbb";
    let out = run_decrypt(input, KEY_1, IV, "CTR");
    assert_eq!(out, normalize_hex(expected));
}
#[test]
fn test_invalid_padding() {
    let op = Sm4Decrypt;
    // 16 bytes of data, but last byte is 20 (invalid padding for 16-byte block)
    let input = hex::decode("000102030405060708090a0b0c0d0e14").unwrap();
    let key = hex::decode("0123456789abcdeffedcba9876543210").unwrap();
    let args = [
        ArgValue::Str(hex::encode(key)),
        ArgValue::Str("".to_string()),
        ArgValue::Str("ECB".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(hex::encode(input).into_bytes(), &args);
    assert!(result.is_err());
}
