// Tests for the parse_ip_range operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_ip_range::

use rxchef::operation::ArgValue;
use rxchef::operations::parse_ip_range::ParseIPRange;
use rxchef::Operation;

#[test]
fn test_cidr_slash30() {
    let op = ParseIPRange;
    let args = vec![
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(b"192.168.1.0/30".to_vec(), &args).unwrap();
    // RFC 4632: /30 leaves two host bits, hence four addresses and mask /30.
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "Network: 192.168.1.0/30\n\
CIDR Notation: 192.168.1.0/30\n\
Mask: 255.255.255.252\n\
Hosts: 4\n\
First: 192.168.1.0\n\
Last: 192.168.1.3\n\n\
192.168.1.0\n\
192.168.1.1\n\
192.168.1.2\n\
192.168.1.3"
    );
}
#[test]
fn test_hyphenated_range() {
    let op = ParseIPRange;
    let args = vec![
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(b"10.0.0.1 - 10.0.0.3".to_vec(), &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("10.0.0.1"));
    assert!(out.contains("10.0.0.2"));
    assert!(out.contains("10.0.0.3"));
}
#[test]
fn test_large_range_blocked() {
    let op = ParseIPRange;
    let args = vec![
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    // /8 would be 16M addresses - should fail
    assert!(op.run(b"10.0.0.0/8".to_vec(), &args).is_err());
}
#[test]
fn test_single_ip() {
    let op = ParseIPRange;
    let args = vec![
        ArgValue::Bool(false),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
    ];
    let result = op.run(b"1.2.3.4".to_vec(), &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("1.2.3.4"));
}
