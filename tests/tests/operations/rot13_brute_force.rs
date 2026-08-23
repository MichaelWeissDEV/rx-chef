// Tests for the rot13_brute_force operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rot13_brute_force::

use rxchef::operations::rot13_brute_force::ROT13BruteForce;
use rxchef::operation::ArgValue;
use rxchef::Operation;

#[test]
fn test_brute_force_contains_rot13() {
    let op = ROT13BruteForce;
    let result = String::from_utf8(op.run(b"Uryyb".to_vec(), &[]).unwrap()).unwrap();
    assert!(result.contains("Hello"));
}
#[test]
fn test_25_rotations() {
    let op = ROT13BruteForce;
    let result = String::from_utf8(op.run(b"ABC".to_vec(), &[]).unwrap()).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 25);
}

#[test]
fn test_rot13_brute_force_matches_pinned_cyberchef_crib_result() {
    let op = ROT13BruteForce;
    let args = [
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Num(100.0),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Str("Hello".to_string()),
    ];
    assert_eq!(
        op.run(b"Uryyb".to_vec(), &args).unwrap(),
        b"Amount = 13: Hello"
    );
}

#[test]
fn test_rot13_brute_force_zero_length_boundary() {
    let op = ROT13BruteForce;
    let args = [
        ArgValue::Bool(true),
        ArgValue::Bool(true),
        ArgValue::Bool(false),
        ArgValue::Num(0.0),
    ];
    let output = String::from_utf8(op.run(b"ABC".to_vec(), &args).unwrap()).unwrap();
    assert_eq!(output.lines().count(), 25);
    assert_eq!(output.lines().next(), Some("Amount =  1: "));
}

#[test]
fn test_rot13_brute_force_rejects_invalid_utf8_sample() {
    let op = ROT13BruteForce;
    assert!(op.run(vec![0xff], &[]).is_err());
}
