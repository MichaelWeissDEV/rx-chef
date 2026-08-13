// Tests for the to_base85 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations to_base85::

use rxchef::operation::ArgValue;
use rxchef::operations::from_base85::FromBase85;
use rxchef::operations::to_base85::ToBase85;
use rxchef::Operation;

fn run(input: &[u8]) -> String {
    let op = ToBase85;
    let result = op.run(input.to_vec(), &[]).unwrap();
    String::from_utf8(result).unwrap()
}

// Classic Ascii85 test vector (Wikipedia "Ascii85" article; also used by
// Adobe's original btoa tool): the 4 bytes "Man " encode to "9jqo^".
#[test]
fn test_classic_ascii85_vector() {
    assert_eq!(run(b"Man "), "9jqo^");
}

#[test]
fn test_empty_input() {
    let op = ToBase85;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_all_zero_block_uses_z_shortcut() {
    // A full 4-byte zero block collapses to a single 'z' in the standard
    // alphabet.
    assert_eq!(run(&[0, 0, 0, 0]), "z");
    assert_eq!(run(&[0, 0, 0, 0, 0, 0, 0, 0]), "zz");
}

#[test]
fn test_include_delimiter() {
    let op = ToBase85;
    let args = [ArgValue::Bool(false), ArgValue::Bool(true)];
    let result = op.run(b"Man ".to_vec(), &args).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "<~9jqo^~>");
}

#[test]
fn test_invalid_alphabet_length_errors() {
    let op = ToBase85;
    let args = [ArgValue::Str("short".to_string())];
    let result = op.run(b"data".to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_roundtrip_with_from_base85() {
    let to_op = ToBase85;
    let from_op = FromBase85;
    let input = b"The quick brown fox jumps over the lazy dog".to_vec();
    let encoded = to_op.run(input.clone(), &[]).unwrap();
    let decoded = from_op.run(encoded, &[]).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_binary_data_with_zero_bytes_roundtrip() {
    let to_op = ToBase85;
    let from_op = FromBase85;
    let input: Vec<u8> = vec![0, 0, 0, 0, 1, 2, 3, 0, 0, 0, 0, 255, 254, 253];
    let encoded = to_op.run(input.clone(), &[]).unwrap();
    let decoded = from_op.run(encoded, &[]).unwrap();
    assert_eq!(decoded, input);
}
