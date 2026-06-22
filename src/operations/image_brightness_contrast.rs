/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Image Brightness / Contrast operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::ImageFormat;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Image Brightness / Contrast operation
pub struct ImageBrightnessContrast;

impl Operation for ImageBrightnessContrast {
    fn name(&self) -> &'static str {
        "Image Brightness / Contrast"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Adjust the brightness or contrast of an image."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Brightness",
                description: "The amount to adjust the brightness by. (-100 to 100)",
                default_value: "0",
            },
            ArgSchema {
                name: "Contrast",
                description: "The amount to adjust the contrast by. (-100 to 100)",
                default_value: "0",
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
        let brightness = args
            .get(0)
            .and_then(|v| match v {
                ArgValue::Num(n) => Some(*n),
                ArgValue::Str(s) => s.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or(0.0);

        let contrast = args
            .get(1)
            .and_then(|v| match v {
                ArgValue::Num(n) => Some(*n),
                ArgValue::Str(s) => s.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or(0.0);

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let mut img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        // Adjust brightness
        // CyberChef brightness is -100 to 100, Jimp factor is -1 to 1.
        // image crate brighten takes i32, where 255 is full brightness.
        if brightness != 0.0 {
            let b_val = (brightness / 100.0 * 255.0) as i32;
            img = img.brighten(b_val);
        }

        // Adjust contrast
        // Jimp factor calculation:
        // factor = (n + 1) / (1 - n) where n is -1 to 1.
        if contrast != 0.0 {
            let n = (contrast / 100.0).clamp(-0.999, 0.999);
            let factor = (n + 1.0) / (1.0 - n);
            img = img.adjust_contrast(factor as f32);
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
