/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Flip Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::ImageFormat;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Flip Image operation
pub struct FlipImage;

impl Operation for FlipImage {
    fn name(&self) -> &'static str {
        "Flip Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Flips an image along its X or Y axis."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Axis",
            description: "The axis to flip along",
            default_value: "Horizontal",
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
        if input.is_empty() {
            return Ok(input);
        }

        let axis = args
            .first()
            .and_then(|v| v.as_str())
            .unwrap_or("Horizontal");

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Failed to load image: {}", e)))?;

        let flipped = match axis {
            "Horizontal" => img.fliph(),
            "Vertical" => img.flipv(),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Axis".to_string(),
                    reason: format!("Invalid axis: {}", axis),
                })
            }
        };

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        // CyberChef: if GIF, convert to PNG. Else keep same format.
        let out_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        flipped.write_to(&mut cursor, out_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}
