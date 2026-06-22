// Tests for the sm4_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sm4_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::sm4_encrypt::Sm4Encrypt;
use rxchef::Operation;

// Test vectors from IETF draft-ribose-cfrg-sm4-09
const TWO_BLOCK_PLAIN: &str =
    "aaaaaaaa bbbbbbbb cccccccc dddddddd eeeeeeee ffffffff aaaaaaaa bbbbbbbb";
const FOUR_BLOCK_PLAIN: &str = "aaaaaaaa aaaaaaaa bbbbbbbb bbbbbbbb cccccccc cccccccc dddddddd dddddddd eeeeeeee eeeeeeee ffffffff ffffffff aaaaaaaa aaaaaaaa bbbbbbbb bbbbbbbb";
const KEY_1: &str = "01234567 89abcdef fedcba98 76543210";
const KEY_2: &str = "fedcba98 76543210 01234567 89abcdef";
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
fn run(input: &str, key: &str, iv: &str, mode: &str) -> String {
    let op = Sm4Encrypt;
    let args = make_args(key, iv, mode);
    let result = op
        .run(input.as_bytes().to_vec(), &args)
        .expect("encrypt failed");
    String::from_utf8(result).unwrap()
}
fn normalize_hex(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
#[test]
fn test_ecb_no_padding_key1() {
    let out = run(TWO_BLOCK_PLAIN, KEY_1, "", "ECB/NoPadding");
    assert_eq!(
        out,
        normalize_hex("5ec8143d e509cff7 b5179f8f 474b8619 2f1d305a 7fb17df9 85f81c84 82192304")
    );
}
#[test]
fn test_ecb_no_padding_key2() {
    let out = run(TWO_BLOCK_PLAIN, KEY_2, "", "ECB/NoPadding");
    assert_eq!(
        out,
        normalize_hex("c5876897 e4a59bbb a72a10c8 3872245b 12dd90bc 2d200692 b529a415 5ac9e600")
    );
}
#[test]
fn test_ecb_with_padding_key1() {
    let out = run(TWO_BLOCK_PLAIN, KEY_1, "", "ECB");
    assert_eq!(
        out,
        normalize_hex(
            "5ec8143d e509cff7 b5179f8f 474b8619 2f1d305a 7fb17df9 85f81c84 82192304 002a8a4e fa863cca d024ac03 00bb40d2"
        )
    );
}
#[test]
fn test_cbc_no_padding_key1() {
    let out = run(TWO_BLOCK_PLAIN, KEY_1, IV, "CBC/NoPadding");
    assert_eq!(
        out,
        normalize_hex("78ebb11c c40b0a48 312aaeb2 040244cb 4cb70169 51909226 979b0d15 dc6a8f6d")
    );
}
#[test]
fn test_cbc_no_padding_key2() {
    let out = run(TWO_BLOCK_PLAIN, KEY_2, IV, "CBC/NoPadding");
    assert_eq!(
        out,
        normalize_hex("0d3a6ddc 2d21c698 85721558 7b7bb59a 91f2c147 911a4144 665e1fa1 d40bae38")
    );
}
#[test]
fn test_ofb_key1() {
    let out = run(TWO_BLOCK_PLAIN, KEY_1, IV, "OFB");
    assert_eq!(
        out,
        normalize_hex("ac3236cb 861dd316 e6413b4e 3c7524b7 1d01aca2 487ca582 cbf5463e 6698539b")
    );
}
#[test]
fn test_cfb_key1() {
    let out = run(TWO_BLOCK_PLAIN, KEY_1, IV, "CFB");
    assert_eq!(
        out,
        normalize_hex("ac3236cb 861dd316 e6413b4e 3c7524b7 69d4c54e d433b9a0 34600 9beb37b2b3f")
            .replace(" ", "")
    );
}
#[test]
fn test_ctr_key1() {
    let out = run(FOUR_BLOCK_PLAIN, KEY_1, IV, "CTR");
    assert_eq!(
        out,
        normalize_hex(
            "ac3236cb 970cc207 91364c39 5a1342d1 a3cbc187 8c6f30cd 074cce38 5cdd70c7 f234bc0e 24c11980 fd128631 0ce37b92 6e02fcd0 faa0baf3 8b293385 1d824514"
        )
    );
}
#[test]
fn test_invalid_key_length() {
    let op = Sm4Encrypt;
    let args = make_args("deadbeef", "", "ECB");
    let result = op.run(b"test".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_invalid_iv_length() {
    let op = Sm4Encrypt;
    let args = make_args(KEY_1, "deadbeef", "CBC");
    let result = op.run(b"test".to_vec(), &args);
    assert!(result.is_err());
}
