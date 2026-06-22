/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Normalise Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{DynamicImage, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Normalise Image operation
pub struct NormaliseImage;

impl Operation for NormaliseImage {
    fn name(&self) -> &'static str {
        "Normalise Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Normalise the image colours by stretching the contrast."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let mut rgb = img.to_rgb8();
        let mut min_r = 255;
        let mut max_r = 0;
        let mut min_g = 255;
        let mut max_g = 0;
        let mut min_b = 255;
        let mut max_b = 0;

        for pixel in rgb.pixels() {
            if pixel[0] < min_r {
                min_r = pixel[0];
            }
            if pixel[0] > max_r {
                max_r = pixel[0];
            }
            if pixel[1] < min_g {
                min_g = pixel[1];
            }
            if pixel[1] > max_g {
                max_g = pixel[1];
            }
            if pixel[2] < min_b {
                min_b = pixel[2];
            }
            if pixel[2] > max_b {
                max_b = pixel[2];
            }
        }

        if max_r > min_r || max_g > min_g || max_b > min_b {
            for pixel in rgb.pixels_mut() {
                if max_r > min_r {
                    pixel[0] = (((pixel[0] as f32 - min_r as f32) / (max_r as f32 - min_r as f32))
                        * 255.0) as u8;
                }
                if max_g > min_g {
                    pixel[1] = (((pixel[1] as f32 - min_g as f32) / (max_g as f32 - min_g as f32))
                        * 255.0) as u8;
                }
                if max_b > min_b {
                    pixel[2] = (((pixel[2] as f32 - min_b as f32) / (max_b as f32 - min_b as f32))
                        * 255.0) as u8;
                }
            }
        }

        let new_img = DynamicImage::ImageRgb8(rgb);
        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let write_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        new_img.write_to(&mut cursor, write_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}
