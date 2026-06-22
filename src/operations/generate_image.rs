/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{ImageBuffer, Rgba};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate Image operation
pub struct GenerateImageOp;

impl Operation for GenerateImageOp {
    fn name(&self) -> &'static str {
        "Generate Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Generates an image using the input as pixel values."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Mode",
                description: "Pixel representation mode",
                default_value: "Greyscale",
            },
            ArgSchema {
                name: "Pixel Scale Factor",
                description: "Scale factor for pixels",
                default_value: "8",
            },
            ArgSchema {
                name: "Pixels per row",
                description: "Width of the image in pixels",
                default_value: "64",
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
        let mode = args.first().and_then(|a| a.as_str()).unwrap_or("Greyscale");
        let scale = args.get(1).and_then(|a| a.as_f64()).unwrap_or(8.0) as u32;
        let width = args.get(2).and_then(|a| a.as_f64()).unwrap_or(64.0) as u32;

        if scale == 0 || width == 0 {
            return Err(OperationError::InvalidArgument {
                name: "Pixel Scale Factor / Pixels per row".to_string(),
                reason: "Must be greater than 0".to_string(),
            });
        }

        let bytes_per_pixel = match mode {
            "Greyscale" => 1,
            "RG" => 2,
            "RGB" => 3,
            "RGBA" => 4,
            "Bits" => 0, // Handled separately
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Mode".to_string(),
                    reason: format!("Unsupported mode: {}", mode),
                })
            }
        };

        if bytes_per_pixel > 0 && input.len() % bytes_per_pixel != 0 {
            return Err(OperationError::InvalidInput(format!(
                "Number of bytes ({}) is not a divisor of bytes per pixel ({})",
                input.len(),
                bytes_per_pixel
            )));
        }

        let pixel_count = if mode == "Bits" {
            input.len() as u32 * 8
        } else {
            (input.len() / bytes_per_pixel) as u32
        };

        if pixel_count == 0 {
            return Ok(Vec::new());
        }

        let height = (pixel_count + width - 1) / width;

        let mut img_rgba = ImageBuffer::new(width, height);

        if mode == "Bits" {
            let mut pixel_idx = 0;
            for byte in input {
                for i in 0..8 {
                    if pixel_idx >= pixel_count {
                        break;
                    }
                    let x = pixel_idx % width;
                    let y = pixel_idx / width;
                    let bit = (byte >> (7 - i)) & 1;
                    let val = if bit == 0 { 255 } else { 0 };
                    img_rgba.put_pixel(x, y, Rgba([val, val, val, 255]));
                    pixel_idx += 1;
                }
            }
        } else {
            for (i, chunk) in input.chunks(bytes_per_pixel).enumerate() {
                let x = (i as u32) % width;
                let y = (i as u32) / width;
                let pixel = match mode {
                    "Greyscale" => Rgba([chunk[0], chunk[0], chunk[0], 255]),
                    "RG" => Rgba([chunk[0], chunk[1], 0, 255]),
                    "RGB" => Rgba([chunk[0], chunk[1], chunk[2], 255]),
                    "RGBA" => Rgba([chunk[0], chunk[1], chunk[2], chunk[3]]),
                    _ => unreachable!(),
                };
                img_rgba.put_pixel(x, y, pixel);
            }
        }

        let final_img = if scale > 1 {
            image::imageops::resize(
                &img_rgba,
                width * scale,
                height * scale,
                image::imageops::FilterType::Nearest,
            )
        } else {
            img_rgba
        };

        let mut buffer = Vec::new();
        final_img
            .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to write PNG: {}", e)))?;

        Ok(buffer)
    }
}
