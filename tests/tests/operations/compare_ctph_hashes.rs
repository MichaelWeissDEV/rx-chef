// Tests for the compare_ctph_hashes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations compare_ctph_hashes::

use rxchef::operation::ArgValue;
use rxchef::operations::compare_ctph_hashes::CompareCTPHHashes;
use rxchef::Operation;

#[test]
fn test_compare_identical_hashes() {
    let op = CompareCTPHHashes;
    // Generate a real ssdeep hash and compare it with itself
    let data = vec![b'A'; 1024];
    let hash = ssdeep::hash(&data).expect("hash should succeed");
    let input = format!("{}\n{}", hash, hash);
    let result = op
        .run(
            input.into_bytes(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .expect("compare should succeed");
    let score: u32 = String::from_utf8(result)
        .expect("valid utf8")
        .parse()
        .expect("valid number");
    assert_eq!(score, 100, "identical hashes should score 100");
}
#[test]
fn test_compare_different_hashes() {
    let op = CompareCTPHHashes;
    let data1 = vec![b'A'; 1024];
    let data2 = vec![b'Z'; 1024];
    let hash1 = ssdeep::hash(&data1).expect("hash should succeed");
    let hash2 = ssdeep::hash(&data2).expect("hash should succeed");
    let input = format!("{}\n{}", hash1, hash2);
    let result = op
        .run(
            input.into_bytes(),
            &[ArgValue::Str("Line feed".to_string())],
        )
        .expect("compare should succeed");
    let score: u32 = String::from_utf8(result)
        .expect("valid utf8")
        .parse()
        .expect("valid number");
    assert!(score <= 100);
}
#[test]
fn test_compare_wrong_delimiter_count() {
    let op = CompareCTPHHashes;
    let input = b"only_one_hash".to_vec();
    let result = op.run(input, &[ArgValue::Str("Line feed".to_string())]);
    assert!(result.is_err());
}
