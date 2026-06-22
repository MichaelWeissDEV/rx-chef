// Tests for the get_all_casings operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations get_all_casings::

use rxchef::operations::get_all_casings::GetAllCasings;
use rxchef::Operation;

#[test]
fn test_all_casings_ab() {
    let op = GetAllCasings;
    let result = op.run(b"ab".to_vec(), &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 4);
    assert!(lines.contains(&"ab"));
    assert!(lines.contains(&"Ab"));
    assert!(lines.contains(&"aB"));
    assert!(lines.contains(&"AB"));
}
#[test]
fn test_all_casings_single() {
    let op = GetAllCasings;
    let result = op.run(b"a".to_vec(), &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 2);
}
#[test]
fn test_all_casings_non_alpha_preserved() {
    let op = GetAllCasings;
    let result = op.run(b"a1b".to_vec(), &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    assert!(s.contains("a1b"));
    assert!(s.contains("A1B"));
}
#[test]
fn test_all_casings_too_long() {
    let op = GetAllCasings;
    let long_input = b"abcdefghijklmnopqrstu".to_vec(); // 21 alpha chars
    let result = op.run(long_input, &[]);
    assert!(result.is_err());
}
