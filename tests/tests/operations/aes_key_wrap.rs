// Tests for the aes_key_wrap operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations aes_key_wrap::
//
// Known-answer vectors from RFC 3394, "Advanced Encryption Standard (AES) Key
// Wrap Algorithm", section 4 ("Test Vectors"). Each case below cites the
// subsection it comes from. These are normative values published by the IETF,
// not output captured from rx-chef or from CyberChef.
//
// The default IV a6a6a6a6a6a6a6a6 is the alternative initial value defined in
// RFC 3394 section 2.2.3.1.

use rxchef::runtime::{self, RuntimeError};

const RFC3394_IV: &str = "hex:a6a6a6a6a6a6a6a6";

fn wrap(kek_hex: &str, key_hex: &str) -> Result<String, RuntimeError> {
    runtime::run_operation(
        "AES Key Wrap",
        key_hex.as_bytes().to_vec(),
        &[
            format!("hex:{kek_hex}"),
            RFC3394_IV.to_string(),
            "Hex".to_string(),
            "Hex".to_string(),
        ],
    )
    .map(|out| String::from_utf8_lossy(&out).into_owned())
}

/// The six RFC 3394 section 4 vectors: (section, KEK, key data, wrapped).
const VECTORS: &[(&str, &str, &str, &str)] = &[
    (
        "4.1 (128-bit KEK, 128-bit key)",
        "000102030405060708090a0b0c0d0e0f",
        "00112233445566778899aabbccddeeff",
        "1fa68b0a8112b447aef34bd8fb5a7b829d3e862371d2cfe5",
    ),
    (
        "4.2 (192-bit KEK, 128-bit key)",
        "000102030405060708090a0b0c0d0e0f1011121314151617",
        "00112233445566778899aabbccddeeff",
        "96778b25ae6ca435f92b5b97c050aed2468ab8a17ad84e5d",
    ),
    (
        "4.3 (256-bit KEK, 128-bit key)",
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00112233445566778899aabbccddeeff",
        "64e8c3f9ce0f5ba263e9777905818a2a93c8191e7d6e8ae7",
    ),
    (
        "4.4 (192-bit KEK, 192-bit key)",
        "000102030405060708090a0b0c0d0e0f1011121314151617",
        "00112233445566778899aabbccddeeff0001020304050607",
        "031d33264e15d33268f24ec260743edce1c6c7ddee725a936ba814915c6762d2",
    ),
    (
        "4.5 (256-bit KEK, 192-bit key)",
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00112233445566778899aabbccddeeff0001020304050607",
        "a8f9bc1612c68b3ff6e6f4fbe30e71e4769c8b80a32cb8958cd5d17d6b254da1",
    ),
    (
        "4.6 (256-bit KEK, 256-bit key)",
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "00112233445566778899aabbccddeeff000102030405060708090a0b0c0d0e0f",
        "28c9f404c4b810f4cbccb35cfb87f8263f5786e2d80ed326cbc7f0e71a99f43bfb988b9b7a02dd21",
    ),
];

#[test]
fn test_aes_key_wrap_rfc_3394_vectors() {
    for (section, kek, key, expected) in VECTORS {
        assert_eq!(
            wrap(kek, key).expect("wrapping a published vector must succeed"),
            *expected,
            "RFC 3394 section {section} failed"
        );
    }
}

#[test]
fn test_aes_key_wrap_output_is_one_block_longer_than_the_key() {
    // RFC 3394 section 2: the wrapped result is the key plus one 64-bit block.
    for (_, kek, key, _) in VECTORS {
        let wrapped = wrap(kek, key).unwrap();
        assert_eq!(
            wrapped.len(),
            key.len() + 16,
            "wrapped length should exceed the key by one 8-byte block"
        );
    }
}

#[test]
fn test_aes_key_wrap_unwraps_back_to_the_original_key() {
    for (section, kek, key, wrapped) in VECTORS {
        let unwrapped = runtime::run_operation(
            "AES Key Unwrap",
            wrapped.as_bytes().to_vec(),
            &[
                format!("hex:{kek}"),
                RFC3394_IV.to_string(),
                "Hex".to_string(),
                "Hex".to_string(),
            ],
        )
        .expect("unwrapping a published vector must succeed");
        assert_eq!(
            String::from_utf8_lossy(&unwrapped),
            *key,
            "RFC 3394 section {section} did not round-trip"
        );
    }
}

#[test]
fn test_aes_key_wrap_rejects_an_invalid_kek_length() {
    // AES keys are 128, 192 or 256 bits; anything else is not a KEK.
    for bad in ["00", "0011223344556677", &"ab".repeat(20)] {
        assert!(
            wrap(bad, "00112233445566778899aabbccddeeff").is_err(),
            "a {}-byte KEK must be rejected",
            bad.len() / 2
        );
    }
}

#[test]
fn test_aes_key_wrap_rejects_key_data_that_is_not_a_multiple_of_64_bits() {
    // RFC 3394 section 2 requires the key data to be a whole number of
    // 64-bit blocks, and at least two of them.
    let kek = "000102030405060708090a0b0c0d0e0f";
    assert!(wrap(kek, "00112233445566778899aabbccddee").is_err());
    assert!(
        wrap(kek, "0011223344556677").is_err(),
        "one block is too short"
    );
}

#[test]
fn test_aes_key_wrap_rejects_empty_key_data() {
    assert!(wrap("000102030405060708090a0b0c0d0e0f", "").is_err());
}

#[test]
fn test_aes_key_wrap_rejects_an_invalid_iv_length() {
    // The integrity check value is exactly 64 bits.
    let result = runtime::run_operation(
        "AES Key Wrap",
        b"00112233445566778899aabbccddeeff".to_vec(),
        &[
            "hex:000102030405060708090a0b0c0d0e0f".to_string(),
            "hex:a6a6a6".to_string(),
            "Hex".to_string(),
            "Hex".to_string(),
        ],
    );
    assert!(result.is_err(), "a short IV must be rejected");
}

#[test]
fn test_aes_key_wrap_is_deterministic() {
    let (_, kek, key, _) = VECTORS[0];
    assert_eq!(wrap(kek, key).unwrap(), wrap(kek, key).unwrap());
}
