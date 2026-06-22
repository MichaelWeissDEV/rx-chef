// Tests for the rot13_brute_force operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rot13_brute_force::

use rxchef::operations::rot13_brute_force::ROT13BruteForce;
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
