/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Remove EXIF operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Remove EXIF operation
pub struct RemoveEXIF;

impl Operation for RemoveEXIF {
    fn name(&self) -> &'static str {
        "Remove EXIF"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Removes EXIF data from a JPEG image.\n\nEXIF data embedded in photos usually contains information about the image file itself as well as the device used to create it."
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

        if input.len() < 2 || input[0] != 0xff || input[1] != 0xd8 {
            return Err(OperationError::InvalidInput(
                "Given data is not a JPEG.".to_string(),
            ));
        }

        match remove_exif_internal(&input) {
            Ok(output) => Ok(output),
            Err(e) if e == "Exif not found." => Ok(input),
            Err(e) => Err(OperationError::ProcessingError(format!(
                "Could not remove EXIF data from image: {}",
                e
            ))),
        }
    }
}

fn remove_exif_internal(input: &[u8]) -> Result<Vec<u8>, String> {
    let segments = split_into_segments(input)?;

    if segments.len() < 2 {
        return Err("Exif not found.".to_string());
    }

    let mut new_segments = Vec::new();
    new_segments.push(segments[0]);

    let mut exif_found = false;

    // CyberChef checks segments[1] and segments[2]
    for (i, &segment) in segments.iter().enumerate().skip(1) {
        if !exif_found
            && i <= 2
            && segment.len() >= 10
            && segment[0] == 0xff
            && segment[1] == 0xe1
            && &segment[4..10] == b"Exif\0\0"
        {
            exif_found = true;
            continue;
        }
        new_segments.push(segment);
    }

    if !exif_found {
        return Err("Exif not found.".to_string());
    }

    let mut output = Vec::new();
    for segment in new_segments {
        output.extend_from_slice(segment);
    }

    Ok(output)
}

fn split_into_segments(data: &[u8]) -> Result<Vec<&[u8]>, String> {
    if data.len() < 2 || data[0] != 0xff || data[1] != 0xd8 {
        return Err("Given data isn't JPEG.".to_string());
    }

    let mut segments = Vec::new();
    segments.push(&data[0..2]);

    let mut head = 2;
    while head < data.len() {
        if head + 2 > data.len() {
            return Err("Wrong JPEG data.".to_string());
        }

        if data[head] == 0xff && data[head + 1] == 0xda {
            // SOS (Start Of Scan)
            segments.push(&data[head..]);
            break;
        } else {
            if head + 4 > data.len() {
                return Err("Wrong JPEG data.".to_string());
            }
            let length = ((data[head + 2] as usize) << 8) | (data[head + 3] as usize);
            let end_point = head + length + 2;
            if end_point > data.len() {
                return Err("Wrong JPEG data.".to_string());
            }
            segments.push(&data[head..end_point]);
            head = end_point;
        }
    }

    Ok(segments)
}
