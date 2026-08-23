// Tests for the parse_object_id_timestamp operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_object_id_timestamp::

use rxchef::operations::parse_object_id_timestamp::ParseObjectIDTimestamp;
use rxchef::Operation;

#[test]
fn test_parse_object_id_timestamp_basic() {
    // MongoDB BSON ObjectID specification: the first four bytes are an
    // unsigned big-endian Unix timestamp. 0x5e4d3c2b = 1582119979.
    let op = ParseObjectIDTimestamp;
    let input = b"5e4d3c2b1234567890abcdef".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert_eq!(out, "2020-02-19T13:46:19.000Z");
}
#[test]
fn test_parse_object_id_timestamp_zero() {
    // ObjectID with zero timestamp (epoch)
    let op = ParseObjectIDTimestamp;
    let input = b"000000001234567890abcdef".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("1970-01-01"), "Expected epoch, got {}", out);
}
#[test]
fn test_parse_object_id_timestamp_short_input() {
    let op = ParseObjectIDTimestamp;
    let input = b"abc".to_vec();
    assert!(op.run(input, &[]).is_err());
}
#[test]
fn test_parse_object_id_timestamp_empty() {
    let op = ParseObjectIDTimestamp;
    let input = b"".to_vec();
    assert!(op.run(input, &[]).is_err());
}
#[test]
fn test_parse_object_id_timestamp_uppercase() {
    // Should handle uppercase hex
    let op = ParseObjectIDTimestamp;
    let input = b"5E4D3C2B1234567890ABCDEF".to_vec();
    let result = op.run(input, &[]).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.starts_with("2020-"));
}
