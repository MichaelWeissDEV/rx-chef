// Tests for the change_ip_format operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations change_ip_format::

use rxchef::operation::ArgValue;
use rxchef::operations::change_ip_format::ChangeIPFormat;
use rxchef::Operation;

#[test]
fn test_change_ip_format_dotted_to_hex() {
    let operation = ChangeIPFormat;
    let input = b"172.20.23.54".to_vec();
    let result = operation
        .run(
            input,
            &[
                ArgValue::Str("Dotted Decimal".to_string()),
                ArgValue::Str("Hex".to_string()),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "ac141736");
}
#[test]
fn test_change_ip_format_hex_to_dotted() {
    let operation = ChangeIPFormat;
    let input = b"ac141736".to_vec();
    let result = operation
        .run(
            input,
            &[
                ArgValue::Str("Hex".to_string()),
                ArgValue::Str("Dotted Decimal".to_string()),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "172.20.23.54");
}
#[test]
fn test_change_ip_format_decimal_to_dotted() {
    let operation = ChangeIPFormat;
    let input = b"2886997814".to_vec(); // 172.20.23.54 as decimal
    let result = operation
        .run(
            input,
            &[
                ArgValue::Str("Decimal".to_string()),
                ArgValue::Str("Dotted Decimal".to_string()),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "172.20.23.54");
}
#[test]
fn test_change_ip_format_same() {
    let operation = ChangeIPFormat;
    let input = b"192.168.1.1".to_vec();
    let result = operation
        .run(
            input,
            &[
                ArgValue::Str("Dotted Decimal".to_string()),
                ArgValue::Str("Dotted Decimal".to_string()),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "192.168.1.1");
}
