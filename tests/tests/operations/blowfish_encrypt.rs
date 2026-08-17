// Tests for the blowfish_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blowfish_encrypt::
//
// The ECB vectors below are the published Blowfish test vectors distributed
// with the reference implementation (Schneier / Eric Young), not values taken
// from rx-chef. Because the operation applies PKCS#7 padding, an exactly
// block-sized plaintext produces a second, all-padding block; the assertions
// therefore pin the first ciphertext block against the published value.
//
// These tests previously asserted only `result.is_ok()` on a valid key, which
// says nothing about whether the cipher is correct.

use rxchef::runtime::{self, RuntimeError};

/// Run through the runtime rather than calling `Operation::run` directly, so
/// the arguments go through the same parsing every frontend uses (in
/// particular the `hex:` prefix that turns a string into bytes).
fn args(key: &str, iv: &str, mode: &str, input: &str, output: &str) -> Vec<String> {
    vec![
        key.to_string(),
        iv.to_string(),
        mode.to_string(),
        input.to_string(),
        output.to_string(),
    ]
}

fn encrypt(input: &[u8], args: &[String]) -> Result<Vec<u8>, RuntimeError> {
    runtime::run_operation("Blowfish Encrypt", input.to_vec(), args)
}

fn encrypt_ecb_hex(key_hex: &str, plaintext_hex: &str) -> String {
    let args = args(&format!("hex:{key_hex}"), "", "ECB", "Hex", "Hex");
    String::from_utf8(encrypt(plaintext_hex.as_bytes(), &args).unwrap()).unwrap()
}

/// The first 64-bit block, which is what the published vectors specify.
fn first_block(ciphertext: &str) -> String {
    ciphertext.chars().take(16).collect()
}

#[test]
fn test_blowfish_encrypt_published_ecb_vectors() {
    for (key, plaintext, expected) in [
        ("0000000000000000", "0000000000000000", "4ef997456198dd78"),
        ("ffffffffffffffff", "ffffffffffffffff", "51866fd5b85ecb8a"),
        ("3000000000000000", "1000000000000001", "7d856f9a613063f2"),
        ("0123456789abcdef", "1111111111111111", "61f9c3802281b096"),
        ("1111111111111111", "1111111111111111", "2466dd878b963c9d"),
        ("fedcba9876543210", "0123456789abcdef", "0aceab0fc6a0a28d"),
    ] {
        assert_eq!(
            first_block(&encrypt_ecb_hex(key, plaintext)),
            expected,
            "Blowfish ECB vector failed for key {key} / plaintext {plaintext}"
        );
    }
}

#[test]
fn test_blowfish_encrypt_appends_a_full_padding_block_for_block_sized_input() {
    // PKCS#7 on an exact multiple of the block size adds one whole block.
    let ciphertext = encrypt_ecb_hex("0000000000000000", "0000000000000000");
    assert_eq!(
        ciphertext.len(),
        32,
        "expected two 8-byte blocks: {ciphertext}"
    );
}

#[test]
fn test_blowfish_encrypt_roundtrips_through_blowfish_decrypt() {
    // The published vectors above pin the cipher itself; this checks that the
    // decrypt operation agrees with it rather than standing in for that proof.
    let key = "hex:0123456789abcdef";
    let ciphertext = encrypt(b"secret message", &args(key, "", "ECB", "Raw", "Hex")).unwrap();
    let recovered = runtime::run_operation(
        "Blowfish Decrypt",
        ciphertext,
        &args(key, "", "ECB", "Hex", "Raw"),
    )
    .unwrap();
    assert_eq!(recovered, b"secret message");
}

#[test]
fn test_blowfish_encrypt_cbc_depends_on_the_iv() {
    let key = "hex:0123456789abcdef";
    let with_zero_iv = encrypt(
        b"same plaintext..",
        &args(key, "hex:0000000000000000", "CBC", "Raw", "Hex"),
    )
    .unwrap();
    let with_other_iv = encrypt(
        b"same plaintext..",
        &args(key, "hex:0011223344556677", "CBC", "Raw", "Hex"),
    )
    .unwrap();
    assert_ne!(
        with_zero_iv, with_other_iv,
        "CBC ciphertext must depend on the IV"
    );
}

#[test]
fn test_blowfish_encrypt_ecb_is_deterministic() {
    let key = "hex:0123456789abcdef";
    let once = encrypt(b"repeatable", &args(key, "", "ECB", "Raw", "Hex")).unwrap();
    let twice = encrypt(b"repeatable", &args(key, "", "ECB", "Raw", "Hex")).unwrap();
    assert_eq!(once, twice);
}

#[test]
fn test_blowfish_encrypt_accepts_the_full_valid_key_length_range() {
    // Blowfish keys are 4 to 56 bytes.
    for key_bytes in [4usize, 8, 16, 24, 56] {
        let key = format!("hex:{}", "ab".repeat(key_bytes));
        assert!(
            encrypt(b"message!", &args(&key, "", "ECB", "Raw", "Hex")).is_ok(),
            "a {key_bytes}-byte key should be accepted"
        );
    }
}

#[test]
fn test_blowfish_encrypt_rejects_out_of_range_key_lengths() {
    for key_bytes in [0usize, 1, 3, 57, 64] {
        let key = format!("hex:{}", "ab".repeat(key_bytes));
        assert!(
            encrypt(b"message!", &args(&key, "", "ECB", "Raw", "Hex")).is_err(),
            "a {key_bytes}-byte key must be rejected"
        );
    }
}

#[test]
fn test_blowfish_encrypt_rejects_an_unknown_mode() {
    let error = encrypt(
        b"message!",
        &args("hex:0123456789abcdef", "", "NOTAMODE", "Raw", "Hex"),
    )
    .expect_err("an unknown mode must be rejected");
    assert!(
        matches!(error, RuntimeError::InvalidArgument { .. }),
        "expected InvalidArgument, got {error:?}"
    );
}
