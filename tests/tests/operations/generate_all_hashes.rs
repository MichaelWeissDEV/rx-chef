// Tests for the generate_all_hashes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_all_hashes::
//
// Expected digests are published values, not values produced by rx-chef:
//   MD5      RFC 1321
//   SHA-1    FIPS 180-4
//   SHA-2    FIPS 180-4
//   SHA-3    FIPS 202
//   SHA-0    FIPS 180 (the withdrawn 1993 standard)
//
// These tests previously asserted only that the output *mentioned* each
// algorithm name. That passed while the listing reported a SHA-1 digest under
// the "SHA0" label, because no digest value was ever checked.

use rxchef::operation::ArgValue;
use rxchef::operations::generate_all_hashes::GenerateAllHashes;
use rxchef::Operation;

fn all_hashes(input: &[u8]) -> String {
    let args = [ArgValue::Str("All".to_string()), ArgValue::Bool(true)];
    String::from_utf8(GenerateAllHashes.run(input.to_vec(), &args).unwrap()).unwrap()
}

/// Read the digest reported for one algorithm label.
fn digest_of(output: &str, algorithm: &str) -> String {
    output
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            (name.trim() == algorithm).then(|| value.trim().to_string())
        })
        .unwrap_or_else(|| panic!("no {algorithm} line in:\n{output}"))
}

#[test]
fn test_generate_all_hashes_known_digests_for_hello() {
    let output = all_hashes(b"hello");
    assert_eq!(
        digest_of(&output, "MD5"),
        "5d41402abc4b2a76b9719d911017c592"
    );
    assert_eq!(
        digest_of(&output, "SHA1"),
        "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    );
    assert_eq!(
        digest_of(&output, "SHA2 256"),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    assert_eq!(
        digest_of(&output, "SHA2 512"),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca7\
         2323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_generate_all_hashes_known_digests_for_abc() {
    // FIPS 180-4 / FIPS 202 publish these for the one-block message "abc".
    let output = all_hashes(b"abc");
    assert_eq!(
        digest_of(&output, "MD5"),
        "900150983cd24fb0d6963f7d28e17f72"
    );
    assert_eq!(
        digest_of(&output, "SHA1"),
        "a9993e364706816aba3e25717850c26c9cd0d89d"
    );
    assert_eq!(
        digest_of(&output, "SHA2 224"),
        "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"
    );
    assert_eq!(
        digest_of(&output, "SHA3 256"),
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
}

#[test]
fn test_generate_all_hashes_sha0_is_not_sha1() {
    // Regression: SHA-0 was computed with SHA-1 ("using SHA1 as proxy"), so a
    // SHA-1 digest was published under the SHA-0 label. The two algorithms
    // differ by a single rotate in the message schedule and must not agree.
    let output = all_hashes(b"abc");
    assert_eq!(
        digest_of(&output, "SHA0"),
        "0164b8a914cd2a5e74c4f7ff082c4d97f1edf880",
        "SHA-0 of \"abc\" per the withdrawn FIPS 180 standard"
    );
    assert_ne!(
        digest_of(&output, "SHA0"),
        digest_of(&output, "SHA1"),
        "SHA-0 and SHA-1 must not produce the same digest"
    );
}

#[test]
fn test_generate_all_hashes_empty_input_uses_the_empty_string_digests() {
    let output = all_hashes(b"");
    assert_eq!(
        digest_of(&output, "MD5"),
        "d41d8cd98f00b204e9800998ecf8427e"
    );
    assert_eq!(
        digest_of(&output, "SHA1"),
        "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );
    assert_eq!(
        digest_of(&output, "SHA0"),
        "f96cea198ad1dd5617ac084a3d92c6107708c0ef"
    );
}

#[test]
fn test_generate_all_hashes_length_filter_selects_by_digest_size() {
    let args = [ArgValue::Str("256".to_string()), ArgValue::Bool(true)];
    let output = String::from_utf8(GenerateAllHashes.run(b"abc".to_vec(), &args).unwrap()).unwrap();
    assert!(
        !output.trim().is_empty(),
        "the 256-bit filter matched nothing"
    );
    for line in output.lines().filter(|line| !line.trim().is_empty()) {
        let digest = line.split_once(':').expect("name: digest").1.trim();
        assert_eq!(
            digest.len(),
            64,
            "the 256-bit filter admitted a {}-bit digest: {line}",
            digest.len() * 4
        );
    }
}

#[test]
fn test_generate_all_hashes_without_names_emits_only_digests() {
    let args = [ArgValue::Str("All".to_string()), ArgValue::Bool(false)];
    let output = String::from_utf8(GenerateAllHashes.run(b"abc".to_vec(), &args).unwrap()).unwrap();
    assert!(
        !output.contains("MD5:"),
        "names should be suppressed: {output}"
    );
    assert!(
        output.contains("900150983cd24fb0d6963f7d28e17f72"),
        "the MD5 digest should still be present: {output}"
    );
}

#[test]
fn test_generate_all_hashes_is_deterministic() {
    assert_eq!(all_hashes(b"abc"), all_hashes(b"abc"));
}

#[test]
fn test_generate_all_hashes_distinguishes_different_inputs() {
    assert_ne!(all_hashes(b"abc"), all_hashes(b"abd"));
}
