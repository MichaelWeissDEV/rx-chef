/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Dither Image operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Dither Image operation
pub struct DitherImage;

impl Operation for DitherImage {
    fn name(&self) -> &'static str {
        "Dither Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Apply Floyd-Steinberg black-and-white dithering to a PNG, JPEG, GIF, BMP, TIFF, or WebP image. The result is encoded as PNG."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        use image::{DynamicImage, GrayImage, ImageFormat, Luma};
        use std::io::Cursor;

        let source = image::load_from_memory(&input)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?
            .to_luma8();
        let (width, height) = source.dimensions();
        let mut values = source
            .pixels()
            .map(|pixel| pixel.0[0] as f32)
            .collect::<Vec<_>>();
        let offset = |x: u32, y: u32| (y * width + x) as usize;
        for y in 0..height {
            for x in 0..width {
                let index = offset(x, y);
                let old = values[index];
                let new = if old < 128.0 { 0.0 } else { 255.0 };
                values[index] = new;
                let error = old - new;
                if x + 1 < width {
                    values[offset(x + 1, y)] += error * 7.0 / 16.0;
                }
                if y + 1 < height {
                    if x > 0 {
                        values[offset(x - 1, y + 1)] += error * 3.0 / 16.0;
                    }
                    values[offset(x, y + 1)] += error * 5.0 / 16.0;
                    if x + 1 < width {
                        values[offset(x + 1, y + 1)] += error / 16.0;
                    }
                }
            }
        }
        let output = GrayImage::from_fn(width, height, |x, y| {
            Luma([values[offset(x, y)].clamp(0.0, 255.0) as u8])
        });
        let mut encoded = Cursor::new(Vec::new());
        DynamicImage::ImageLuma8(output)
            .write_to(&mut encoded, ImageFormat::Png)
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
        Ok(encoded.into_inner())
    }
}
