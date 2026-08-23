// Tests for the gost_hash operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations gost_hash::

use rxchef::operation::{ArgValue, OperationError};
use rxchef::operations::gost_hash::GostHash;
use rxchef::Operation;

fn args(algorithm: &str, digest_length: &str, sbox: &str) -> [ArgValue; 3] {
    [
        ArgValue::Str(algorithm.to_string()),
        ArgValue::Str(digest_length.to_string()),
        ArgValue::Str(sbox.to_string()),
    ]
}

fn hash(input: &[u8], algorithm: &str, digest_length: &str, sbox: &str) -> String {
    let args = args(algorithm, digest_length, sbox);
    String::from_utf8(GostHash.run(input.to_vec(), &args).unwrap()).unwrap()
}

#[test]
fn test_streebog_256_standard_empty_vector() {
    // GOST R 34.11-2012 / RFC 6986-compatible empty-message vector.
    assert_eq!(
        hash(b"", "GOST R 34.11 (2012)", "256", "E-TEST"),
        "3f539a213e97c802cc229d474c6aa32a825a360b2a933a949fd925208d9ce1bb"
    );
}

#[test]
fn test_gost94_cryptopro_empty_vector() {
    // GOST R 34.11-94 CryptoPro/D-A empty-message regression vector.
    assert_eq!(
        hash(b"", "GOST 28147 (1994)", "256", "D-A"),
        "981e5f3ca30c841487830f84fb433e13ac1101569b9c13584ac483234cd656c0"
    );
}

#[test]
fn test_gost_hash_gost94_produces_256_bit_digest() {
    // GOST R 34.11-94 is defined as a 256-bit hash: 64 hexadecimal digits.
    let digest = hash(b"", "GOST 28147 (1994)", "256", "E-TEST");
    assert_eq!(digest.len(), 64);
    assert!(digest.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_gost_hash_streebog_digest_lengths() {
    // GOST R 34.11-2012 (Streebog) is defined for 256- and 512-bit digests.
    assert_eq!(hash(b"", "GOST R 34.11 (2012)", "256", "E-TEST").len(), 64);
    assert_eq!(hash(b"", "GOST R 34.11 (2012)", "512", "E-TEST").len(), 128);
}

#[test]
fn test_gost_hash_streebog_lengths_differ() {
    let short = hash(b"abc", "GOST R 34.11 (2012)", "256", "E-TEST");
    let long = hash(b"abc", "GOST R 34.11 (2012)", "512", "E-TEST");
    assert_ne!(short, long);
    assert!(!long.starts_with(&short));
}

#[test]
fn test_gost_hash_sbox_selects_a_different_parameter_set() {
    let test_sbox = hash(b"abc", "GOST 28147 (1994)", "256", "E-TEST");
    let cryptopro = hash(b"abc", "GOST 28147 (1994)", "256", "CryptoPro");
    assert_ne!(
        test_sbox, cryptopro,
        "the test and CryptoPro parameter sets must not produce the same digest"
    );
}

#[test]
fn test_gost_hash_is_deterministic() {
    assert_eq!(
        hash(b"abc", "GOST 28147 (1994)", "256", "E-TEST"),
        hash(b"abc", "GOST 28147 (1994)", "256", "E-TEST")
    );
}

#[test]
fn test_gost_hash_differs_for_different_input() {
    assert_ne!(
        hash(b"abc", "GOST 28147 (1994)", "256", "E-TEST"),
        hash(b"abd", "GOST 28147 (1994)", "256", "E-TEST")
    );
}

#[test]
fn test_gost_hash_rejects_unknown_algorithm() {
    // Regression: an unrecognised algorithm used to fall through to Streebog
    // and silently return a digest from a different hash function.
    let error = GostHash
        .run(
            b"abc".to_vec(),
            &args("not-a-gost-algorithm", "256", "E-TEST"),
        )
        .expect_err("an unknown algorithm must be rejected");
    assert!(
        matches!(&error, OperationError::InvalidArgument { name, .. } if name == "Algorithm"),
        "expected an InvalidArgument error naming Algorithm, got {error:?}"
    );
}

#[test]
fn test_gost_hash_rejects_unknown_sbox() {
    let error = GostHash
        .run(b"abc".to_vec(), &args("GOST 28147 (1994)", "256", "nope"))
        .expect_err("an unknown parameter set must be rejected");
    assert!(
        matches!(&error, OperationError::InvalidArgument { name, .. } if name == "sBox"),
        "expected an InvalidArgument error naming sBox, got {error:?}"
    );
}

#[test]
fn test_gost_hash_rejects_invalid_streebog_digest_length() {
    let error = GostHash
        .run(
            b"abc".to_vec(),
            &args("GOST R 34.11 (2012)", "384", "E-TEST"),
        )
        .expect_err("only 256 and 512 bit Streebog digests exist");
    assert!(
        matches!(&error, OperationError::InvalidArgument { name, .. } if name == "Digest length"),
        "expected an InvalidArgument error naming Digest length, got {error:?}"
    );
}
