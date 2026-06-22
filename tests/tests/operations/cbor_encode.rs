// Tests for the cbor_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cbor_encode::

use rxchef::operations::cbor_encode::CBOREncode;
use rxchef::Operation;

#[test]
fn test_cbor_encode_empty_input() {
    let op = CBOREncode;
    let input = vec![];
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_cbor_encode_simple_integer() {
    let op = CBOREncode;
    let input = b"42".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of integer 42 should be 0x18 0x2a
    assert_eq!(result, vec![0x18, 0x2a]);
}

#[test]
fn test_cbor_encode_simple_string() {
    let op = CBOREncode;
    let input = b"\"hello\"".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of string "hello" should be 0x65 0x68 0x65 0x6c 0x6c 0x6f
    assert_eq!(result, vec![0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
}

#[test]
fn test_cbor_encode_array() {
    let op = CBOREncode;
    let input = b"[1, 2, 3]".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of array [1, 2, 3] should be 0x83 0x01 0x02 0x03
    assert_eq!(result, vec![0x83, 0x01, 0x02, 0x03]);
}

#[test]
fn test_cbor_encode_object() {
    let op = CBOREncode;
    let input = b"{\"a\": 1, \"b\": 2}".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of object {"a": 1, "b": 2}
    // Should be 0xa2 0x61 0x61 0x01 0x61 0x62 0x02
    assert_eq!(result, vec![0xa2, 0x61, 0x61, 0x01, 0x61, 0x62, 0x02]);
}

#[test]
fn test_cbor_encode_boolean_true() {
    let op = CBOREncode;
    let input = b"true".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of true should be 0xf5
    assert_eq!(result, vec![0xf5]);
}

#[test]
fn test_cbor_encode_boolean_false() {
    let op = CBOREncode;
    let input = b"false".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of false should be 0xf4
    assert_eq!(result, vec![0xf4]);
}

#[test]
fn test_cbor_encode_null() {
    let op = CBOREncode;
    let input = b"null".to_vec();
    let result = op.run(input, &[]).unwrap();
    // CBOR encoding of null should be 0xf6
    assert_eq!(result, vec![0xf6]);
}

#[test]
fn test_cbor_encode_invalid_json() {
    let op = CBOREncode;
    let input = b"not valid json".to_vec();
    let result = op.run(input, &[]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid JSON"));
}

#[test]
fn test_cbor_encode_nested_structures() {
    let op = CBOREncode;
    let input = b"{\"data\": [1, 2, {\"nested\": true}]}".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Should contain the expected CBOR structure
    assert!(!result.is_empty());
    // Should start with map marker (0xa1 for 1-element map)
    assert_eq!(result[0], 0xa1);
}

#[test]
fn test_cbor_encode_float() {
    let op = CBOREncode;
    let input = b"3.14".to_vec();
    let result = op.run(input, &[]).unwrap();
    // Should contain CBOR float encoding
    assert!(!result.is_empty());
}
