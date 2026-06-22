// Tests for the rabbit operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rabbit::

use rxchef::operation::ArgValue;
use rxchef::operations::rabbit::RabbitOp;
use rxchef::Operation;

#[test]
fn test_rabbit_basic() {
    let op = RabbitOp;
    let input = b"Hello".to_vec();
    let args = [
        ArgValue::Str("000102030405060708090a0b0c0d0e0f".to_string()),
        ArgValue::Str("0001020304050607".to_string()),
        ArgValue::Str("Big".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}
