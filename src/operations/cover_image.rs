/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Cover Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{imageops::FilterType, GenericImageView, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Cover Image operation
pub struct CoverImage;

impl Operation for CoverImage {
    fn name(&self) -> &'static str {
        "Cover Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Scales the image to the given width and height, keeping the aspect ratio. The image may be clipped."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Width",
                description: "The width of the covered image",
                default_value: "100",
            },
            ArgSchema {
                name: "Height",
                description: "The height of the covered image",
                default_value: "100",
            },
            ArgSchema {
                name: "Horizontal align",
                description: "The horizontal alignment of the image within the cover area",
                default_value: "Center",
            },
            ArgSchema {
                name: "Vertical align",
                description: "The vertical alignment of the image within the cover area",
                default_value: "Middle",
            },
            ArgSchema {
                name: "Resizing algorithm",
                description: "The algorithm to use when resizing the image",
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
            return Ok(input);
        }

        let width = args.first().and_then(|v| v.as_i64()).unwrap_or(100) as u32;
        let height = args.get(1).and_then(|v| v.as_i64()).unwrap_or(100) as u32;
        let h_align = args.get(2).and_then(|v| v.as_str()).unwrap_or("Center");
        let v_align = args.get(3).and_then(|v| v.as_str()).unwrap_or("Middle");
        let algorithm = args.get(4).and_then(|v| v.as_str()).unwrap_or("Bilinear");

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let filter = match algorithm {
            "Nearest Neighbour" => FilterType::Nearest,
            "Bilinear" => FilterType::Triangle,
            "Bicubic" => FilterType::CatmullRom,
            "Hermite" => FilterType::Gaussian,
            "Bezier" => FilterType::Lanczos3,
            _ => FilterType::Triangle,
        };

        // Resize image to fill while maintaining aspect ratio
        // We can use resize_to_fill for center alignment, but for custom alignment we do it manually.

        let (orig_w, orig_h) = img.dimensions();
        let ratio_w = width as f32 / orig_w as f32;
        let ratio_h = height as f32 / orig_h as f32;
        let ratio = ratio_w.max(ratio_h);

        let new_w = (orig_w as f32 * ratio).round() as u32;
        let new_h = (orig_h as f32 * ratio).round() as u32;

        let resized = img.resize(new_w, new_h, filter);

        let x = match h_align {
            "Left" => 0,
            "Right" => new_w.saturating_sub(width),
            _ => (new_w.saturating_sub(width)) / 2,
        };

        let y = match v_align {
            "Top" => 0,
            "Bottom" => new_h.saturating_sub(height),
            _ => (new_h.saturating_sub(height)) / 2,
        };

        let cropped = resized.crop_imm(x, y, width, height);

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let out_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        cropped.write_to(&mut cursor, out_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}
