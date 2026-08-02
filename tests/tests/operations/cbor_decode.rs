// Tests for the cbor_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cbor_decode::

use rxchef::operations::cbor_decode::CBORDecode;
use rxchef::Operation;

#[test]
fn test_cbor_decode_empty_input() {
    let op = CBORDecode;
    let input = vec![];
    let result = op.run(input, &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_cbor_decode_simple_integer() {
    let op = CBORDecode;
    // CBOR encoding of integer 42
    let input = vec![0x18, 0x2a]; // Major type 1 (unsigned int), value 42
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("42"));
}

#[test]
fn test_cbor_decode_simple_string() {
    let op = CBORDecode;
    // CBOR encoding of string "hello"
    let input = vec![0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f]; // Major type 3 (text string), length 5, "hello"
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("hello"));
}

#[test]
fn test_cbor_decode_array() {
    let op = CBORDecode;
    // CBOR encoding of array [1, 2, 3]
    let input = vec![0x83, 0x01, 0x02, 0x03]; // Major type 4 (array), length 3, values 1, 2, 3
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("1"));
    assert!(result_str.contains("2"));
    assert!(result_str.contains("3"));
}

#[test]
fn test_cbor_decode_map() {
    let op = CBORDecode;
    // CBOR encoding of map {"a": 1, "b": 2}
    let input = vec![
        0xa2, // Major type 5 (map), length 2
        0x61, 0x61, // "a"
        0x01, // 1
        0x61, 0x62, // "b"
        0x02, // 2
    ];
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("a"));
    assert!(result_str.contains("b"));
    assert!(result_str.contains("1"));
    assert!(result_str.contains("2"));
}

#[test]
fn test_cbor_decode_nested_structures() {
    let op = CBORDecode;
    // CBOR encoding of nested structure {"data": [1, 2, {"nested": true}]}
    let input = vec![
        0xa1, // map, length 1
        0x64, 0x64, 0x61, 0x74, 0x61, // "data"
        0x83, // array, length 3
        0x01, // 1
        0x02, // 2
        0xa1, // map, length 1
        0x66, 0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, // "nested"
        0xf5, // true
    ];
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("data"));
    assert!(result_str.contains("nested"));
    assert!(result_str.contains("true"));
}

#[test]
fn test_cbor_decode_invalid_input() {
    let op = CBORDecode;
    // Invalid CBOR data
    let input = vec![0xff, 0xff, 0xff];
    let result = op.run(input, &[]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("CBOR decode failed"));
}

#[test]
fn test_cbor_decode_boolean_true() {
    let op = CBORDecode;
    // CBOR encoding of true
    let input = vec![0xf5]; // true
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("true"));
}

#[test]
fn test_cbor_decode_boolean_false() {
    let op = CBORDecode;
    // CBOR encoding of false
    let input = vec![0xf4]; // false
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("false"));
}

#[test]
fn test_cbor_decode_null_value() {
    let op = CBORDecode;
    // CBOR encoding of null
    let input = vec![0xf6]; // null
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("null"));
}
