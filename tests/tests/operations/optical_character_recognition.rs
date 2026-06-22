// Tests for the optical_character_recognition operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations optical_character_recognition::

use rxchef::operations::optical_character_recognition::OpticalCharacterRecognition;
use rxchef::Operation;

#[test]
fn test_ocr_is_broken() {
    let op = OpticalCharacterRecognition;
    #[cfg(not(feature = "tesseract"))]
    assert!(op.is_broken());
    #[cfg(feature = "tesseract")]
    assert!(!op.is_broken());
}
#[test]
fn test_ocr_without_feature() {
    let op = OpticalCharacterRecognition;
    let input = vec![0u8];
    let args = vec![];
    let result = op.run(input, &args);
    #[cfg(not(feature = "tesseract"))]
    assert!(result.is_err());
}
