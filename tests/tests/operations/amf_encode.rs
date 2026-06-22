// Tests for the amf_encode operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations amf_encode::

use rxchef::operation::ArgValue;
use rxchef::operations::amf_encode::AmfEncode;
use rxchef::Operation;

#[test]
fn test_amf_encode_format_selection() {
    let op = AmfEncode;
    let input = b"test".to_vec();
    let args = [ArgValue::Str("AMF0".to_string())];
    let result = op.run(input.clone(), &args);
    assert!(result.is_err());
    let args = [ArgValue::Str("AMF3".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
}
