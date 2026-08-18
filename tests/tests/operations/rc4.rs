// Tests for the rc4 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc4::
//
// Known-answer vectors are the widely published RC4 test values that appear in
// the algorithm's original description and in RFC 6229's introduction material
// (the "Key"/"Plaintext" and "Wiki"/"pedia" pairs). They are reproduced in
// every reference implementation and are not values captured from rx-chef.
//
// RC4 is a legacy stream cipher with known biases and is not suitable for new
// designs; it is implemented here for compatibility with existing data.

use rxchef::runtime::{self, RuntimeError};

fn rc4(
    input: &[u8],
    key: &str,
    input_format: &str,
    output_format: &str,
) -> Result<String, RuntimeError> {
    runtime::run_operation(
        "RC4",
        input.to_vec(),
        &[
            key.to_string(),
            input_format.to_string(),
            output_format.to_string(),
        ],
    )
    .map(|out| String::from_utf8_lossy(&out).into_owned())
}

#[test]
fn test_rc4_published_vectors() {
    for (key, plaintext, expected) in [
        ("Key", "Plaintext", "bbf316e8d940af0ad3"),
        ("Wiki", "pedia", "1021bf0420"),
        ("Secret", "Attack at dawn", "45a01f645fc35b383552544b9bf5"),
    ] {
        assert_eq!(
            rc4(plaintext.as_bytes(), key, "Raw", "Hex").unwrap(),
            expected,
            "RC4 vector for key {key:?} failed"
        );
    }
}

#[test]
fn test_rc4_is_its_own_inverse() {
    // RC4 XORs a keystream, so applying it twice returns the plaintext.
    let ciphertext = rc4(b"Attack at dawn", "Secret", "Raw", "Hex").unwrap();
    let recovered = rc4(ciphertext.as_bytes(), "Secret", "Hex", "Raw").unwrap();
    assert_eq!(recovered, "Attack at dawn");
}

#[test]
fn test_rc4_keystream_depends_on_the_key() {
    let a = rc4(b"same plaintext", "key-a", "Raw", "Hex").unwrap();
    let b = rc4(b"same plaintext", "key-b", "Raw", "Hex").unwrap();
    assert_ne!(a, b);
}

#[test]
fn test_rc4_output_length_matches_the_input_length() {
    // A stream cipher neither pads nor truncates.
    for length in [0usize, 1, 15, 16, 17, 256, 1000] {
        let plaintext = vec![b'x'; length];
        let out = rc4(&plaintext, "Key", "Raw", "Hex").unwrap();
        assert_eq!(out.len(), length * 2, "length {length}");
    }
}

#[test]
fn test_rc4_empty_input_produces_empty_output() {
    assert_eq!(rc4(b"", "Key", "Raw", "Hex").unwrap(), "");
}

#[test]
fn test_rc4_handles_binary_input() {
    // Every byte value must pass through the keystream unharmed.
    let binary: Vec<u8> = (0u8..=255).collect();
    let ciphertext = rc4(&binary, "Key", "Raw", "Hex").unwrap();
    assert_eq!(ciphertext.len(), 512);
    let recovered = runtime::run_operation(
        "RC4",
        ciphertext.into_bytes(),
        &["Key".to_string(), "Hex".to_string(), "Raw".to_string()],
    )
    .unwrap();
    assert_eq!(recovered, binary);
}

#[test]
fn test_rc4_rejects_an_empty_key() {
    // RC4's key schedule is undefined for a zero-length key.
    assert!(
        rc4(b"plaintext", "", "Raw", "Hex").is_err(),
        "an empty key must be rejected"
    );
}

#[test]
fn test_rc4_rejects_malformed_hex_input() {
    assert!(rc4(b"not hex at all", "Key", "Hex", "Raw").is_err());
}

#[test]
fn test_rc4_is_deterministic() {
    assert_eq!(
        rc4(b"Plaintext", "Key", "Raw", "Hex").unwrap(),
        rc4(b"Plaintext", "Key", "Raw", "Hex").unwrap()
    );
}
