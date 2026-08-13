/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse QR Code operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse QR Code operation
pub struct ParseQRCode;

impl Operation for ParseQRCode {
    fn name(&self) -> &'static str {
        "Parse QR Code"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Reads an image file and attempts to detect and read a Quick Response (QR) code from the image.<br><br><u>Normalise Image</u><br>Attempts to normalise the image before parsing it to improve detection of a QR code."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Normalise image",
            description: "Attempts to normalise the image before parsing it to improve detection of a QR code.",
            default_value: "false",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Err(OperationError::InvalidInput("No input".to_string()));
        }

        // Validate that it's an image
        let image = image::load_from_memory(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid image: {}", e)))?;
        let normalise = args.first().and_then(ArgValue::as_bool).unwrap_or(false);
        let grayscale = if normalise {
            image.adjust_contrast(25.0).to_luma8()
        } else {
            image.to_luma8()
        };
        let mut prepared = rqrr::PreparedImage::prepare_from_greyscale(
            grayscale.width() as usize,
            grayscale.height() as usize,
            |x, y| grayscale.get_pixel(x as u32, y as u32).0[0],
        );
        let grids = prepared.detect_grids();
        if grids.is_empty() {
            return Err(OperationError::ProcessingError(
                "No QR code found in image".to_string(),
            ));
        }
        let decoded = grids
            .iter()
            .map(|grid| {
                grid.decode()
                    .map(|(_, content)| content)
                    .map_err(|error| OperationError::ProcessingError(error.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(decoded.join("\n").into_bytes())
    }
}
