// Tests for the group_ip_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations group_ip_addresses::

use rxchef::operation::ArgValue;
use rxchef::operations::group_ip_addresses::GroupIPAddresses;
use rxchef::Operation;

#[test]
fn test_group_ips_v4() {
    let op = GroupIPAddresses;
    let input = b"192.168.1.1\n192.168.1.50\n10.0.0.1\n192.168.2.1".to_vec();
    let result = op
        .run(
            input,
            &[
                ArgValue::Str("Line feed".to_string()),
                ArgValue::Num(24.0),
                ArgValue::Bool(false),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.contains("10.0.0.0/24"));
    assert!(output.contains("192.168.1.0/24"));
    assert!(output.contains("192.168.2.0/24"));
}
#[test]
fn test_group_ips_v4_only_subnets() {
    let op = GroupIPAddresses;
    let input = b"192.168.1.1\n192.168.1.50\n10.0.0.1\n192.168.2.1".to_vec();
    let result = op
        .run(
            input,
            &[
                ArgValue::Str("Line feed".to_string()),
                ArgValue::Num(24.0),
                ArgValue::Bool(true),
            ],
        )
        .unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "10.0.0.0/24\n192.168.1.0/24\n192.168.2.0/24");
}
