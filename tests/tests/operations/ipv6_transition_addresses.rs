// Tests for the ipv6_transition_addresses operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ipv6_transition_addresses::
//
// Expected values follow the transition mechanisms these addresses come from:
// 6to4 (RFC 3056, 2002::/16), IPv4-mapped (RFC 4291, ::ffff:0:0/96),
// IPv4-translated (RFC 2765) and NAT64 (RFC 6052, 64:ff9b::/96).

use rxchef::operation::{ArgValue, OperationError};
use rxchef::operations::ipv6_transition_addresses::IPv6TransitionAddresses;
use rxchef::Operation;

fn convert(input: &str) -> String {
    let args = [ArgValue::Bool(true), ArgValue::Bool(false)];
    String::from_utf8(
        IPv6TransitionAddresses
            .run(input.as_bytes().to_vec(), &args)
            .unwrap(),
    )
    .unwrap()
}

#[test]
fn test_ipv6_transition_empty_input() {
    assert_eq!(convert(""), "");
}

#[test]
fn test_ipv6_transition_6to4_prefix() {
    // 192.168.1.1 -> 0xc0a80101, so the 6to4 prefix is 2002:c0a8:0101::/48.
    let output = convert("192.168.1.1");
    assert!(
        output.contains("2002:c0a8:0101::/48"),
        "missing 6to4 address in: {output}"
    );
}

#[test]
fn test_ipv6_transition_ipv4_mapped_address() {
    // RFC 4291 section 2.5.5.2: ::ffff:a.b.c.d
    let output = convert("192.168.1.1");
    assert!(
        output.contains("::ffff:c0a8:0101"),
        "missing IPv4-mapped address in: {output}"
    );
}

#[test]
fn test_ipv6_transition_nat64_address() {
    // RFC 6052: the well-known prefix is 64:ff9b::/96.
    let output = convert("192.168.1.1");
    assert!(
        output.contains("64:ff9b::c0a8:0101"),
        "missing NAT64 address in: {output}"
    );
}

#[test]
fn test_ipv6_transition_encodes_each_octet_as_hex() {
    // 10.0.0.255 -> 0a 00 00 ff
    let output = convert("10.0.0.255");
    assert!(
        output.contains("0a00:00ff") || output.contains("0a00:0ff") || output.contains("0a0000ff"),
        "octets not hex-encoded as expected in: {output}"
    );
}

#[test]
fn test_ipv6_transition_converts_a_mac_address_to_eui_64() {
    // RFC 4291 appendix A: split the 48-bit MAC, insert FFFE in the middle,
    // and invert the universal/local bit of the first octet (00 -> 02).
    let output = convert("00:11:22:33:44:55");
    assert!(
        output.contains("0211:22ff:fe33:4455"),
        "expected the EUI-64 interface identifier, got: {output}"
    );
}

#[test]
fn test_ipv6_transition_rejects_unrecognised_input() {
    // Regression: unparseable input used to be returned as a successful
    // result carrying the prose "Enter compressed or expanded IPv6 address,
    // IPv4 address or MAC Address.", so callers saw exit status 0 and treated
    // the message as a converted address.
    let args = [ArgValue::Bool(true), ArgValue::Bool(false)];
    let error = IPv6TransitionAddresses
        .run(b"definitely not an address".to_vec(), &args)
        .expect_err("unrecognised input must be an error");
    assert!(
        matches!(error, OperationError::InvalidInput(_)),
        "expected InvalidInput, got {error:?}"
    );
}

#[test]
fn test_ipv6_transition_error_message_names_the_offending_item() {
    let args = [ArgValue::Bool(true), ArgValue::Bool(false)];
    let error = IPv6TransitionAddresses
        .run(b"999.999.999.999x".to_vec(), &args)
        .expect_err("unrecognised input must be an error");
    assert!(
        error.to_string().contains("999.999.999.999x"),
        "error should quote the offending input, got: {error}"
    );
}
