// Tests for the varint_decode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations varint_decode::
//
// Expected values follow the Protocol Buffers base-128 varint encoding.

use rxchef::operations::varint_decode::VarIntDecode;
use rxchef::Operation;

fn decode(bytes: &[u8]) -> String {
    String::from_utf8(VarIntDecode.run(bytes.to_vec(), &[]).unwrap()).unwrap()
}

#[test]
fn test_varint_decode_empty_input_is_zero() {
    // No groups contribute no bits, so the accumulator stays at zero.
    assert_eq!(decode(&[]), "0");
}

#[test]
fn test_varint_decode_single_byte_values() {
    assert_eq!(decode(&[0x00]), "0");
    assert_eq!(decode(&[0x01]), "1");
    assert_eq!(decode(&[0x7F]), "127");
}

#[test]
fn test_varint_decode_boundary_at_128() {
    assert_eq!(decode(&[0x80, 0x01]), "128");
}

#[test]
fn test_varint_decode_canonical_300_example() {
    // The worked example from the Protocol Buffers encoding documentation.
    assert_eq!(decode(&[0xAC, 0x02]), "300");
}

#[test]
fn test_varint_decode_max_u64() {
    assert_eq!(
        decode(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]),
        "18446744073709551615"
    );
}

#[test]
fn test_varint_decode_stops_at_the_first_terminating_byte() {
    // Trailing bytes after a byte with the continuation bit clear are not
    // part of this varint and must not change the value.
    assert_eq!(decode(&[0xAC, 0x02, 0xFF, 0xFF]), "300");
}

#[test]
fn test_varint_decode_truncated_input_returns_the_partial_value() {
    // Documented divergence from a strict decoder: when the final byte still
    // has the continuation bit set the varint is incomplete, but upstream
    // CyberChef returns the bits accumulated so far rather than failing, and
    // this port matches that behaviour deliberately.
    assert_eq!(decode(&[0x80]), "0");
    assert_eq!(decode(&[0xAC]), "44");
}

#[test]
fn test_varint_decode_handles_values_wider_than_64_bits() {
    // The implementation accumulates into a big integer, so oversized inputs
    // widen rather than wrap.
    assert_eq!(
        decode(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]),
        "151115727451828646838271"
    );
}
