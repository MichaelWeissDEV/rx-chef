// Tests for the analyse_hash operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations analyse_hash::

use rxchef::operations::analyse_hash::AnalyseHash;
use rxchef::Operation;

#[test]
fn test_analyse_hash_md5() {
    let operation = AnalyseHash;
    // MD5 is 32 hex characters = 128 bits
    let input = b"5d41402abc4b2a76b9719d911017c592".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("MD5"));
}
#[test]
fn test_analyse_hash_sha1() {
    let operation = AnalyseHash;
    // SHA-1 is 40 hex characters = 160 bits
    let input = b"aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("SHA-1"));
}
#[test]
fn test_analyse_hash_invalid() {
    let operation = AnalyseHash;
    let input = b"not-a-hex-string".to_vec();
    let result = operation.run(input, &[]);
    assert!(result.is_err());
}
