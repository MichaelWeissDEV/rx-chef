/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Randomize Colour Palette operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{GenericImage, GenericImageView, Pixel, Rgba};
use md5::{Digest, Md5};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Randomize Colour Palette operation
pub struct RandomizeColourPalette;

impl Operation for RandomizeColourPalette {
    fn name(&self) -> &'static str {
        "Randomize Colour Palette"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Randomizes each colour in an image's colour palette. This can often reveal text or symbols that were previously a very similar colour to their surroundings, a technique sometimes used in Steganography."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Seed",
            description: "The seed for the randomization.",
            default_value: "",
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
        let seed = args.first().and_then(|v| v.as_str()).unwrap_or("");

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let (width, height) = img.dimensions();
        let mut result_img = img.clone();

        // CyberChef logic:
        // rgbString = this.bitmap.data.slice(idx, idx + 3).join(".");
        // rgbHash = runHash("md5", Utils.strToArrayBuffer(seed + rgbString));
        // rgbHex = rgbHash.substr(0, 6) + "ff";
        // parsedImage.setPixelColor(parseInt(rgbHex, 16), x, y);

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let rgba = pixel.to_rgba();
                let rgb_string = format!("{}.{}.{}", rgba[0], rgba[1], rgba[2]);

                let mut hasher = Md5::new();
                hasher.update(seed.as_bytes());
                hasher.update(rgb_string.as_bytes());
                let hash = hasher.finalize();

                // Take first 3 bytes from hash for RGB
                let new_r = hash[0];
                let new_g = hash[1];
                let new_b = hash[2];

                result_img.put_pixel(x, y, Rgba([new_r, new_g, new_b, rgba[3]]));
            }
        }

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        result_img.write_to(&mut cursor, format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}
