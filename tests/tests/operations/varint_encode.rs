// Tests for the varint_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations varint_encode::
//
// Expected values follow the Protocol Buffers base-128 varint encoding: the
// low 7 bits of each byte carry data, little-endian group order, and the high
// bit marks "another byte follows".

use rxchef::operations::varint_encode::VarIntEncode;
use rxchef::Operation;

fn encode(decimal: &str) -> Vec<u8> {
    VarIntEncode.run(decimal.as_bytes().to_vec(), &[]).unwrap()
}

#[test]
fn test_varint_encode_single_byte_values() {
    // Values below 128 occupy one byte with the continuation bit clear.
    assert_eq!(encode("0"), vec![0x00]);
    assert_eq!(encode("1"), vec![0x01]);
    assert_eq!(encode("127"), vec![0x7F]);
}

#[test]
fn test_varint_encode_boundary_at_128() {
    // 128 is the first value needing a continuation byte: 0x80 0x01.
    assert_eq!(encode("128"), vec![0x80, 0x01]);
}

#[test]
fn test_varint_encode_canonical_300_example() {
    // The worked example from the Protocol Buffers encoding documentation.
    assert_eq!(encode("300"), vec![0xAC, 0x02]);
}

#[test]
fn test_varint_encode_two_byte_maximum() {
    // 16383 = 2^14 - 1 is the largest two-byte varint.
    assert_eq!(encode("16383"), vec![0xFF, 0x7F]);
    assert_eq!(encode("16384"), vec![0x80, 0x80, 0x01]);
}

#[test]
fn test_varint_encode_max_u64() {
    // u64::MAX encodes as ten bytes: nine 0xFF groups then 0x01.
    assert_eq!(
        encode("18446744073709551615"),
        vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
    );
}

#[test]
fn test_varint_encode_rejects_non_numeric_input() {
    assert!(VarIntEncode.run(b"not a number".to_vec(), &[]).is_err());
    assert!(VarIntEncode.run(b"12.5".to_vec(), &[]).is_err());
}

#[test]
fn test_varint_encode_roundtrips_through_varint_decode() {
    use rxchef::operations::varint_decode::VarIntDecode;

    for value in ["0", "1", "127", "128", "300", "16384", "1234567890"] {
        let decoded = VarIntDecode.run(encode(value), &[]).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), value);
    }
}
