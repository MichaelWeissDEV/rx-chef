// Tests for the frequency_distribution operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations frequency_distribution::

use rxchef::operation::ArgValue;
use rxchef::operations::frequency_distribution::FrequencyDistribution;
use rxchef::Operation;

#[test]
fn test_frequency_distribution_basic() {
    let op = FrequencyDistribution;
    // Input: "aab" => 'a'=2, 'b'=1
    let input = b"aab".to_vec();
    let args = [ArgValue::Bool(false), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    let lines: Vec<_> = out.lines().collect();
    assert_eq!(lines[0], "Total data length: 3");
    assert_eq!(lines[1], "Number of bytes represented: 2");
    assert_eq!(lines[2], "Number of bytes not represented: 254");
    let a = lines.iter().find(|line| line.starts_with("61 ")).unwrap();
    let b = lines.iter().find(|line| line.starts_with("62 ")).unwrap();
    assert!(a.starts_with("61     66.67%"));
    assert!(b.starts_with("62     33.33%"));
    assert_eq!(a.bytes().filter(|byte| *byte == b'|').count(), 67);
    assert_eq!(b.bytes().filter(|byte| *byte == b'|').count(), 34);
}
#[test]
fn test_frequency_distribution_empty() {
    let op = FrequencyDistribution;
    assert!(op.run(vec![], &[]).is_err());
}
#[test]
fn test_frequency_distribution_show_zeroes() {
    let op = FrequencyDistribution;
    let input = b"a".to_vec();
    let args = [ArgValue::Bool(true), ArgValue::Bool(false)];
    let result = op.run(input, &args).unwrap();
    let out = String::from_utf8(result).unwrap();
    // Should include byte 00 which is zero
    assert!(out.contains("00"));
}
