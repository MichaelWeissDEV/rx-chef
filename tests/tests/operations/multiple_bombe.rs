// Tests for the multiple_bombe operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations multiple_bombe::

use rxchef::operation::ArgValue;
use rxchef::operations::multiple_bombe::MultipleBombe;
use rxchef::Operation;

#[test]
fn test_multiple_bombe_basic() {
    let op = MultipleBombe;
    let input = b"HELLOWORLD".to_vec();
    let args = [
        ArgValue::Str("".to_string()),
        ArgValue::Str(
            "EKMFLGDQVZNTOWYHXUSPAIBRCJ\nAJDKSIRUXBLHWTMCQGZNPYFVOE\nBDFHJLCPRTXVZNYEIWGAKMUSQO"
                .to_string(),
        ),
        ArgValue::Str("".to_string()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".to_string()),
        ArgValue::Str("HELLO".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(result.len() > 0);
}
