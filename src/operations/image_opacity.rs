/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Image Opacity operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{DynamicImage, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Image Opacity operation
pub struct ImageOpacity;

impl Operation for ImageOpacity {
    fn name(&self) -> &'static str {
        "Image Opacity"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Adjust the opacity of an image."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Opacity (%)",
            description: "The opacity to set (0-100).",
            default_value: "100",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let opacity = args.first().and_then(|v| v.as_f64()).unwrap_or(100.0) / 100.0;

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let mut rgba = img.to_rgba8();

        if opacity != 1.0 {
            for pixel in rgba.pixels_mut() {
                let a = pixel[3] as f32;
                pixel[3] = (a * opacity as f32).clamp(0.0, 255.0) as u8;
            }
        }

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let write_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        DynamicImage::ImageRgba8(rgba)
            .write_to(&mut cursor, write_format)
            .map_err(|e| {
                OperationError::ProcessingError(format!("Failed to write image: {}", e))
            })?;

        Ok(output)
    }
}
