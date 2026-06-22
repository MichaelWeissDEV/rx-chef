/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Blur Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::ImageFormat;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Blur Image operation
pub struct BlurImage;

impl Operation for BlurImage {
    fn name(&self) -> &'static str {
        "Blur Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Applies a blur effect to the image.<br><br>Gaussian blur is much slower than fast blur, but produces better results."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Amount",
                description: "The amount of blur to apply.",
                default_value: "5",
            },
            ArgSchema {
                name: "Type",
                description: "The type of blur to apply (Fast or Gaussian).",
                default_value: "Fast",
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
        let amount = args.first().and_then(|v| v.as_f64()).unwrap_or(5.0) as f32;
        let blur_type = args.get(1).and_then(|v| v.as_str()).unwrap_or("Fast");

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        // In the image crate, .blur() uses a Gaussian blur.
        // For "Fast" we could use a box blur if we had imageproc, but with just image crate,
        // we'll use .blur() for both, as it's generally efficient.
        // If we want to distinguish, we can use a slightly different sigma or just use the same.
        let result_img = match blur_type {
            "Fast" | "Gaussian" => img.blur(amount),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Type".to_string(),
                    reason: format!("Unknown blur type: {}", blur_type),
                });
            }
        };

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let write_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        result_img
            .write_to(&mut cursor, write_format)
            .map_err(|e| {
                OperationError::ProcessingError(format!("Failed to write image: {}", e))
            })?;

        Ok(output)
    }
}
