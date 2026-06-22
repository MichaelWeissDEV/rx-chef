// Tests for the generate_uuid operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_uuid::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_uuid::GenerateUUID;
use rxchef::Operation;

#[test]
fn test_generate_uuid_v4() {
    let op = GenerateUUID;
    let result = op.run(vec![], &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    // UUID format: 8-4-4-4-12
    assert_eq!(s.len(), 36);
    let parts: Vec<&str> = s.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
}
#[test]
fn test_generate_uuid_invalid_version() {
    let op = GenerateUUID;
    let result = op.run(vec![], &[ArgValue::Str("v1".to_string())]);
    assert!(result.is_err());
}
#[test]
fn test_generate_uuid_uniqueness() {
    let op = GenerateUUID;
    let a = op.run(vec![], &[]).expect("first uuid");
    let b = op.run(vec![], &[]).expect("second uuid");
    assert_ne!(a, b);
}
