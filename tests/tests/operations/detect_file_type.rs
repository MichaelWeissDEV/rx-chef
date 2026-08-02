// Tests for the detect_file_type operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations detect_file_type::

use rxchef::operations::detect_file_type::DetectFileType;
use rxchef::Operation;

fn run(input: &[u8], args: &[rxchef::operation::ArgValue]) -> Vec<u8> {
    let op = DetectFileType;
    op.run(input.to_vec(), args).unwrap()
}

#[test]
fn test_detect_file_type_empty_input() {
    let op = DetectFileType;
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "Unknown file type.".as_bytes());
}

#[test]
fn test_detect_file_type_jpeg() {
    // Test JPEG detection
    let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&jpeg_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Joint Photographic Experts Group image"));
    assert!(result_str.contains("jpg"));
    assert!(result_str.contains("image/jpeg"));
}

#[test]
fn test_detect_file_type_png() {
    // Test PNG detection
    let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&png_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Portable Network Graphics image"));
    assert!(result_str.contains("png"));
    assert!(result_str.contains("image/png"));
}

#[test]
fn test_detect_file_type_gif() {
    // Test GIF detection
    let gif_header = vec![0x47, 0x49, 0x46, 0x38];
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&gif_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Graphics Interchange Format image"));
    assert!(result_str.contains("gif"));
    assert!(result_str.contains("image/gif"));
}

#[test]
fn test_detect_file_type_mp4() {
    // Test MP4 detection
    let mut mp4_header = vec![0u8; 8];
    mp4_header[4..8].copy_from_slice(&[0x66, 0x74, 0x79, 0x70]); // "ftyp"
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&mp4_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("MPEG-4 video"));
    assert!(result_str.contains("mp4"));
    assert!(result_str.contains("video/mp4"));
}

#[test]
fn test_detect_file_type_flv() {
    // Test FLV detection
    let flv_header = vec![0x46, 0x4C, 0x56, 0x01];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&flv_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Flash Video"));
    assert!(result_str.contains("flv"));
    assert!(result_str.contains("video/x-flv"));
}

#[test]
fn test_detect_file_type_wav() {
    // Test WAV detection
    let mut wav_header = vec![0u8; 12];
    wav_header[0..4].copy_from_slice(&[0x52, 0x49, 0x46, 0x46]); // "RIFF"
    wav_header[8..12].copy_from_slice(&[0x57, 0x41, 0x56, 0x45]); // "WAVE"
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&wav_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Waveform Audio"));
    assert!(result_str.contains("wav"));
    assert!(result_str.contains("audio/x-wav"));
}

#[test]
fn test_detect_file_type_mp3() {
    // Test MP3 detection
    let mp3_header = vec![0x49, 0x44, 0x33];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&mp3_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("MPEG-3 audio"));
    assert!(result_str.contains("mp3"));
    assert!(result_str.contains("audio/mpeg"));
}

#[test]
fn test_detect_file_type_pdf() {
    // Test PDF detection
    let pdf_header = vec![0x25, 0x50, 0x44, 0x46];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&pdf_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Portable Document Format"));
    assert!(result_str.contains("pdf"));
    assert!(result_str.contains("application/pdf"));
}

#[test]
fn test_detect_file_type_rtf() {
    // Test RTF detection
    let rtf_header = vec![0x7B, 0x5C, 0x72, 0x74];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&rtf_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Rich Text Format"));
    assert!(result_str.contains("rtf"));
    assert!(result_str.contains("application/rtf"));
}

#[test]
fn test_detect_file_type_exe() {
    // Test EXE detection
    let exe_header = vec![0x4D, 0x5A]; // "MZ"
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&exe_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Windows Portable Executable"));
    assert!(result_str.contains("exe"));
    assert!(result_str.contains("application/vnd.microsoft.portable-executable"));
}

#[test]
fn test_detect_file_type_elf() {
    // Test ELF detection
    let elf_header = vec![0x7F, 0x45, 0x4C, 0x46]; // "\x7FELF"
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&elf_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Executable and Linkable Format"));
    assert!(result_str.contains("elf"));
    assert!(result_str.contains("application/x-executable"));
}

#[test]
fn test_detect_file_type_zip() {
    // Test ZIP detection
    let zip_header = vec![0x50, 0x4B, 0x03, 0x04];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = run(&zip_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("PKZIP archive"));
    assert!(result_str.contains("zip"));
    assert!(result_str.contains("application/zip"));
}

#[test]
fn test_detect_file_type_gzip() {
    // Test GZIP detection
    let gzip_header = vec![0x1F, 0x8B, 0x08];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = run(&gzip_header, &args);
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("Gzip"));
    assert!(result_str.contains("gz"));
    assert!(result_str.contains("application/gzip"));
}

#[test]
fn test_detect_file_type_unknown() {
    // Test unknown file type
    let unknown_data = vec![0x00, 0x01, 0x02, 0x03];
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = run(&unknown_data, &args);
    assert_eq!(result, "Unknown file type.".as_bytes());
}

#[test]
fn test_detect_file_type_multiple_matches() {
    // Test when input matches multiple signatures (should show all)
    // This is unlikely in practice but possible
    let op = DetectFileType;
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    
    // Test with ZIP header (should only match ZIP)
    let zip_header = vec![0x50, 0x4B, 0x03, 0x04];
    let result = op.run(zip_header, &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("PKZIP archive"));
    // Should not contain other file types
    assert!(!result_str.contains("Portable Document Format"));
}

#[test]
fn test_detect_file_type_partial_match() {
    // Test when input partially matches a signature (should still match if it matches any signature)
    let partial_jpeg = vec![0xFF, 0xD8, 0xFF, 0x00]; // Wrong third byte but still matches JPEG start
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&partial_jpeg, &args);
    let result_str = String::from_utf8(result).unwrap();
    // Should still detect as JPEG since it matches the JPEG signature pattern
    assert!(result_str.contains("Joint Photographic Experts Group image"));
}

#[test]
fn test_detect_file_type_disabled_categories() {
    // Test when all categories are disabled
    let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
    let args = [
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Bool(false),
    ];
    let result = run(&jpeg_header, &args);
    assert_eq!(result, "Unknown file type.".as_bytes());
}

#[test]
fn test_detect_file_type_short_input() {
    // Test with input shorter than any signature
    let short_input = vec![0xFF];
    let args = [
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = run(&short_input, &args);
    assert_eq!(result, "Unknown file type.".as_bytes());
}
