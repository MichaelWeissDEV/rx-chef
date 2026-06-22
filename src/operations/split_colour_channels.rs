/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Split Colour Channels operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{GenericImageView, ImageBuffer, Rgba};
use zip::{write::SimpleFileOptions, ZipWriter};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Split Colour Channels operation
pub struct SplitColourChannels;

impl Operation for SplitColourChannels {
    fn name(&self) -> &'static str {
        "Split Colour Channels"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Splits the given image into its red, green and blue colour channels."
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

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let (width, height) = img.dimensions();

        let mut red_img = ImageBuffer::new(width, height);
        let mut green_img = ImageBuffer::new(width, height);
        let mut blue_img = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.pixels() {
            let Rgba([r, g, b, a]) = pixel;
            red_img.put_pixel(x, y, Rgba([r, 0, 0, a]));
            green_img.put_pixel(x, y, Rgba([0, g, 0, a]));
            blue_img.put_pixel(x, y, Rgba([0, 0, b, a]));
        }

        let mut zip_buf = Vec::new();
        {
            let mut zip = ZipWriter::new(Cursor::new(&mut zip_buf));
            let options =
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

            // Red channel
            zip.start_file("red.png", options)
                .map_err(|e| OperationError::ProcessingError(format!("Zip error: {}", e)))?;
            let mut red_buf = Vec::new();
            image::DynamicImage::ImageRgba8(red_img)
                .write_to(&mut Cursor::new(&mut red_buf), image::ImageFormat::Png)
                .map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to write red PNG: {}", e))
                })?;
            std::io::Write::write_all(&mut zip, &red_buf)
                .map_err(|e| OperationError::ProcessingError(format!("Zip write error: {}", e)))?;

            // Green channel
            zip.start_file("green.png", options)
                .map_err(|e| OperationError::ProcessingError(format!("Zip error: {}", e)))?;
            let mut green_buf = Vec::new();
            image::DynamicImage::ImageRgba8(green_img)
                .write_to(&mut Cursor::new(&mut green_buf), image::ImageFormat::Png)
                .map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to write green PNG: {}", e))
                })?;
            std::io::Write::write_all(&mut zip, &green_buf)
                .map_err(|e| OperationError::ProcessingError(format!("Zip write error: {}", e)))?;

            // Blue channel
            zip.start_file("blue.png", options)
                .map_err(|e| OperationError::ProcessingError(format!("Zip error: {}", e)))?;
            let mut blue_buf = Vec::new();
            image::DynamicImage::ImageRgba8(blue_img)
                .write_to(&mut Cursor::new(&mut blue_buf), image::ImageFormat::Png)
                .map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to write blue PNG: {}", e))
                })?;
            std::io::Write::write_all(&mut zip, &blue_buf)
                .map_err(|e| OperationError::ProcessingError(format!("Zip write error: {}", e)))?;

            zip.finish()
                .map_err(|e| OperationError::ProcessingError(format!("Zip finish error: {}", e)))?;
        }

        Ok(zip_buf)
    }
}
