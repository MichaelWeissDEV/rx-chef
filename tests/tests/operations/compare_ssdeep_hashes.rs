// Tests for the compare_ssdeep_hashes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations compare_ssdeep_hashes::

use rxchef::operation::ArgValue;
use rxchef::operations::compare_ssdeep_hashes::CompareSSDEEPHashes;
use rxchef::Operation;

#[test]
fn test_compare_ssdeep_hashes() {
    let op = CompareSSDEEPHashes;
    let input = b"3:Hn:Hn\n3:Hn:Hn".to_vec();
    let args = vec![ArgValue::Str("Line feed".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "100");
}
