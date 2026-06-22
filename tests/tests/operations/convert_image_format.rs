// Tests for the convert_image_format operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations convert_image_format::

use rxchef::operations::convert_image_format::ConvertImageFormat;
use rxchef::Operation;

#[test]
fn test_convert_image_format_empty_input() {
    let op = ConvertImageFormat;
    let input = vec![];
    let args = [
        rxchef::operation::ArgValue::Str("JPEG".to_string()),
        rxchef::operation::ArgValue::Num(80.0),
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err()); // Empty input should fail
}

#[test]
fn test_convert_image_format_invalid_format() {
    let op = ConvertImageFormat;
    let input = b"This is not an image".to_vec();
    let args = [
        rxchef::operation::ArgValue::Str("JPEG".to_string()),
        rxchef::operation::ArgValue::Num(80.0),
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    let result = op.run(input, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Error loading image"));
}

#[test]
fn test_convert_image_format_to_jpeg() {
    let op = ConvertImageFormat;
    
    // Create a simple PNG image (use RGB for JPEG compatibility)
    let mut img_buf = Vec::new();
    let mut img = image::RgbImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([255, 0, 0]); // Red
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Str("JPEG".to_string()),
        rxchef::operation::ArgValue::Num(80.0), // JPEG quality
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Verify it's a valid JPEG
    let converted_img = image::load_from_memory(&result).unwrap();
    assert_eq!(converted_img.width(), 10);
    assert_eq!(converted_img.height(), 10);
}

#[test]
fn test_convert_image_format_to_png() {
    let op = ConvertImageFormat;
    
    // Create a simple JPEG image (use RGB for JPEG compatibility)
    let mut img_buf = Vec::new();
    let mut img = image::RgbImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([0, 255, 0]); // Green
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Jpeg).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Str("PNG".to_string()),
        rxchef::operation::ArgValue::Num(80.0),
        rxchef::operation::ArgValue::Str("Auto".to_string()), // PNG filter
        rxchef::operation::ArgValue::Num(9.0), // PNG deflate level
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Verify it's a valid PNG
    let converted_img = image::load_from_memory(&result).unwrap();
    assert_eq!(converted_img.width(), 10);
    assert_eq!(converted_img.height(), 10);
}

#[test]
fn test_convert_image_format_to_bmp() {
    let op = ConvertImageFormat;
    
    // Create a simple PNG image (use RGB for JPEG compatibility)
    let mut img_buf = Vec::new();
    let mut img = image::RgbImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([0, 0, 255]); // Blue
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Str("BMP".to_string()),
        rxchef::operation::ArgValue::Num(80.0),
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Verify it's a valid BMP
    let converted_img = image::load_from_memory(&result).unwrap();
    assert_eq!(converted_img.width(), 10);
    assert_eq!(converted_img.height(), 10);
}

#[test]
fn test_convert_image_format_to_tiff() {
    let op = ConvertImageFormat;
    
    // Create a simple PNG image
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 255, 0, 255]); // Yellow
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Str("TIFF".to_string()),
        rxchef::operation::ArgValue::Num(80.0),
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Verify it's a valid TIFF
    let converted_img = image::load_from_memory(&result).unwrap();
    assert_eq!(converted_img.width(), 10);
    assert_eq!(converted_img.height(), 10);
}

#[test]
fn test_convert_image_format_jpeg_quality() {
    let op = ConvertImageFormat;
    
    // Create a simple PNG image (use RGB for JPEG compatibility)
    let mut img_buf = Vec::new();
    let mut img = image::RgbImage::new(20, 20);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([128, 128, 128]); // Gray
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test different JPEG qualities
    for quality in [10, 50, 80, 95] {
        let args = [
            rxchef::operation::ArgValue::Str("JPEG".to_string()),
            rxchef::operation::ArgValue::Num(quality as f64), // Different quality
            rxchef::operation::ArgValue::Str("Auto".to_string()),
            rxchef::operation::ArgValue::Num(9.0),
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let converted_img = image::load_from_memory(&result).unwrap();
        assert_eq!(converted_img.width(), 20);
        assert_eq!(converted_img.height(), 20);
    }
}

#[test]
fn test_convert_image_format_png_options() {
    let op = ConvertImageFormat;
    
    // Create a simple PNG image
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 255, 255]); // Magenta
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    // Test different PNG filter types
    let filter_types = ["None", "Sub", "Up", "Average", "Paeth", "Auto"];
    
    for filter_type in filter_types {
        let args = [
            rxchef::operation::ArgValue::Str("PNG".to_string()),
            rxchef::operation::ArgValue::Num(80.0),
            rxchef::operation::ArgValue::Str(filter_type.to_string()), // Different filter
            rxchef::operation::ArgValue::Num(9.0),
        ];
        
        let result = op.run(img_buf.clone(), &args).unwrap();
        assert!(!result.is_empty());
        
        let converted_img = image::load_from_memory(&result).unwrap();
        assert_eq!(converted_img.width(), 10);
        assert_eq!(converted_img.height(), 10);
    }
}

#[test]
fn test_convert_image_format_invalid_output_format() {
    let op = ConvertImageFormat;
    
    // Create a simple PNG image
    let mut img_buf = Vec::new();
    let mut img = image::RgbaImage::new(10, 10);
    for pixel in img.pixels_mut() {
        *pixel = image::Rgba([255, 0, 0, 255]);
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Str("INVALID".to_string()), // Invalid format
        rxchef::operation::ArgValue::Num(80.0),
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    
    let result = op.run(img_buf, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unsupported output format"));
}

#[test]
fn test_convert_image_format_large_image() {
    let op = ConvertImageFormat;
    
    // Create a larger image (100x100) (use RGB for JPEG compatibility)
    let mut img_buf = Vec::new();
    let mut img = image::RgbImage::new(100, 100);
    
    // Fill with a gradient pattern
    for y in 0..100 {
        for x in 0..100 {
            let color = ((x + y) % 256) as u8;
            img.put_pixel(x, y, image::Rgb([color, color, color]));
        }
    }
    
    let mut cursor = std::io::Cursor::new(&mut img_buf);
    img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
    
    let args = [
        rxchef::operation::ArgValue::Str("JPEG".to_string()),
        rxchef::operation::ArgValue::Num(85.0), // High quality
        rxchef::operation::ArgValue::Str("Auto".to_string()),
        rxchef::operation::ArgValue::Num(9.0),
    ];
    
    let result = op.run(img_buf, &args).unwrap();
    assert!(!result.is_empty());
    
    // Verify it's a valid JPEG
    let converted_img = image::load_from_memory(&result).unwrap();
    assert_eq!(converted_img.width(), 100);
    assert_eq!(converted_img.height(), 100);
}
