// Tests for the derive_pbkdf2_key operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations derive_pbkdf2_key::
//
// Known-answer vectors from RFC 6070, "PBKDF2 Test Vectors", which publishes
// PBKDF2-HMAC-SHA1 results for the parameter sets below. These are normative
// IETF values, not output captured from rx-chef.
//
// The operation takes the key size in bits, so RFC 6070's dkLen of 20 octets
// is requested as 160.

use rxchef::runtime::{self, RuntimeError};

fn derive(
    passphrase: &str,
    key_bits: u32,
    iterations: u32,
    hash: &str,
    salt: &str,
) -> Result<String, RuntimeError> {
    runtime::run_operation(
        "Derive PBKDF2 key",
        Vec::new(),
        &[
            passphrase.to_string(),
            key_bits.to_string(),
            iterations.to_string(),
            hash.to_string(),
            salt.to_string(),
        ],
    )
    .map(|out| String::from_utf8_lossy(&out).into_owned())
}

#[test]
fn test_derive_pbkdf2_key_rfc_6070_vectors() {
    // (iterations, dkLen bits, expected) for P = "password", S = "salt".
    for (iterations, bits, expected) in [
        (1u32, 160u32, "0c60c80f961f0e71f3a9b524af6012062fe037a6"),
        (2, 160, "ea6c014dc72d6f8ccd1ed92ace1d41f0d8de8957"),
        (4096, 160, "4b007901b765489abead49d926f721d065a429c1"),
    ] {
        assert_eq!(
            derive("password", bits, iterations, "SHA1", "salt")
                .expect("a published vector must succeed"),
            expected,
            "RFC 6070 vector with c={iterations} failed"
        );
    }
}

#[test]
fn test_derive_pbkdf2_key_rfc_6070_longer_salt_and_key() {
    // RFC 6070's fourth vector: P = "passwordPASSWORDpassword",
    // S = "saltSALTsaltSALTsaltSALTsaltSALTsalt", c = 4096, dkLen = 25.
    assert_eq!(
        derive(
            "passwordPASSWORDpassword",
            200,
            4096,
            "SHA1",
            "saltSALTsaltSALTsaltSALTsaltSALTsalt"
        )
        .expect("a published vector must succeed"),
        "3d2eec4fe41c849b80c8d83662c0e44a8b291a964cf2f07038"
    );
}

#[test]
fn test_derive_pbkdf2_key_output_length_follows_the_requested_size() {
    for bits in [128u32, 160, 256, 512] {
        let derived = derive("password", bits, 1, "SHA256", "salt").unwrap();
        assert_eq!(
            derived.len(),
            (bits / 8 * 2) as usize,
            "{bits}-bit key should be {} hex characters",
            bits / 8 * 2
        );
    }
}

#[test]
fn test_derive_pbkdf2_key_iterations_change_the_result() {
    let one = derive("password", 160, 1, "SHA1", "salt").unwrap();
    let two = derive("password", 160, 2, "SHA1", "salt").unwrap();
    assert_ne!(one, two, "the iteration count must affect the key");
}

#[test]
fn test_derive_pbkdf2_key_salt_changes_the_result() {
    let a = derive("password", 160, 1, "SHA1", "salt").unwrap();
    let b = derive("password", 160, 1, "SHA1", "pepper").unwrap();
    assert_ne!(a, b, "the salt must affect the key");
}

#[test]
fn test_derive_pbkdf2_key_hash_function_changes_the_result() {
    let sha1 = derive("password", 160, 1, "SHA1", "salt").unwrap();
    let sha256 = derive("password", 160, 1, "SHA256", "salt").unwrap();
    assert_ne!(sha1, sha256, "the PRF must affect the key");
}

#[test]
fn test_derive_pbkdf2_key_empty_passphrase_and_salt_are_defined() {
    // PKCS#5 places no lower bound on either, so both must derive a key
    // rather than fail.
    let derived = derive("", 128, 1, "SHA256", "").unwrap();
    assert_eq!(derived.len(), 32);
}

#[test]
fn test_derive_pbkdf2_key_rejects_zero_iterations() {
    // RFC 8018 requires c to be a positive integer.
    assert!(
        derive("password", 160, 0, "SHA1", "salt").is_err(),
        "zero iterations must be rejected"
    );
}

#[test]
fn test_derive_pbkdf2_key_rejects_an_unknown_hash_function() {
    assert!(derive("password", 160, 1, "NotAHash", "salt").is_err());
}

#[test]
fn test_derive_pbkdf2_key_is_deterministic() {
    assert_eq!(
        derive("password", 160, 100, "SHA256", "salt").unwrap(),
        derive("password", 160, 100, "SHA256", "salt").unwrap()
    );
}
