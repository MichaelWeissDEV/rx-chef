// Tests for the bson_deserialise operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bson_deserialise::

use bson::doc;
use rxchef::operations::bson_deserialise::BsonDeserialise;
use rxchef::Operation;

fn make_bson_bytes(d: &bson::Document) -> Vec<u8> {
    let mut buf = Vec::new();
    d.to_writer(&mut buf).unwrap();
    buf
}
#[test]
fn test_bson_deserialise_empty() {
    let op = BsonDeserialise;
    let result = op.run(vec![], &[]).unwrap();
    assert!(result.is_empty());
}
#[test]
fn test_bson_deserialise_simple_document() {
    let op = BsonDeserialise;
    let d = doc! { "name": "Alice", "age": 30i32 };
    let input = make_bson_bytes(&d);
    let result = op.run(input, &[]).unwrap();
    let json_str = String::from_utf8(result).unwrap();
    assert!(json_str.contains("Alice"));
    assert!(json_str.contains("name"));
}
#[test]
fn test_bson_deserialise_invalid_input() {
    let op = BsonDeserialise;
    let result = op.run(vec![0x01, 0x02, 0x03], &[]);
    assert!(result.is_err());
}
