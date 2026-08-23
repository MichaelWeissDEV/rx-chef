// Tests for the compare_ctph_hashes operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations compare_ctph_hashes::

use rxchef::operation::ArgValue;
use rxchef::operations::compare_ctph_hashes::CompareCTPHHashes;
use rxchef::operations::ctph::CTPH;
use rxchef::Operation;

#[test]
fn test_compare_identical_hashes() {
    let op = CompareCTPHHashes;
    // The CTPH comparison contract defines identity as similarity 100.
    let data = vec![b'A'; 1024];
    let hash = String::from_utf8(CTPH.run(data, &[]).unwrap()).unwrap();
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
    let hash1 = String::from_utf8(CTPH.run(data1, &[]).unwrap()).unwrap();
    let hash2 = String::from_utf8(CTPH.run(data2, &[]).unwrap()).unwrap();
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
