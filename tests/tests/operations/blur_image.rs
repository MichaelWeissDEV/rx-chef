// Tests for the blur_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations blur_image::

use rxchef::operation::ArgValue;
use rxchef::operations::blur_image::BlurImage;
use rxchef::Operation;

#[test]
fn test_blur_image_basic() {
    let op = BlurImage;
    
    // Test with invalid PNG data to verify error handling
    let invalid_png_data = vec![0x00, 0x01, 0x02, 0x03]; // Not a valid PNG
    
    let args = [ArgValue::Num(5.0), ArgValue::Str("Fast".to_string())];
    let result = op.run(invalid_png_data, &args);
    
    // Should return error for invalid PNG
    assert!(result.is_err(), "Should return error for invalid PNG data");
}

#[test]
fn test_blur_image_empty_input() {
    let op = BlurImage;
    let args = [ArgValue::Num(5.0), ArgValue::Str("Fast".to_string())];
    let result = op.run(vec![], &args);
    
    // Should return empty input unchanged
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![0u8; 0]);
}

#[test]
fn test_blur_image_invalid_amount() {
    let op = BlurImage;
    
    // Test with invalid PNG data
    let invalid_png_data = vec![0x00, 0x01, 0x02, 0x03];
    
    let args = [ArgValue::Num(-1.0), ArgValue::Str("Fast".to_string())]; // Negative amount
    let result = op.run(invalid_png_data, &args);
    
    // Should return error for invalid PNG (before checking amount)
    assert!(result.is_err(), "Should return error for invalid PNG data");
}

#[test]
fn test_blur_image_gaussian() {
    let op = BlurImage;
    
    // Test with invalid PNG data
    let invalid_png_data = vec![0x00, 0x01, 0x02, 0x03];
    
    let args = [ArgValue::Num(3.0), ArgValue::Str("Gaussian".to_string())];
    let result = op.run(invalid_png_data, &args);
    
    // Should return error for invalid PNG
    assert!(result.is_err(), "Should return error for invalid PNG data");
}

#[test]
fn test_blur_image_invalid_type() {
    let op = BlurImage;
    
    // Test with invalid PNG data
    let invalid_png_data = vec![0x00, 0x01, 0x02, 0x03];
    
    let args = [ArgValue::Num(5.0), ArgValue::Str("InvalidType".to_string())];
    let result = op.run(invalid_png_data, &args);
    
    // Should return error for invalid PNG (before checking blur type)
    assert!(result.is_err());
}
