// Tests for the rabbit operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rabbit::

use rxchef::operation::ArgValue;
use rxchef::operations::rabbit::RabbitOp;
use rxchef::Operation;

#[test]
fn test_rabbit_rfc_4503_zero_key_vector() {
    // RFC 4503 appendix A.1: first 48 octets for an all-zero key and no IV.
    let output = RabbitOp
        .run(
            vec![0; 48],
            &[
                ArgValue::Str("hex:00000000000000000000000000000000".into()),
                ArgValue::Str("hex:".into()),
                ArgValue::Str("Big".into()),
                ArgValue::Str("Raw".into()),
                ArgValue::Str("Hex".into()),
            ],
        )
        .unwrap();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "b15754f036a5d6ecf56b45261c4af70288e8d815c59c0c397b696c4789c68aa7\
         f416a1c3700cd451da68d1881673d696"
            .replace(' ', "")
    );
}

#[test]
fn test_rabbit_basic() {
    let op = RabbitOp;
    let input = b"Hello".to_vec();
    let args = [
        ArgValue::Str("hex:000102030405060708090a0b0c0d0e0f".to_string()),
        ArgValue::Str("hex:0001020304050607".to_string()),
        ArgValue::Str("Big".to_string()),
        ArgValue::Str("Raw".to_string()),
        ArgValue::Str("Hex".to_string()),
    ];
    let result = op.run(input, &args).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_rabbit_both_endiannesses_round_trip_and_differ() {
    let op = RabbitOp;
    let input = b"Rabbit supports partial blocks too".to_vec();
    let mut ciphertexts = Vec::new();
    for endianness in ["Big", "Little"] {
        let args = [
            ArgValue::Str("hex:000102030405060708090a0b0c0d0e0f".to_string()),
            ArgValue::Str("hex:0001020304050607".to_string()),
            ArgValue::Str(endianness.to_string()),
            ArgValue::Str("Raw".to_string()),
            ArgValue::Str("Raw".to_string()),
        ];
        let encrypted = op.run(input.clone(), &args).unwrap();
        assert_eq!(op.run(encrypted.clone(), &args).unwrap(), input);
        ciphertexts.push(encrypted);
    }
    assert_ne!(ciphertexts[0], ciphertexts[1]);
}
