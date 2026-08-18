// Tests for the derive_hkdf_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations derive_hkdf_key::
//
// Known-answer vectors from RFC 5869, "HMAC-based Extract-and-Expand Key
// Derivation Function (HKDF)", appendix A. These are normative IETF values.
//
// The input carries the IKM; salt, info, hash, extract mode and output length
// are arguments. "skip" mode treats the input as an already-extracted PRK,
// which lets the expand stage be pinned on its own.

use rxchef::runtime::{self, RuntimeError};

fn hkdf(
    ikm: &[u8],
    salt: &str,
    info: &str,
    hash: &str,
    mode: &str,
    length: u32,
) -> Result<String, RuntimeError> {
    runtime::run_operation(
        "Derive HKDF key",
        ikm.to_vec(),
        &[
            salt.to_string(),
            info.to_string(),
            hash.to_string(),
            mode.to_string(),
            length.to_string(),
        ],
    )
    .map(|out| String::from_utf8_lossy(&out).into_owned())
}

fn bytes(hex_text: &str) -> Vec<u8> {
    hex::decode(hex_text).expect("test vector must be valid hex")
}

#[test]
fn test_derive_hkdf_key_rfc_5869_test_case_1_sha256() {
    // A.1: basic case with salt and info.
    let ikm = bytes(&"0b".repeat(22));
    assert_eq!(
        hkdf(
            &ikm,
            "hex:000102030405060708090a0b0c",
            "hex:f0f1f2f3f4f5f6f7f8f9",
            "SHA256",
            "with salt",
            42
        )
        .unwrap(),
        "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
    );
}

#[test]
fn test_derive_hkdf_key_rfc_5869_test_case_2_longer_inputs() {
    // A.2: longer IKM, salt and info.
    let ikm: Vec<u8> = (0u8..=0x4f).collect();
    let salt: String = (0x60u8..=0xaf).map(|b| format!("{b:02x}")).collect();
    let info: String = (0xb0u8..=0xff).map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        hkdf(
            &ikm,
            &format!("hex:{salt}"),
            &format!("hex:{info}"),
            "SHA256",
            "with salt",
            82
        )
        .unwrap(),
        "b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c\
         59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71\
         cc30c58179ec3e87c14c01d5c1f3434f1d87"
            .replace('\n', "")
            .replace(' ', "")
    );
}

#[test]
fn test_derive_hkdf_key_rfc_5869_test_case_3_zero_length_salt_and_info() {
    // A.3: salt and info are both empty.
    let ikm = bytes(&"0b".repeat(22));
    assert_eq!(
        hkdf(&ikm, "", "", "SHA256", "with salt", 42).unwrap(),
        "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8"
    );
}

#[test]
fn test_derive_hkdf_key_rfc_5869_test_case_4_sha1() {
    // A.4: the SHA-1 variant, kept for legacy interoperability.
    let ikm = bytes(&"0b".repeat(11));
    assert_eq!(
        hkdf(
            &ikm,
            "hex:000102030405060708090a0b0c",
            "hex:f0f1f2f3f4f5f6f7f8f9",
            "SHA1",
            "with salt",
            42
        )
        .unwrap(),
        "085a01ea1b10f36933068b56efa5ad81a4f14b822f5b091568a9cdd4f155fda2c22e422478d305f3f896"
    );
}

#[test]
fn test_derive_hkdf_key_skip_mode_expands_a_ready_made_prk() {
    // The PRK from RFC 5869 A.1, expanded directly. This pins the expand
    // stage independently of extraction.
    let prk = bytes("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5");
    assert_eq!(
        hkdf(&prk, "", "hex:f0f1f2f3f4f5f6f7f8f9", "SHA256", "skip", 42).unwrap(),
        "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
    );
}

#[test]
fn test_derive_hkdf_key_output_length_is_honoured() {
    let ikm = bytes(&"0b".repeat(22));
    for length in [1u32, 16, 32, 42, 64] {
        let okm = hkdf(&ikm, "", "", "SHA256", "with salt", length).unwrap();
        assert_eq!(okm.len(), (length * 2) as usize, "L={length}");
    }
}

#[test]
fn test_derive_hkdf_key_shorter_lengths_are_prefixes_of_longer_ones() {
    // HKDF-Expand is a stream: a shorter request is a prefix of a longer one.
    let ikm = bytes(&"0b".repeat(22));
    let short = hkdf(&ikm, "", "", "SHA256", "with salt", 16).unwrap();
    let long = hkdf(&ikm, "", "", "SHA256", "with salt", 42).unwrap();
    assert!(long.starts_with(&short));
}

#[test]
fn test_derive_hkdf_key_salt_and_info_change_the_result() {
    let ikm = bytes(&"0b".repeat(22));
    let base = hkdf(&ikm, "", "", "SHA256", "with salt", 32).unwrap();
    let salted = hkdf(&ikm, "hex:00010203", "", "SHA256", "with salt", 32).unwrap();
    let informed = hkdf(&ikm, "", "hex:f0f1f2", "SHA256", "with salt", 32).unwrap();
    assert_ne!(base, salted, "salt must affect the key");
    assert_ne!(base, informed, "info must affect the key");
}

#[test]
fn test_derive_hkdf_key_rejects_an_unknown_hash_function() {
    let ikm = bytes(&"0b".repeat(22));
    assert!(hkdf(&ikm, "", "", "NotAHash", "with salt", 32).is_err());
}

#[test]
fn test_derive_hkdf_key_rejects_an_unknown_extract_mode() {
    let ikm = bytes(&"0b".repeat(22));
    assert!(hkdf(&ikm, "", "", "SHA256", "NotAMode", 32).is_err());
}

#[test]
fn test_derive_hkdf_key_rejects_an_output_longer_than_255_hash_blocks() {
    // RFC 5869 section 2.3 caps L at 255 * HashLen; for SHA-256 that is 8160.
    let ikm = bytes(&"0b".repeat(22));
    assert!(
        hkdf(&ikm, "", "", "SHA256", "with salt", 8161).is_err(),
        "an over-long output must be rejected"
    );
}

#[test]
fn test_derive_hkdf_key_empty_ikm_is_defined() {
    // HKDF places no lower bound on the IKM.
    assert!(hkdf(b"", "", "", "SHA256", "with salt", 32).is_ok());
}
