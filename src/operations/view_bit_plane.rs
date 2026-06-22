/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the View Bit Plane operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{load_from_memory, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// View Bit Plane operation
///
/// Extracts and displays a bit plane of any given image. These show only a single bit from each pixel, and can be used to hide messages in Steganography.
pub struct ViewBitPlane;

impl Operation for ViewBitPlane {
    fn name(&self) -> &'static str {
        "View Bit Plane"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Extracts and displays a bit plane of any given image. These show only a single bit from each pixel, and can be used to hide messages in Steganography."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Colour",
                description: "The colour channel to view",
                default_value: "Red",
            },
            ArgSchema {
                name: "Bit",
                description: "The bit to view",
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
        let colour = args.first().and_then(|a| a.as_str()).unwrap_or("Red");
        let bit = args.get(1).and_then(|a| a.as_f64()).unwrap_or(0.0) as u8;

        if bit > 7 {
            return Err(OperationError::InvalidArgument {
                name: "Bit".to_string(),
                reason: "Bit argument must be between 0 and 7".to_string(),
            });
        }

        let channel_idx = match colour {
            "Red" => 0,
            "Green" => 1,
            "Blue" => 2,
            "Alpha" => 3,
            _ => 0,
        };

        let mut img = match load_from_memory(&input) {
            Ok(img) => img.into_rgba8(),
            Err(e) => {
                return Err(OperationError::InvalidInput(format!(
                    "Please enter a valid image file: {}",
                    e
                )))
            }
        };

        for pixel in img.pixels_mut() {
            let val = pixel[channel_idx];
            let is_one = (val >> bit) & 1 == 1;

            // From CC: if bit is 1, newPixelValue = 0 (black); if 0, newPixelValue = 255 (white).
            let new_pixel_value = if is_one { 0 } else { 255 };

            pixel[0] = new_pixel_value;
            pixel[1] = new_pixel_value;
            pixel[2] = new_pixel_value;
            pixel[3] = 255;
        }

        let mut output = Vec::new();
        // CyberChef tries to keep the mime type, but PNG is a safe default for raw buffers.
        let mut cursor = Cursor::new(&mut output);
        if let Err(e) = img.write_to(&mut cursor, ImageFormat::Png) {
            return Err(OperationError::ProcessingError(format!(
                "Failed to write output image: {}",
                e
            )));
        }

        Ok(output)
    }
}
