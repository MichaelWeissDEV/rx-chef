// Tests for the parse_uri operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_uri::

use rxchef::operations::parse_uri::ParseURI;
use rxchef::Operation;

#[test]
fn test_parse_uri() {
    let op = ParseURI;
    let input = b"https://user:pass@example.com:8080/path?a=1&b=2#hash".to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Protocol:\thttps"));
    assert!(result_str.contains("Hostname:\texample.com"));
    assert!(result_str.contains("Port:\t\t8080"));
    assert!(result_str.contains("Path name:\t/path"));
    assert!(result_str.contains("Arguments:"));
    assert!(result_str.contains("a = 1"));
    assert!(result_str.contains("Hash:\t\t#hash"));
}
