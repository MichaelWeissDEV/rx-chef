// Tests for the from_punycode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations from_punycode::

use rxchef::operation::ArgValue;
use rxchef::operations::from_punycode::FromPunycode;
use rxchef::Operation;

#[test]
fn test_from_punycode_ascii_only() {
    let op = FromPunycode;
    // mnchen-3ya is münchen
    let input = b"mnchen-3ya".to_vec();
    let args = [ArgValue::Bool(false)];
    let result = op.run(input, &args);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(result.unwrap()).unwrap(), "münchen");
}
#[test]
fn test_from_punycode_idn_domain() {
    let op = FromPunycode;
    let input = b"xn--nxasmq6b.com".to_vec();
    let args = [ArgValue::Bool(true)];
    let result = op.run(input, &args);
    assert!(result.is_ok());
}
