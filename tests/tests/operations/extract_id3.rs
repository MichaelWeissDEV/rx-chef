// Tests for the extract_id3 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_id3::

use rxchef::operations::extract_id3::ExtractID3;
use rxchef::Operation;

fn tagged_mp3() -> Vec<u8> {
    // ID3v2.3 TIT2("Title") followed by one MPEG-1 Layer III, 128 kb/s,
    // 44.1 kHz frame (417 bytes including its four-byte header).
    let mut bytes = vec![
        b'I', b'D', b'3', 3, 0, 0, 0, 0, 0, 16, b'T', b'I', b'T', b'2', 0, 0, 0, 6, 0, 0, 0,
        b'T', b'i', b't', b'l', b'e', 0xff, 0xfb, 0x90, 0x64,
    ];
    bytes.resize(10 + 16 + 417, 0);
    bytes.extend_from_slice(&[0xff, 0xfb, 0x90, 0x64]);
    bytes.resize(10 + 16 + 2 * 417, 0);
    bytes
}

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
    let minimal_id3 = vec![0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
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

#[test]
fn test_extract_id3_reads_v23_title_frame() {
    let result = ExtractID3.run(tagged_mp3(), &[]).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(
        value,
        serde_json::json!({
            "Type": "ID3",
            "Tags": {"TrackTitle": {"Description": "TrackTitle", "Data": "Title"}}
        })
    );
}
