// Tests for the from_message_pack operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_message_pack::

use rxchef::operations::from_message_pack::FromMessagePack;
use rxchef::Operation;

#[test]
fn test_from_message_pack_empty_input() {
    let op = FromMessagePack;
    let args = [];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_from_message_pack_simple_string() {
    let op = FromMessagePack;
    let args = [];
    // MessagePack encoded string "hello"
    let msgpack_input = vec![0xA5, 0x68, 0x65, 0x6C, 0x6C, 0x6F]; // Fixstr 5 bytes: "hello"
    let result = op.run(msgpack_input, &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    let json_str = String::from_utf8_lossy(&decoded);
    assert!(json_str.contains("hello"));
}

#[test]
fn test_from_message_pack_integer() {
    let op = FromMessagePack;
    let args = [];
    // MessagePack encoded integer 42
    let msgpack_input = vec![0x2A]; // Positive fixint 42
    let result = op.run(msgpack_input, &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    let json_str = String::from_utf8_lossy(&decoded);
    assert!(json_str.contains("42"));
}

#[test]
fn test_from_message_pack_array() {
    let op = FromMessagePack;
    let args = [];
    // MessagePack encoded array [1, 2, 3]
    let msgpack_input = vec![0x93, 0x01, 0x02, 0x03]; // Fixarray 3 elements: 1, 2, 3
    let result = op.run(msgpack_input, &args);
    assert!(result.is_ok());
    let decoded = result.unwrap();
    let json_str = String::from_utf8_lossy(&decoded);
    assert!(json_str.contains("1"));
    assert!(json_str.contains("2"));
    assert!(json_str.contains("3"));
}

#[test]
fn test_from_message_pack_invalid_data() {
    let op = FromMessagePack;
    let args = [];
    // Empty MessagePack data
    let msgpack_input = vec![];
    let result = op.run(msgpack_input, &args);
    // Should succeed with empty result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<u8>::new());
}
