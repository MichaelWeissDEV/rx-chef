// Tests for the php_deserialize operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations php_deserialize::

use rxchef::operation::ArgValue;
use rxchef::operations::php_deserialize::PHPDeserialize;
use rxchef::Operation;

#[test]
fn test_php_deserialize_basic() {
    let op = PHPDeserialize;
    let input = b"a:2:{s:1:\"a\";i:10;i:0;a:1:{s:2:\"ab\";b:1;}}".to_vec();
    let result = op.run(input, &[ArgValue::Bool(true)]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "{\"a\": 10,\"0\": {\"ab\": true}}"
    );
}
#[test]
fn test_php_deserialize_invalid_json() {
    let op = PHPDeserialize;
    let input = b"a:2:{s:1:\"a\";i:10;i:0;a:1:{s:2:\"ab\";b:1;}}".to_vec();
    let result = op.run(input, &[ArgValue::Bool(false)]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "{\"a\": 10,0: {\"ab\": true}}"
    );
}
