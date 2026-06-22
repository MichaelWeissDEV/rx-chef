// Tests for the protobuf_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations protobuf_encode::

use rxchef::operations::protobuf_encode::ProtobufEncode;
use rxchef::Operation;

#[test]
fn test_protobuf_encode_basic() {
    let op = ProtobufEncode;
    let input = br#"{"1": 123, "2": "hello"}"#.to_vec();
    let result = op.run(input, &[]).unwrap();
    // Field 1, wire type 0, value 123 -> 0x08, 0x7b
    // Field 2, wire type 2, len 5, value "hello" -> 0x12, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f
    assert_eq!(
        result,
        vec![0x08, 0x7b, 0x12, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]
    );
}
#[test]
fn test_protobuf_encode_nested() {
    let op = ProtobufEncode;
    let input = br#"{"3": {"1": 1}}"#.to_vec();
    let result = op.run(input, &[]).unwrap();
    // Field 3, wire type 2, len 2, value {Field 1, wire type 0, value 1}
    // 0x1a, 0x02, 0x08, 0x01
    assert_eq!(result, vec![0x1a, 0x02, 0x08, 0x01]);
}
#[test]
fn test_protobuf_encode_repeated() {
    let op = ProtobufEncode;
    let input = br#"{"1": [1, 2]}"#.to_vec();
    let result = op.run(input, &[]).unwrap();
    // Field 1, wire type 0, value 1
    // Field 1, wire type 0, value 2
    // 0x08, 0x01, 0x08, 0x02
    assert_eq!(result, vec![0x08, 0x01, 0x08, 0x02]);
}
