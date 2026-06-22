// Tests for the sha2 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations sha2::

use rxchef::operation::ArgValue;
use rxchef::operations::sha2::SHA2;
use rxchef::Operation;

#[test]
fn test_sha2_256_basic() {
    let operation = SHA2;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("256".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA2-256 hash of "Hello, World!"
    assert_eq!(
        output,
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    );
}
#[test]
fn test_sha2_224_basic() {
    let operation = SHA2;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("224".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA2-224 hash of "Hello, World!"
    assert_eq!(
        output,
        "72a23dfa411ba6fde01dbfabf3b00a709c93ebf273dc29e2d8b261ff"
    );
}
#[test]
fn test_sha2_384_basic() {
    let operation = SHA2;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("384".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA2-384 hash of "Hello, World!"
    assert_eq!(
        output,
        "5485cc9b3365b4305dfb4e8337e0a598a574f8242bf17289e0dd6c20a3cd44a089de16ab4ab308f63e44b1170eb5f515"
    );
}
#[test]
fn test_sha2_512_basic() {
    let operation = SHA2;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("512".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA2-512 hash of "Hello, World!"
    assert_eq!(
        output,
        "374d794a95cdcfd8b35993185fef9ba368f160d8daf432d08ba9f1ed1e5abe6cc69291e0fa2fe0006a52570ef18c19def4e617c33ce52ef0a6e5fbe318cb0387"
    );
}
#[test]
fn test_sha2_512_256_basic() {
    let operation = SHA2;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("512/256".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA2-512/256 hash of "Hello, World!"
    assert_eq!(
        output,
        "0686f0a605973dc1bf035d1e2b9bad1985a0bff712ddd88abd8d2593e5f99030"
    );
}
#[test]
fn test_sha2_512_224_basic() {
    let operation = SHA2;
    let input = b"Hello, World!".to_vec();
    let result = operation
        .run(input, &[ArgValue::Str("512/224".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    // SHA2-512/224 hash of "Hello, World!"
    assert_eq!(
        output,
        "766745f058e8a0438f19de48ae56ea5f123fe738af39bca050a7547a"
    );
}
#[test]
fn test_sha2_invalid_size() {
    let operation = SHA2;
    let input = b"test".to_vec();
    let result = operation.run(input, &[ArgValue::Str("500".to_string())]);
    assert!(result.is_err());
}
#[test]
fn test_sha2_rounds_validation() {
    let operation = SHA2;
    let input = b"test".to_vec();
    let args = [
        ArgValue::Str("256".to_string()),
        ArgValue::Num(10.0), // Invalid - less than 16
        ArgValue::Num(160.0),
    ];
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
#[test]
fn test_sha2_binary() {
    let operation = SHA2;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation
        .run(input, &[ArgValue::Str("256".to_string())])
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output.len(), 64); // 256 bits = 64 hex chars
}
