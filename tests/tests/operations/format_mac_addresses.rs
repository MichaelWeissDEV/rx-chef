// Tests for the format_mac_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations format_mac_addresses::

use rxchef::operation::ArgValue;
use rxchef::operations::format_mac_addresses::FormatMACAddresses;
use rxchef::Operation;

/// Insert a separator character every `every` hex digits.
fn insert_sep(clean: &str, sep: char, every: usize) -> String {
    let chars: Vec<char> = clean.chars().collect();
    let mut result = String::new();
    let mut count = 0;
    for (i, c) in chars.iter().enumerate() {
        result.push(*c);
        count += 1;
        if count == every && i + 1 < chars.len() {
            result.push(sep);
            count = 0;
        }
    }
    result
}

#[test]
fn test_format_mac_lower_only() {
    let op = FormatMACAddresses;
    let input = b"AA:BB:CC:DD:EE:FF".to_vec();
    let args = [
        ArgValue::Str("Lower only".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("aabbccddeeff"));
    assert!(out.contains("aa-bb-cc-dd-ee-ff"));
    assert!(out.contains("aa:bb:cc:dd:ee:ff"));
}
#[test]
fn test_format_mac_upper_only() {
    let op = FormatMACAddresses;
    let input = b"aa:bb:cc:dd:ee:ff".to_vec();
    let args = [
        ArgValue::Str("Upper only".to_string()),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
        ArgValue::Bool(false),
    ];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    assert!(out.contains("AABBCCDDEEFF"));
}
#[test]
fn test_insert_sep() {
    assert_eq!(insert_sep("aabbccddeeff", '-', 2), "aa-bb-cc-dd-ee-ff");
    assert_eq!(insert_sep("aabbccddeeff", '.', 4), "aabb.ccdd.eeff");
}
