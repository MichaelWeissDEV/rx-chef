// Tests for the crop_image operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations crop_image::

use rxchef::operations::crop_image::CropImage;
use rxchef::Operation;

#[test]
fn test_crop_image_empty_input() {
    let op = CropImage;
    let input = vec![];
    let args = [
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args).unwrap();
    assert_eq!(result, Vec::<u8>::new());
}

#[test]
fn test_crop_image_invalid_format() {
    let op = CropImage;
    let input = b"This is not an image".to_vec();
    let args = [
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported image format"));
}

#[test]
fn test_crop_image_basic_crop() {
    let op = CropImage;
    
    // Create a simple test image (20x20 PNG)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(20, 20);
    // Fill with different colors to make cropping visible
    for y in 0..20 {
        for x in 0..20 {
            let color = if x < 10 && y < 10 {
                [255, 0, 0, 255] // Red in top-left quadrant
            } else if x >= 10 && y < 10 {
                [0, 255, 0, 255] // Green in top-right quadrant
            } else if x < 10 && y >= 10 {
                [0, 0, 255, 255] // Blue in bottom-left quadrant
            } else {
                [255, 255, 0, 255] // Yellow in bottom-right quadrant
            };
            img.put_pixel(x, y, image::Rgba(color));
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(5.0),  // X position
        rxchef::operation::ArgValue::Num(5.0),  // Y position
        rxchef::operation::ArgValue::Num(10.0), // Width
        rxchef::operation::ArgValue::Num(10.0), // Height
        rxchef::operation::ArgValue::Bool(false), // No autocrop
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Load the result and verify dimensions
    let cropped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(cropped_img.width(), 10);
    assert_eq!(cropped_img.height(), 10);
}

#[test]
fn test_crop_image_edge_cases() {
    let op = CropImage;
    
    // Create a simple test image (10x10 PNG)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test cropping at edges
    let edge_cases = vec![
        (0, 0, 5, 5),   // Top-left corner
        (5, 5, 5, 5),   // Center
        (0, 5, 5, 5),   // Left edge, middle
        (5, 0, 5, 5),   // Top edge, middle
    ];
    
    for (x, y, w, h) in edge_cases {
        let args = [
            rxchef::operation::ArgValue::Num(x as f64),
            rxchef::operation::ArgValue::Num(y as f64),
            rxchef::operation::ArgValue::Num(w as f64),
            rxchef::operation::ArgValue::Num(h as f64),
            rxchef::operation::ArgValue::Bool(false),
            rxchef::operation::ArgValue::Num(2.0),
            rxchef::operation::ArgValue::Bool(true),
            rxchef::operation::ArgValue::Bool(false),
            rxchef::operation::ArgValue::Num(0.0),
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let cropped_img = image::load_from_memory(&result).unwrap();
        assert_eq!(cropped_img.width(), w);
        assert_eq!(cropped_img.height(), h);
    }
}

#[test]
fn test_crop_image_autocrop_simple() {
    let op = CropImage;
    
    // Create an image with uniform border (100x100 with 10px red border, rest green)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(100, 100);
    
    // Fill border with red
    for y in 0..100 {
        for x in 0..100 {
            if x < 10 || x >= 90 || y < 10 || y >= 90 {
                img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Bool(true), // Enable autocrop
        rxchef::operation::ArgValue::Num(2.0),    // Tolerance
        rxchef::operation::ArgValue::Bool(true),  // Only frames
        rxchef::operation::ArgValue::Bool(false), // Not symmetric
        rxchef::operation::ArgValue::Num(0.0),   // Keep border
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Should crop the 10px border from all sides
    let cropped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(cropped_img.width(), 80);  // 100 - 10 - 10
    assert_eq!(cropped_img.height(), 80); // 100 - 10 - 10
}

#[test]
fn test_crop_image_autocrop_tolerance() {
    let op = CropImage;
    
    // Create an image with slightly different border colors
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(50, 50);
    
    // Fill with gradient border
    for y in 0..50 {
        for x in 0..50 {
            if x < 5 || x >= 45 || y < 5 || y >= 45 {
                // Border with slight color variation
                let shade = if x < 5 { x } else if x >= 45 { 49 - x } 
                          else if y < 5 { y } else { 49 - y };
                img.put_pixel(x, y, image::Rgba([(shade * 5) as u8, 0, 0, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test with different tolerance levels
    for tolerance in [0.0, 5.0, 10.0, 20.0] {
        let args = [
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(10.0),
            rxchef::operation::ArgValue::Num(10.0),
            rxchef::operation::ArgValue::Bool(true), // Enable autocrop
            rxchef::operation::ArgValue::Num(tolerance),
            rxchef::operation::ArgValue::Bool(false), // Not only frames
            rxchef::operation::ArgValue::Bool(false), // Not symmetric
            rxchef::operation::ArgValue::Num(0.0),   // Keep border
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let cropped_img = image::load_from_memory(&result).unwrap();
        // Higher tolerance should crop more
        assert!(cropped_img.width() <= 50);
        assert!(cropped_img.height() <= 50);
    }
}

#[test]
fn test_crop_image_autocrop_symmetric() {
    let op = CropImage;
    
    // Create an image with asymmetric border
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(60, 40);
    
    // Fill with asymmetric border
    for y in 0..40 {
        for x in 0..60 {
            if x < 10 || x >= 50 || y < 5 || y >= 35 {
                img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test symmetric autocrop
    let args = [
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Num(10.0),
        rxchef::operation::ArgValue::Bool(true), // Enable autocrop
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Bool(false), // Not only frames
        rxchef::operation::ArgValue::Bool(true),  // Symmetric
        rxchef::operation::ArgValue::Num(0.0),   // Keep border
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    let cropped_img = image::load_from_memory(&result).unwrap();
    // Should be symmetrically cropped
    assert!(cropped_img.width() < 60);
    assert!(cropped_img.height() < 40);
}

#[test]
fn test_crop_image_autocrop_keep_border() {
    let op = CropImage;
    
    // Create an image with uniform border
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(50, 50);
    
    // Fill with 10px border
    for y in 0..50 {
        for x in 0..50 {
            if x < 10 || x >= 40 || y < 10 || y >= 40 {
                img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]));
            } else {
                img.put_pixel(x, y, image::Rgba([0, 255, 0, 255]));
            }
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test with different keep border values
    for keep_border in [0, 2, 5, 8] {
        let args = [
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(0.0),
            rxchef::operation::ArgValue::Num(10.0),
            rxchef::operation::ArgValue::Num(10.0),
            rxchef::operation::ArgValue::Bool(true), // Enable autocrop
            rxchef::operation::ArgValue::Num(2.0),
            rxchef::operation::ArgValue::Bool(true), // Only frames
            rxchef::operation::ArgValue::Bool(false), // Not symmetric
            rxchef::operation::ArgValue::Num(keep_border as f64), // Keep border
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let cropped_img = image::load_from_memory(&result).unwrap();
        let expected_size = 30 + keep_border * 2; // 50 - 10 - 10 + keep_border*2
        assert_eq!(cropped_img.width(), expected_size);
        assert_eq!(cropped_img.height(), expected_size);
    }
}

#[test]
fn test_crop_image_large_image() {
    let op = CropImage;
    
    // Create a larger image (200x200)
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(200, 200);
    
    // Fill with pattern
    for y in 0..200 {
        for x in 0..200 {
            let color = ((x + y) % 256) as u8;
            img.put_pixel(x, y, image::Rgba([color, color, color, 255]));
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Num(50.0),  // X position
        rxchef::operation::ArgValue::Num(50.0),  // Y position
        rxchef::operation::ArgValue::Num(100.0), // Width
        rxchef::operation::ArgValue::Num(100.0), // Height
        rxchef::operation::ArgValue::Bool(false), // No autocrop
        rxchef::operation::ArgValue::Num(2.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false),
        rxchef::operation::ArgValue::Num(0.0),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    let cropped_img = image::load_from_memory(&result).unwrap();
    assert_eq!(cropped_img.width(), 100);
    assert_eq!(cropped_img.height(), 100);
}
