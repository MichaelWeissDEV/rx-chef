// Tests for the remove_exif operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations remove_exif::

use rxchef::operations::remove_exif::RemoveEXIF;
use rxchef::Operation;

#[test]
fn test_remove_exif_no_exif() {
    let op = RemoveEXIF;
    let input = vec![
        0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0, 1, 1, 1, 0, 0x48, 0, 0x48,
        0, 0,
    ];
    let result = op.run(input.clone(), &[]).unwrap();
    assert_eq!(result, input);
}
#[test]
fn test_not_jpeg() {
    let op = RemoveEXIF;
    let input = vec![1, 2, 3, 4];
    let result = op.run(input, &[]);
    assert!(result.is_err());
}
