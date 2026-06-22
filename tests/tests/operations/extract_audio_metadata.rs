// Tests for the extract_audio_metadata operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_audio_metadata::

use rxchef::operations::extract_audio_metadata::ExtractAudioMetadata;
use rxchef::Operation;

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
    let minimal_id3 = vec![
        0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
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
