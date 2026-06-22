// Tests for the generate_de_bruijn_sequence operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations generate_de_bruijn_sequence::

use rxchef::operation::ArgValue;
use rxchef::operations::generate_de_bruijn_sequence::GenerateDeBruijnSequence;
use rxchef::Operation;

#[test]
fn test_de_bruijn_b2_3() {
    let op = GenerateDeBruijnSequence;
    let result = op
        .run(vec![], &[ArgValue::Num(2.0), ArgValue::Num(3.0)])
        .expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    // B(2,3) = 0001011 (length 2^3 = 8)
    assert_eq!(s.len(), 8);
    // Every 3-bit window should appear once
    let cyclic: String = s.clone() + &s[..2];
    let mut seen = std::collections::HashSet::new();
    for i in 0..8 {
        seen.insert(&cyclic[i..i + 3]);
    }
    assert_eq!(seen.len(), 8);
}
#[test]
fn test_de_bruijn_invalid_k() {
    let op = GenerateDeBruijnSequence;
    let result = op.run(vec![], &[ArgValue::Num(10.0), ArgValue::Num(3.0)]);
    assert!(result.is_err());
}
#[test]
fn test_de_bruijn_invalid_n() {
    let op = GenerateDeBruijnSequence;
    let result = op.run(vec![], &[ArgValue::Num(2.0), ArgValue::Num(1.0)]);
    assert!(result.is_err());
}
#[test]
fn test_de_bruijn_too_large() {
    let op = GenerateDeBruijnSequence;
    // 9^6 = 531441 > 50000
    let result = op.run(vec![], &[ArgValue::Num(9.0), ArgValue::Num(6.0)]);
    assert!(result.is_err());
}
