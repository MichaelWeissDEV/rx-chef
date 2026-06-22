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
    assert!(out.contains("61")); // 0x61 = 'a'
    assert!(out.contains("62")); // 0x62 = 'b'
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
