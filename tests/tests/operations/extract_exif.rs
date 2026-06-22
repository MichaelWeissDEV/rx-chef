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
