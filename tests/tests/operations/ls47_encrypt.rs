// Tests for the ls47_encrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ls47_encrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::ls47_encrypt::LS47Encrypt;
use rxchef::Operation;

#[test]
fn test_ls47_encrypt_basic() {
    let op = LS47Encrypt;
    let input = b"hello".to_vec();
    let args = vec![
        ArgValue::Str("password".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Str("".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
