/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract LSB operation.
 * -----------------------------------------------------------------------------
 */

use image::GenericImageView;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract LSB operation
pub struct ExtractLSB;

impl Operation for ExtractLSB {
    fn name(&self) -> &'static str {
        "Extract LSB"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Extracts the Least Significant Bit data from each pixel in an image. This is a common way to hide data in Steganography."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Colour Pattern #1",
                description: "Colour to extract from",
                default_value: "R",
            },
            ArgSchema {
                name: "Colour Pattern #2",
                description: "Colour to extract from",
                default_value: "",
            },
            ArgSchema {
                name: "Colour Pattern #3",
                description: "Colour to extract from",
                default_value: "",
            },
            ArgSchema {
                name: "Colour Pattern #4",
                description: "Colour to extract from",
                default_value: "",
            },
            ArgSchema {
                name: "Pixel Order",
                description: "Order to process pixels",
                default_value: "Row",
            },
            ArgSchema {
                name: "Bit",
                description: "Bit to extract (0-7)",
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
        let bit = args.get(5).and_then(|v| v.as_usize()).unwrap_or(0);
        if bit > 7 {
            return Err(OperationError::InvalidArgument {
                name: "Bit".to_string(),
                reason: "Must be between 0 and 7".to_string(),
            });
        }

        let pixel_order = args.get(4).and_then(|v| v.as_str()).unwrap_or("Row");

        let mut colours = Vec::new();
        for i in 0..4 {
            if let Some(c) = args.get(i).and_then(|v| v.as_str()) {
                match c {
                    "R" => colours.push(0),
                    "G" => colours.push(1),
                    "B" => colours.push(2),
                    "A" => colours.push(3),
                    _ => {}
                }
            }
        }

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();

        let mut bits = Vec::new();

        if pixel_order == "Row" {
            for pixel in rgba.pixels() {
                for &colour in &colours {
                    bits.push((pixel[colour] >> bit) & 1);
                }
            }
        } else {
            for x in 0..width {
                for y in 0..height {
                    let pixel = rgba.get_pixel(x, y);
                    for &colour in &colours {
                        bits.push((pixel[colour] >> bit) & 1);
                    }
                }
            }
        }

        let mut output = Vec::with_capacity((bits.len() + 7) / 8);
        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &b) in chunk.iter().enumerate() {
                if b == 1 {
                    byte |= 1 << (chunk.len() - 1 - i);
                }
            }
            output.push(byte);
        }

        Ok(output)
    }
}
