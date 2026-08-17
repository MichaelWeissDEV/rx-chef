// Tests for the protobuf_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations protobuf_decode::
//
// Wire-format expectations follow the Protocol Buffers encoding specification:
// each field is a varint key of (field_number << 3 | wire_type) followed by the
// payload. Wire type 0 is varint, 2 is length-delimited.

use rxchef::operation::ArgValue;
use rxchef::operations::protobuf_decode::ProtobufDecode;
use rxchef::Operation;

fn decode(bytes: &[u8]) -> serde_json::Value {
    let args = [
        ArgValue::Str(String::new()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let output = ProtobufDecode.run(bytes.to_vec(), &args).unwrap();
    serde_json::from_slice(&output).expect("the operation declares JSON output")
}

#[test]
fn test_protobuf_decode_varint_field() {
    // Key 0x08 = field 1, wire type 0; payload 0x96 0x01 = varint 150.
    assert_eq!(decode(&[0x08, 0x96, 0x01]), serde_json::json!({"1": 150}));
}

#[test]
fn test_protobuf_decode_length_delimited_field() {
    // Key 0x12 = field 2, wire type 2; length 3; payload "abc".
    assert_eq!(
        decode(&[0x12, 0x03, b'a', b'b', b'c']),
        serde_json::json!({"2": "abc"})
    );
}

#[test]
fn test_protobuf_decode_multiple_fields() {
    assert_eq!(
        decode(&[0x08, 0x96, 0x01, 0x12, 0x03, b'a', b'b', b'c']),
        serde_json::json!({"1": 150, "2": "abc"})
    );
}

#[test]
fn test_protobuf_decode_small_varint_values() {
    assert_eq!(decode(&[0x08, 0x00]), serde_json::json!({"1": 0}));
    assert_eq!(decode(&[0x08, 0x01]), serde_json::json!({"1": 1}));
    assert_eq!(decode(&[0x08, 0x7F]), serde_json::json!({"1": 127}));
}

#[test]
fn test_protobuf_decode_high_field_numbers() {
    // Field 16, wire type 0: key = (16 << 3) | 0 = 128 -> varint 0x80 0x01.
    assert_eq!(decode(&[0x80, 0x01, 0x2A]), serde_json::json!({"16": 42}));
}

#[test]
fn test_protobuf_decode_empty_input_is_an_empty_message() {
    assert_eq!(decode(&[]), serde_json::json!({}));
}

#[test]
fn test_protobuf_decode_empty_length_delimited_field_becomes_an_empty_message() {
    // Without a schema, wire type 2 is ambiguous between string, bytes, and a
    // nested message. A zero-length payload is decoded as an empty nested
    // message rather than an empty string.
    assert_eq!(decode(&[0x12, 0x00]), serde_json::json!({"2": {}}));
}

#[test]
fn test_protobuf_decode_rejects_a_truncated_varint() {
    let args = [
        ArgValue::Str(String::new()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    // 0xFF sets the continuation bit with no following byte.
    assert!(ProtobufDecode.run(vec![0xFF, 0xFF], &args).is_err());
}

#[test]
fn test_protobuf_decode_rejects_a_truncated_payload() {
    let args = [
        ArgValue::Str(String::new()),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    // Declares 10 bytes but supplies 2.
    assert!(ProtobufDecode
        .run(vec![0x12, 0x0A, b'a', b'b'], &args)
        .is_err());
}
