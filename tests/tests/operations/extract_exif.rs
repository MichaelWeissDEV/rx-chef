// Tests for the extract_exif operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations extract_exif::

use rxchef::operations::extract_exif::ExtractEXIF;
use rxchef::Operation;

#[test]
fn test_extract_exif_invalid() {
    let op = ExtractEXIF;
    let input = vec![0, 1, 2, 3];
    let result = op.run(input, &[]);
    assert!(result.is_err());
}

#[test]
fn test_extract_exif_minimal_little_endian_tiff_make_tag() {
    // TIFF 6.0: little-endian header followed by one IFD0 ASCII Make entry.
    let tiff = vec![
        b'I', b'I', 0x2a, 0x00, 0x08, 0x00, 0x00, 0x00, // header + IFD offset
        0x01, 0x00, // one entry
        0x0f, 0x01, 0x02, 0x00, // Make, ASCII
        0x06, 0x00, 0x00, 0x00, // six bytes
        0x1a, 0x00, 0x00, 0x00, // value at offset 26
        0x00, 0x00, 0x00, 0x00, // no next IFD
        b'C', b'a', b'n', b'o', b'n', 0x00,
    ];
    assert_eq!(
        ExtractEXIF.run(tiff, &[]).unwrap(),
        b"Found 1 tags.\n\nMake: \"Canon\""
    );
}
