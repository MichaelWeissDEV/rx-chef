// Tests for the keccak operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations keccak::

use rxchef::operation::ArgValue;
use rxchef::operations::keccak::Keccak;
use rxchef::Operation;

#[test]
fn test_keccak_256_test() {
    let op = Keccak;
    let result = op
        .run(b"test".to_vec(), &[ArgValue::Str("256".to_string())])
        .unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    // Keccak-256("test") from GenerateAllHashes test vectors
    assert_eq!(
        hex_out,
        "9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658"
    );
}
#[test]
fn test_keccak_224_test() {
    let op = Keccak;
    let result = op
        .run(b"test".to_vec(), &[ArgValue::Str("224".to_string())])
        .unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    // Keccak-224("test") from GenerateAllHashes test vectors
    assert_eq!(
        hex_out,
        "3be30a9ff64f34a5861116c5198987ad780165f8366e67aff4760b5e"
    );
}
#[test]
fn test_keccak_384_test() {
    let op = Keccak;
    let result = op
        .run(b"test".to_vec(), &[ArgValue::Str("384".to_string())])
        .unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    // Keccak-384("test") from GenerateAllHashes test vectors
    assert_eq!(
        hex_out,
        "53d0ba137307d4c2f9b6674c83edbd58b70c0f4340133ed0adc6fba1d2478a6a03b7788229e775d2de8ae8c0759d0527"
    );
}
#[test]
fn test_keccak_512_test() {
    let op = Keccak;
    let result = op
        .run(b"test".to_vec(), &[ArgValue::Str("512".to_string())])
        .unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    // Keccak-512("test") from GenerateAllHashes test vectors
    assert_eq!(
        hex_out,
        "1e2e9fc2002b002d75198b7503210c05a1baac4560916a3c6d93bcce3a50d7f00fd395bf1647b9abb8d1afcc9c76c289b0c9383ba386a956da4b38934417789e"
    );
}
#[test]
fn test_keccak_invalid_size() {
    let op = Keccak;
    let result = op.run(b"test".to_vec(), &[ArgValue::Str("100".to_string())]);
    assert!(result.is_err());
}
#[test]
fn test_keccak_256_empty() {
    let op = Keccak;
    let result = op
        .run(b"".to_vec(), &[ArgValue::Str("256".to_string())])
        .unwrap();
    let hex_out = String::from_utf8(result).unwrap();
    assert_eq!(hex_out.len(), 64);
}
