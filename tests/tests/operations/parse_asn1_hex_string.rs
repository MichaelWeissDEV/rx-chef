// Tests for the parse_asn1_hex_string operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_asn1_hex_string::

use rxchef::operations::parse_asn1_hex_string::ParseASN1HexString;
use rxchef::Operation;

#[test]
fn test_parse_asn1_single_integer_boundary() {
    // DER INTEGER 5: universal tag 2, length 1, value 05.
    let output = ParseASN1HexString.run(b"020105".to_vec(), &[]).unwrap();
    assert_eq!(output, b"INTEGER: 05\n");
}

#[test]
fn test_parse_asn1_rejects_non_hex_input() {
    let error = ParseASN1HexString.run(b"zz".to_vec(), &[]).unwrap_err();
    assert!(matches!(
        error,
        rxchef::operation::OperationError::InvalidInput(_)
    ));
}

#[test]
fn test_parse_asn1_hex_basic() {
    let op = ParseASN1HexString;
    // 30 03 02 01 05 -> SEQUENCE { INTEGER 5 }
    let input = b"3003020105".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("SEQUENCE"));
    assert!(result_str.contains("INTEGER"));
}
