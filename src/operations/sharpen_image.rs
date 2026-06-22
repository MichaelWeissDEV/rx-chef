/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Sharpen Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{DynamicImage, GenericImageView, ImageFormat, Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Sharpen Image operation
pub struct SharpenImage;

impl Operation for SharpenImage {
    fn name(&self) -> &'static str {
        "Sharpen Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Sharpens an image using the Unsharp Mask algorithm."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Radius",
                description: "Blur radius for the unsharp mask",
                default_value: "2",
            },
            ArgSchema {
                name: "Amount",
                description: "The amount of sharpening to apply",
                default_value: "1",
            },
            ArgSchema {
                name: "Threshold",
                description: "Luminance difference threshold",
                default_value: "10",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let radius = args.first().and_then(|a| a.as_f64()).unwrap_or(2.0);
        let amount = args.get(1).and_then(|a| a.as_f64()).unwrap_or(1.0);
        let threshold = args.get(2).and_then(|a| a.as_f64()).unwrap_or(10.0);

        let img = image::load_from_memory(&input).map_err(|e| OperationError::InvalidArgument {
            name: "Input".to_string(),
            reason: format!("Failed to load image: {}", e),
        })?;

        let (width, height) = img.dimensions();
        let rgba_img = img.to_rgba8();

        // Gaussian blur for the mask
        // imageproc::gaussian_blur_f32 uses sigma. radius roughly maps to sigma.
        let sigma = radius;
        let blurred_img = gaussian_blur_f32(&rgba_img, sigma as f32);

        let mut sharpened_img = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let original_pixel = rgba_img.get_pixel(x, y);
                let blurred_pixel = blurred_img.get_pixel(x, y);

                let mut new_pixel = [0u8; 4];
                new_pixel[3] = original_pixel[3]; // Keep alpha

                // Calculate luminance difference
                let original_luma = 0.2126 * original_pixel[0] as f64
                    + 0.7152 * original_pixel[1] as f64
                    + 0.0722 * original_pixel[2] as f64;
                let blurred_luma = 0.2126 * blurred_pixel[0] as f64
                    + 0.7152 * blurred_pixel[1] as f64
                    + 0.0722 * blurred_pixel[2] as f64;

                let _mask_luma = if original_luma > blurred_luma {
                    original_luma - blurred_luma
                } else {
                    0.0
                };
                let luma_diff = (original_luma - blurred_luma).abs();

                if (luma_diff / 255.0) * 100.0 >= threshold {
                    for i in 0..3 {
                        let mask_val = if original_pixel[i] > blurred_pixel[i] {
                            (original_pixel[i] - blurred_pixel[i]) as f64
                        } else {
                            0.0
                        };

                        let sharpened = original_pixel[i] as f64 + mask_val * amount;
                        new_pixel[i] = sharpened.clamp(0.0, 255.0) as u8;
                    }
                } else {
                    for i in 0..3 {
                        new_pixel[i] = original_pixel[i];
                    }
                }

                sharpened_img.put_pixel(x, y, Rgba(new_pixel));
            }
        }

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);
        DynamicImage::ImageRgba8(sharpened_img)
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to save image: {}", e)))?;

        Ok(output)
    }
}
