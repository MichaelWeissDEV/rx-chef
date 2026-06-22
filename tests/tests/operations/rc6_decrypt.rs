// Tests for the rc6_decrypt operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rc6_decrypt::

use rxchef::operation::ArgValue;
use rxchef::operations::rc6_decrypt::RC6Decrypt;
use rxchef::Operation;

#[test]
fn test_rc6_decrypt_basic() {
    let op = RC6Decrypt;
    // This is a dummy test just to ensure it runs
    let input = b"00".to_vec();
    let args = [
        ArgValue::Str("secret".to_string()),
        ArgValue::Str("".to_string()),
        ArgValue::Str("CBC".to_string()),
        ArgValue::Str("Hex".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("PKCS5".to_string()),
        ArgValue::Str("32".to_string()),
        ArgValue::Str("20".to_string()),
    ];
    let _ = op.run(input, &args);
}
