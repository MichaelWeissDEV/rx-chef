// Tests for the to_base45 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base45::

use rxchef::operation::ArgValue;
use rxchef::operations::from_base45::FromBase45;
use rxchef::operations::to_base45::ToBase45;
use rxchef::Operation;

fn run(input: &[u8]) -> String {
    let op = ToBase45;
    let result = op.run(input.to_vec(), &[]).unwrap();
    String::from_utf8(result).unwrap()
}

// RFC 9285 section 4.3 test vectors.
#[test]
fn test_rfc9285_vectors() {
    assert_eq!(run(b"AB"), "BB8");
    assert_eq!(run(b"Hello!!"), "%69 VD92EX0");
    assert_eq!(run(b"base-45"), "UJCLQE7W581");
}

#[test]
fn test_empty_input() {
    assert_eq!(run(b""), "");
}

#[test]
fn test_single_byte_produces_two_chars() {
    // An odd-length final chunk of 1 byte encodes to exactly 2 base45 chars
    // (c and d digits only, no e digit), per RFC 9285.
    assert_eq!(run(&[0x00]), "00");
    // 255 = 5*45 + 30 -> c=30 ('U'), d=5 ('5')
    assert_eq!(run(&[0xff]), "U5");
}

#[test]
fn test_args_schema_alphabet_is_documented_but_unused() {
    // to_base45's run() ignores the supplied args entirely and always uses
    // the fixed RFC 9285 alphabet, so passing an arbitrary alphabet argument
    // has no effect on the output.
    let op = ToBase45;
    let args = [ArgValue::Str("unused".to_string())];
    let with_args = op.run(b"AB".to_vec(), &args).unwrap();
    let without_args = op.run(b"AB".to_vec(), &[]).unwrap();
    assert_eq!(with_args, without_args);
    assert_eq!(String::from_utf8(with_args).unwrap(), "BB8");
}

#[test]
fn test_roundtrip_with_from_base45() {
    let to_op = ToBase45;
    let from_op = FromBase45;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let encoded = to_op.run(input.clone(), &[]).unwrap();
    let decoded = from_op.run(encoded, &[]).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_binary_data_roundtrip() {
    let to_op = ToBase45;
    let from_op = FromBase45;
    let input: Vec<u8> = (0..=255).collect();
    let encoded = to_op.run(input.clone(), &[]).unwrap();
    let decoded = from_op.run(encoded, &[]).unwrap();
    assert_eq!(decoded, input);
}
