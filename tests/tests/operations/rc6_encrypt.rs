// Tests for the rc6_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc6_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rc6_encrypt::RC6Encrypt;
use rxchef::Operation;

#[test]
fn test_rc6_encrypt_basic() {
    let op = RC6Encrypt;
    let input = b"hello world".to_vec();
    let args = [
        ArgValue::Str("secret".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("PKCS5".to_string()),
        ArgValue::Str("32".to_string()),
        ArgValue::Str("20".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
