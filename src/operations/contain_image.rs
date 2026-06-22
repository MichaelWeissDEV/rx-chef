/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Contain Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Contain Image operation
pub struct ContainImage;

impl Operation for ContainImage {
    fn name(&self) -> &'static str {
        "Contain Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Scales an image to the specified width and height, maintaining the aspect ratio. The image may be letterboxed."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Width",
                description: "The width of the contained image",
                default_value: "100",
            },
            ArgSchema {
                name: "Height",
                description: "The height of the contained image",
                default_value: "100",
            },
            ArgSchema {
                name: "Horizontal align",
                description: "The horizontal alignment of the image within the container",
                default_value: "Center",
            },
            ArgSchema {
                name: "Vertical align",
                description: "The vertical alignment of the image within the container",
                default_value: "Middle",
            },
            ArgSchema {
                name: "Resizing algorithm",
                description: "The algorithm to use when resizing the image",
                default_value: "Bilinear",
            },
            ArgSchema {
                name: "Opaque background",
                description: "Whether to use an opaque black background instead of transparency",
                default_value: "true",
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
        let opaque_bg = args.get(5).and_then(|v| v.as_bool()).unwrap_or(true);

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
            "Bezier" => FilterType::Lanczos3, // Closest match in image crate
            _ => FilterType::Triangle,
        };

        // Resize image to fit while maintaining aspect ratio
        let resized = img.resize(width, height, filter);
        let (rw, rh) = resized.dimensions();

        // Calculate position based on alignment
        let x = match h_align {
            "Left" => 0,
            "Right" => width.saturating_sub(rw),
            _ => (width.saturating_sub(rw)) / 2, // Center
        };

        let y = match v_align {
            "Top" => 0,
            "Bottom" => height.saturating_sub(rh),
            _ => (height.saturating_sub(rh)) / 2, // Middle
        };

        // Create container image
        let mut container = if opaque_bg {
            DynamicImage::new_rgb8(width, height)
        } else {
            DynamicImage::new_rgba8(width, height)
        };

        // If opaque, it's already black (0,0,0). If transparent, it's already transparent (0,0,0,0).

        // Copy resized image into container
        image::imageops::overlay(&mut container, &resized, x as i64, y as i64);

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let out_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        container.write_to(&mut cursor, out_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}
