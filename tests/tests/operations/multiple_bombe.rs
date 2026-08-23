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
    let result: serde_json::Value = serde_json::from_slice(&result).unwrap();
    assert!(result["n_loops"].as_u64().unwrap() > 0);
    assert!(result["bombe_runs"].is_array());
}

#[test]
fn test_multiple_bombe_recovers_upstream_reference_stop() {
    let args = [
        ArgValue::Str("User defined".into()),
        ArgValue::Str("EKMFLGDQVZNTOWYHXUSPAIBRCJ<R\nAJDKSIRUXBLHWTMCQGZNPYFVOE<F\nBDFHJLCPRTXVZNYEIWGAKMUSQO<W".into()),
        ArgValue::Str(String::new()),
        ArgValue::Str("AY BR CU DH EQ FS GL IP JX KN MO TZ VW".into()),
        ArgValue::Str("THISISATESTMESSAGE".into()),
        ArgValue::Num(0.0),
        ArgValue::Bool(false),
    ];
    let output = String::from_utf8(MultipleBombe.run(b"BBYFLTHHYIJQAYBBYS".to_vec(), &args).unwrap()).unwrap();
    assert!(output.contains("LGA"), "{output}");
    assert!(output.contains("SS"), "{output}");
    assert!(output.contains("VFISUSGTKSTMPSUNAK"), "{output}");
}
