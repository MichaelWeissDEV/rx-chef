// Tests for the flip_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations flip_image::

use rxchef::operations::flip_image::FlipImage;
use rxchef::Operation;

#[test]
fn test_flip_image_empty_input() {
    let op = FlipImage;
    let input = vec![];
    let args = [rxchef::operation::ArgValue::Str("Horizontal".to_string())];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_flip_image_invalid_format() {
    let op = FlipImage;
    let input = b"This is not an image".to_vec();
    let args = [rxchef::operation::ArgValue::Str("Horizontal".to_string())];
    let result = op.run(input, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported image format"));
}

#[test]
fn test_flip_image_horizontal() {
    let op = FlipImage;
    
    // Create a simple test image (2x2 PNG with different colors)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(2, 2);
    img.put_pixel(0, 0, image::Rgba([255, 0, 0, 255])); // Red
    img.put_pixel(1, 0, image::Rgba([0, 255, 0, 255])); // Green
    img.put_pixel(0, 1, image::Rgba([0, 0, 255, 255])); // Blue
    img.put_pixel(1, 1, image::Rgba([255, 255, 0, 255])); // Yellow
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [rxchef::operation::ArgValue::Str("Horizontal".to_string())];
    let result = op.run(img_buf, &args).unwrap();
    
    // Should be valid image data
    assert!(!result.is_empty());
    
    // Load the result and verify it's a valid image
    let flipped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(flipped_img.width(), 2);
    assert_eq!(flipped_img.height(), 2);
}

#[test]
fn test_flip_image_vertical() {
    let op = FlipImage;
    
    // Create a simple test image (2x2 PNG with different colors)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(2, 2);
    img.put_pixel(0, 0, image::Rgba([255, 0, 0, 255])); // Red
    img.put_pixel(1, 0, image::Rgba([0, 255, 0, 255])); // Green
    img.put_pixel(0, 1, image::Rgba([0, 0, 255, 255])); // Blue
    img.put_pixel(1, 1, image::Rgba([255, 255, 0, 255])); // Yellow
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [rxchef::operation::ArgValue::Str("Vertical".to_string())];
    let result = op.run(img_buf, &args).unwrap();
    
    // Should be valid image data
    assert!(!result.is_empty());
    
    // Load the result and verify it's a valid image
    let flipped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(flipped_img.width(), 2);
    assert_eq!(flipped_img.height(), 2);
}

#[test]
fn test_flip_image_invalid_axis() {
    let op = FlipImage;
    
    // Create a simple test image
    let mut img_buf = Vec::new();
    let img = image::RgbaImage::new(1, 1);
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [rxchef::operation::ArgValue::Str("InvalidAxis".to_string())];
    let result = op.run(img_buf, &args);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid axis"));
}

#[test]
fn test_flip_image_default_axis() {
    let op = FlipImage;
    
    // Create a simple test image
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(2, 2);
    img.put_pixel(0, 0, image::Rgba([255, 0, 0, 255]));
    img.put_pixel(1, 0, image::Rgba([0, 255, 0, 255]));
    img.put_pixel(0, 1, image::Rgba([0, 0, 255, 255]));
    img.put_pixel(1, 1, image::Rgba([255, 255, 0, 255]));
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // No args provided, should default to Horizontal
    let result = op.run(img_buf, &[]).unwrap();
    
    // Should be valid image data
    assert!(!result.is_empty());
    
    // Load the result and verify it's a valid image
    let flipped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(flipped_img.width(), 2);
    assert_eq!(flipped_img.height(), 2);
}

#[test]
fn test_flip_image_large_image() {
    let op = FlipImage;
    
    // Create a larger test image (100x100)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(100, 100);
    
    // Fill with a gradient pattern
    for y in 0..100 {
        for x in 0..100 {
            let color = ((x + y) % 256) as u8;
            img.put_pixel(x, y, image::Rgba([color, color, color, 255]));
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [rxchef::operation::ArgValue::Str("Horizontal".to_string())];
    let result = op.run(img_buf, &args).unwrap();
    
    // Should be valid image data
    assert!(!result.is_empty());
    
    // Load the result and verify dimensions
    let flipped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(flipped_img.width(), 100);
    assert_eq!(flipped_img.height(), 100);
}
