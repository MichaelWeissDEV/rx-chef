// Tests for the analyse_uuid operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations analyse_uuid::

use rxchef::operations::analyse_uuid::AnalyseUUID;
use rxchef::Operation;

#[test]
fn test_analyse_uuid_v1() {
    let operation = AnalyseUUID;
    let input = b"6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Version:\n1"));
}
#[test]
fn test_analyse_uuid_v4() {
    let operation = AnalyseUUID;
    let input = b"550e8400-e29b-41d4-a716-446655440000".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("Version:\n4"));
}
#[test]
fn test_analyse_uuid_invalid() {
    let operation = AnalyseUUID;
    let input = b"not-a-uuid".to_vec();
    let result = operation.run(input, &[]);
    assert!(result.is_err());
}
