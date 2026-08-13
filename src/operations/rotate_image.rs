/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rotate Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{ImageFormat, Rgba};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Rotate Image operation
pub struct RotateImage;

impl Operation for RotateImage {
    fn name(&self) -> &'static str {
        "Rotate Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Rotates an image by the specified number of degrees."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Rotation amount (degrees)",
            description: "Rotation amount in degrees",
            default_value: "90",
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
        if input.is_empty() {
            return Ok(input);
        }

        let degrees = args.first().and_then(|a| a.as_f64()).unwrap_or(90.0);

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Could not guess image format: {}", e))
        })?;
        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Error loading image: {}", e)))?;

        let rotated = match degrees as i64 {
            90 | -270 => img.rotate90(),
            180 | -180 => img.rotate180(),
            270 | -90 => img.rotate270(),
            _ if degrees.is_finite() => image::DynamicImage::ImageRgba8(rotate_about_center(
                &img.to_rgba8(),
                degrees.to_radians() as f32,
                Interpolation::Bilinear,
                Rgba([0, 0, 0, 0]),
            )),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Rotation amount (degrees)".to_string(),
                    reason: "Rotation must be a finite number".to_string(),
                });
            }
        };

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        // Use PNG if original was GIF as per CyberChef source
        let out_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        rotated
            .write_to(&mut cursor, out_format)
            .map_err(|e| OperationError::ProcessingError(format!("Error saving image: {}", e)))?;

        Ok(output)
    }
}
