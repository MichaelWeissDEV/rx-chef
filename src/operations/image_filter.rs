/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Image Filter operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{DynamicImage, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Image Filter operation
pub struct ImageFilter;

impl Operation for ImageFilter {
    fn name(&self) -> &'static str {
        "Image Filter"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Applies a greyscale or sepia filter to an image."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Filter type",
            description: "The filter to apply.",
            default_value: "Greyscale", // Options: Greyscale, Sepia
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
        let filter_type = args.first().and_then(|v| v.as_str()).unwrap_or("Greyscale");

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let mut img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        match filter_type {
            "Greyscale" | "Grayscale" => {
                img = img.grayscale();
            }
            "Sepia" => {
                let mut rgba = img.to_rgba8();
                for pixel in rgba.pixels_mut() {
                    let r = pixel[0] as f32;
                    let g = pixel[1] as f32;
                    let b = pixel[2] as f32;

                    let tr = (r * 0.393) + (g * 0.769) + (b * 0.189);
                    let tg = (r * 0.349) + (g * 0.686) + (b * 0.168);
                    let tb = (r * 0.272) + (g * 0.534) + (b * 0.131);

                    pixel[0] = tr.min(255.0) as u8;
                    pixel[1] = tg.min(255.0) as u8;
                    pixel[2] = tb.min(255.0) as u8;
                }
                img = DynamicImage::ImageRgba8(rgba);
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Filter type".to_string(),
                    reason: format!("Unknown filter type: {}", filter_type),
                });
            }
        }

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let write_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        img.write_to(&mut cursor, write_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}
