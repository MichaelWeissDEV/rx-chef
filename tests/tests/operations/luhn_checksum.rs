// Tests for the luhn_checksum operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations luhn_checksum::

use rxchef::operation::ArgValue;
use rxchef::operations::luhn_checksum::LuhnChecksum;
use rxchef::Operation;

fn run_op(input: &str, radix: usize) -> String {
    let op = LuhnChecksum;
    let args = [ArgValue::Num(radix as f64)];
    let result = op.run(input.as_bytes().to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}
#[test]
fn test_luhn_empty() {
    let op = LuhnChecksum;
    let args = [ArgValue::Num(10.0)];
    let result = op.run(b"".to_vec(), &args).unwrap();
    assert_eq!(result, b"");
}
#[test]
fn test_luhn_standard_data_1() {
    let out = run_op("35641709012469", 10);
    assert_eq!(
        out,
        "Checksum: 7\nCheckdigit: 0\nLuhn Validated String: 356417090124690"
    );
}
#[test]
fn test_luhn_standard_data_2() {
    let out = run_op("896101950123440000", 10);
    assert_eq!(
        out,
        "Checksum: 5\nCheckdigit: 1\nLuhn Validated String: 8961019501234400001"
    );
}
#[test]
fn test_luhn_standard_data_3() {
    let out = run_op("35726908971331", 10);
    assert_eq!(
        out,
        "Checksum: 6\nCheckdigit: 7\nLuhn Validated String: 357269089713317"
    );
}
#[test]
fn test_luhn_radix_10_basic() {
    let out = run_op("0123456789", 10);
    assert_eq!(
        out,
        "Checksum: 7\nCheckdigit: 7\nLuhn Validated String: 01234567897"
    );
}
#[test]
fn test_luhn_radix_16() {
    let out = run_op("0123456789abcdef", 16);
    assert_eq!(
        out,
        "Checksum: 4\nCheckdigit: 4\nLuhn Validated String: 0123456789abcdef4"
    );
}
#[test]
fn test_luhn_invalid_radix_odd() {
    let op = LuhnChecksum;
    let args = [ArgValue::Num(11.0)];
    let result = op.run(b"123".to_vec(), &args);
    assert!(result.is_err());
}
#[test]
fn test_luhn_invalid_radix_out_of_range() {
    let op = LuhnChecksum;
    let args = [ArgValue::Num(38.0)];
    let result = op.run(b"123".to_vec(), &args);
    assert!(result.is_err());
}
