// Tests for the bson_serialise operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bson_serialise::

use rxchef::operations::bson_serialise::BsonSerialise;
use rxchef::Operation;

#[test]
fn test_bson_serialise_empty() {
    let op = BsonSerialise;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_bson_serialise_simple_object() {
    let op = BsonSerialise;
    let input = b"{\"name\": \"Alice\", \"age\": 30}".to_vec();
    let result = op.run(input, &[]).unwrap();
    // BSON documents begin with their total byte length as a little-endian i32
    assert!(result.len() >= 5);
    // Verify it roundtrips via bson
    let mut cursor = std::io::Cursor::new(result);
    let doc = bson::Document::from_reader(&mut cursor).unwrap();
    assert_eq!(doc.get_str("name").unwrap(), "Alice");
}
#[test]
fn test_bson_serialise_non_object_fails() {
    let op = BsonSerialise;
    let input = b"[1,2,3]".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
#[test]
fn test_bson_serialise_invalid_json_fails() {
    let op = BsonSerialise;
    let input = b"not json".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
