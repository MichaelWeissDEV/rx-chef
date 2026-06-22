/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract EXIF operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use exif::Reader;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract EXIF operation
pub struct ExtractEXIF;

impl Operation for ExtractEXIF {
    fn name(&self) -> &'static str {
        "Extract EXIF"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Extracts EXIF data from an image.\n\nEXIF data is metadata embedded in images (JPEG, JPG, TIFF) and audio files.\n\nEXIF data from photos usually contains information about the image file itself as well as the device used to create it."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Err(OperationError::InvalidInput("No input data.".to_string()));
        }

        let mut cursor = Cursor::new(&input);
        let reader = Reader::new();
        let exif = match reader.read_from_container(&mut cursor) {
            Ok(exif) => exif,
            Err(e) => {
                return Err(OperationError::ProcessingError(format!(
                    "Could not extract EXIF data from image: {}",
                    e
                )));
            }
        };

        let mut lines = Vec::new();
        let mut count = 0;

        for field in exif.fields() {
            lines.push(format!(
                "{}: {}",
                field.tag,
                field.display_value().with_unit(&exif)
            ));
            count += 1;
        }

        let mut output = format!("Found {} tags.\n\n", count);
        output.push_str(&lines.join("\n"));

        Ok(output.into_bytes())
    }
}
