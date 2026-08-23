// Tests for the extract_audio_metadata operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_audio_metadata::

use rxchef::operations::extract_audio_metadata::ExtractAudioMetadata;
use rxchef::Operation;

fn tagged_mp3() -> Vec<u8> {
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
fn test_extract_audio_metadata_empty_input() {
    let op = ExtractAudioMetadata;
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Num(524288.0),
    ];
    let result = op.run(vec![], &args);
    // Empty input should fail during audio file parsing
    assert!(result.is_err());
}

#[test]
fn test_extract_audio_metadata_invalid_audio() {
    let op = ExtractAudioMetadata;
    let args = [
        rxchef::operation::ArgValue::Str("test.mp3".to_string()),
        rxchef::operation::ArgValue::Num(524288.0),
    ];
    // Invalid audio data
    let invalid_audio = vec![0x00, 0x01, 0x02, 0x03];
    let result = op.run(invalid_audio, &args);
    // Should fail to parse as audio file
    assert!(result.is_err());
}

#[test]
fn test_extract_audio_metadata_with_filename() {
    let op = ExtractAudioMetadata;
    let args = [
        rxchef::operation::ArgValue::Str("test.mp3".to_string()),
        rxchef::operation::ArgValue::Num(524288.0),
    ];
    // Minimal ID3 tag (just header)
    let minimal_id3 = vec![0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let result = op.run(minimal_id3, &args);
    // This will likely fail but we're testing the argument handling
    assert!(result.is_err());
}

#[test]
fn test_extract_audio_metadata_custom_max_bytes() {
    let op = ExtractAudioMetadata;
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Num(1024.0), // Small max bytes
    ];
    let result = op.run(vec![0x00], &args);
    // Should still fail due to invalid audio format
    assert!(result.is_err());
}

#[test]
fn test_extract_audio_metadata_large_input() {
    let op = ExtractAudioMetadata;
    let args = [
        rxchef::operation::ArgValue::Str("".to_string()),
        rxchef::operation::ArgValue::Num(524288.0),
    ];
    // Large random data that's not a valid audio file
    let large_data = vec![0xFF; 1024];
    let result = op.run(large_data, &args);
    // Should fail to parse as audio file
    assert!(result.is_err());
}

#[test]
fn test_extract_audio_metadata_reads_standard_id3_title() {
    let bytes = tagged_mp3();
    let result = ExtractAudioMetadata
        .run(
            bytes.clone(),
            &[
                rxchef::operation::ArgValue::Str("song.mp3".to_string()),
                rxchef::operation::ArgValue::Num(524288.0),
            ],
        )
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&result).unwrap();
    assert_eq!(
        value,
        serde_json::json!({
            "artifact": {"filename": "song.mp3", "byte_length": bytes.len()},
            "tags": {"common": {"title": "Title"}}
        })
    );
}
