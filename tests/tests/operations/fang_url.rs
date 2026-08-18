// Tests for the fang_url operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations fang_url::

use rxchef::operations::fang_url::FangURL;
use rxchef::Operation;

#[test]
fn test_fang_url_basic() {
    let operation = FangURL;
    let input = b"https://example[.]com".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "https://example.com");
}
#[test]
fn test_fang_url_hxxp() {
    let operation = FangURL;
    let input = b"hxxp://example.com".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "http://example.com");
}
#[test]
fn test_fang_url_slashes() {
    let operation = FangURL;
    let input = b"http[://]example.com".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "http://example.com");
}
#[test]
fn test_fang_url_combined() {
    let operation = FangURL;
    let input = b"hxxp://example[.]com[://]path".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert_eq!(output, "http://example.com://path");
}
