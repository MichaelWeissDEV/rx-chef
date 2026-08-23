// Tests for the parse_uri operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_uri::

use rxchef::operations::parse_uri::ParseURI;
use rxchef::Operation;

#[test]
fn test_parse_uri_rejects_relative_non_uri() {
    assert!(ParseURI.run(b"not a URI".to_vec(), &[]).is_err());
}

#[test]
fn test_parse_uri() {
    let op = ParseURI;
    let input = b"https://user:pass@example.com:8080/path?a=1&b=2#hash".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Exact RFC 3986 component decomposition, including both query pairs.
    assert_eq!(
        result_str,
        "Protocol:\thttps\nAuth:\t\tuser:pass\nHostname:\texample.com\nPort:\t\t8080\nPath name:\t/path\nArguments:\n\ta = 1\n\tb = 2\nHash:\t\t#hash\n"
    );
}

#[test]
fn test_parse_uri_minimal_absolute_uri_boundary() {
    let output = ParseURI.run(b"x:a".to_vec(), &[]).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert_eq!(text.lines().next(), Some("Protocol:\tx"));
}
