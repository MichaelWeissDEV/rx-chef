// Tests for the extract_id3 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_id3::

use rxchef::operations::extract_id3::ExtractID3;
use rxchef::Operation;

#[test]
fn test_extract_id3_empty_input() {
    let op = ExtractID3;
    let args = [];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_id3_invalid_mp3() {
    let op = ExtractID3;
    let args = [];
    // Invalid MP3 data
    let invalid_mp3 = vec![0x00, 0x01, 0x02, 0x03];
    let result = op.run(invalid_mp3, &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_id3_minimal_id3_tag() {
    let op = ExtractID3;
    let args = [];
    // Minimal ID3v2 tag (just header)
    let minimal_id3 = vec![
        0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let result = op.run(minimal_id3, &args);
    // This will likely fail but we're testing the parsing attempt
    assert!(result.is_err());
}

#[test]
fn test_extract_id3_large_input() {
    let op = ExtractID3;
    let args = [];
    // Large random data that's not a valid MP3
    let large_data = vec![0xFF; 1024];
    let result = op.run(large_data, &args);
    assert!(result.is_err());
}

#[test]
fn test_extract_id3_truncated_id3() {
    let op = ExtractID3;
    let args = [];
    // Incomplete ID3 tag
    let truncated_id3 = vec![0x49, 0x44, 0x33];
    let result = op.run(truncated_id3, &args);
    assert!(result.is_err());
}
