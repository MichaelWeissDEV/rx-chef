// Tests for the entropy operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations entropy::

use rxchef::operation::ArgValue;
use rxchef::operations::entropy::Entropy;
use rxchef::Operation;

fn run_op(input: &[u8], chunk_size: usize) -> String {
    let op = Entropy;
    let args = [ArgValue::Num(chunk_size as f64)];
    let result = op.run(input.to_vec(), &args).unwrap();
    String::from_utf8(result).unwrap()
}
#[test]
fn test_entropy_empty_input() {
    let result = run_op(b"", 0);
    let val: f64 = result.trim().parse().unwrap();
    assert!((val - 0.0).abs() < 1e-10);
}
#[test]
fn test_entropy_all_same_bytes() {
    let input = vec![0xAA_u8; 100];
    let result = run_op(&input, 0);
    let val: f64 = result.trim().parse().unwrap();
    assert!((val - 0.0).abs() < 1e-10);
}
#[test]
fn test_entropy_all_distinct_bytes() {
    // All 256 distinct byte values -> entropy = 8.0
    let input: Vec<u8> = (0..=255).collect();
    let result = run_op(&input, 0);
    let val: f64 = result.trim().parse().unwrap();
    assert!((val - 8.0).abs() < 1e-10, "Expected 8.0, got {}", val);
}
#[test]
fn test_entropy_chunked_same_bytes() {
    let input = vec![0x00_u8; 8];
    let result = run_op(&input, 4);
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
    for line in lines {
        let val: f64 = line.trim().parse().unwrap();
        assert!((val - 0.0).abs() < 1e-10);
    }
}
#[test]
fn test_entropy_english_text_range() {
    let input = b"The quick brown fox jumps over the lazy dog";
    let result = run_op(input, 0);
    let val: f64 = result.trim().parse().unwrap();
    assert!(
        val >= 3.5 && val <= 5.0,
        "English text entropy {} out of expected range",
        val
    );
}
