// Tests for the rot47_brute_force operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rot47_brute_force::

use rxchef::operation::ArgValue;
use rxchef::operations::rot47_brute_force::ROT47BruteForce;
use rxchef::Operation;

#[test]
fn test_rot47_brute_force_finds_hello() {
    let op = ROT47BruteForce;
    // ROT47 of "Hello World" (rotation by 47) is "w6==@ (户C=5"
    // Brute force should find "Hello World" at amount 47
    let input = "w6==@ (户外5".as_bytes().to_vec();
    let args = [
        ArgValue::Num(100.0),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Str("Hello".to_string()),
    ];
    let result = String::from_utf8(op.run(input, &args).unwrap()).unwrap();
    assert!(result.contains("Hello"));
}
#[test]
fn test_rot47_brute_force_no_crib_all_93() {
    let op = ROT47BruteForce;
    let input = b"!".to_vec();
    let result = String::from_utf8(op.run(input, &[]).unwrap()).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 93);
}
