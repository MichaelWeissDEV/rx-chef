/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Resize Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{imageops::FilterType, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Resize Image operation
pub struct ResizeImage;

impl Operation for ResizeImage {
    fn name(&self) -> &'static str {
        "Resize Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Resizes an image to the specified width and height values."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Width",
                description: "Width of the new image",
                default_value: "100",
            },
            ArgSchema {
                name: "Height",
                description: "Height of the new image",
                default_value: "100",
            },
            ArgSchema {
                name: "Unit type",
                description: "Pixels or Percent",
                default_value: "Pixels",
            },
            ArgSchema {
                name: "Maintain aspect ratio",
                description: "Whether to maintain the aspect ratio",
                default_value: "false",
            },
            ArgSchema {
                name: "Resizing algorithm",
                description: "The algorithm to use for resizing",
                default_value: "Bilinear",
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
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let width_val = args.first().and_then(|a| a.as_f64()).unwrap_or(100.0);
        let height_val = args.get(1).and_then(|a| a.as_f64()).unwrap_or(100.0);
        let unit = args.get(2).and_then(|a| a.as_str()).unwrap_or("Pixels");
        let aspect = args.get(3).and_then(|a| a.as_bool()).unwrap_or(false);
        let algorithm = args.get(4).and_then(|a| a.as_str()).unwrap_or("Bilinear");

        let filter = match algorithm {
            "Nearest Neighbour" => FilterType::Nearest,
            "Bilinear" => FilterType::Triangle,
            "Bicubic" => FilterType::CatmullRom,
            "Hermite" => FilterType::Gaussian,
            "Bezier" => FilterType::Lanczos3, // Closest available in 'image' crate
            _ => FilterType::Triangle,
        };

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported or invalid image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Failed to load image: {}", e)))?;

        let (mut target_w, mut target_h) = if unit == "Percent" {
            (
                (img.width() as f64 * (width_val / 100.0)) as u32,
                (img.height() as f64 * (height_val / 100.0)) as u32,
            )
        } else {
            (width_val as u32, height_val as u32)
        };

        if target_w == 0 {
            target_w = 1;
        }
        if target_h == 0 {
            target_h = 1;
        }

        let resized = if aspect {
            img.resize(target_w, target_h, filter)
        } else {
            img.resize_exact(target_w, target_h, filter)
        };

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        // If it was GIF, CyberChef converts to PNG. We'll do the same if needed, or just keep format.
        let target_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        resized.write_to(&mut cursor, target_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write resized image: {}", e))
        })?;

        Ok(output)
    }
}
