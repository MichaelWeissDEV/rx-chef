/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract RGBA operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract RGBA operation
pub struct ExtractRGBA;

impl Operation for ExtractRGBA {
    fn name(&self) -> &'static str {
        "Extract RGBA"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Extracts each pixel's RGBA value in an image. These are sometimes used in Steganography to hide text or data."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The delimiter between values.",
                default_value: " ",
            },
            ArgSchema {
                name: "Include Alpha",
                description: "Whether to include the alpha channel.",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let delimiter = args.first().and_then(|v| v.as_str()).unwrap_or(" ");
        let include_alpha = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let mut values = Vec::new();
        let rgba = img.to_rgba8();

        for pixel in rgba.pixels() {
            values.push(pixel[0].to_string());
            values.push(pixel[1].to_string());
            values.push(pixel[2].to_string());
            if include_alpha {
                values.push(pixel[3].to_string());
            }
        }

        Ok(values.join(delimiter).into_bytes())
    }
}
