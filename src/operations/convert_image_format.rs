/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Convert Image Format operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{
    codecs::{
        jpeg::JpegEncoder,
        png::{CompressionType, FilterType, PngEncoder},
    },
    ImageEncoder, ImageFormat,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Convert Image Format operation
pub struct ConvertImageFormat;

impl Operation for ConvertImageFormat {
    fn name(&self) -> &'static str {
        "Convert Image Format"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Converts an image between different formats. Supported formats:<br><ul><li>Joint Photographic Experts Group (JPEG)</li><li>Portable Network Graphics (PNG)</li><li>Bitmap (BMP)</li><li>Tagged Image File Format (TIFF)</li></ul><br>Note: GIF files are supported for input, but cannot be outputted."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Output Format",
                description: "The format to convert the image to",
                default_value: "JPEG",
            },
            ArgSchema {
                name: "JPEG Quality",
                description: "The quality of the JPEG output (1-100)",
                default_value: "80",
            },
            ArgSchema {
                name: "PNG Filter Type",
                description: "The filter type to use for PNG output",
                default_value: "Auto",
            },
            ArgSchema {
                name: "PNG Deflate Level",
                description: "The deflate level to use for PNG output (0-9)",
                default_value: "9",
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
        let format_str = args.first().and_then(|v| v.as_str()).unwrap_or("JPEG");
        let jpeg_quality = args.get(1).and_then(|v| v.as_i64()).unwrap_or(80) as u8;
        let png_filter_str = args.get(2).and_then(|v| v.as_str()).unwrap_or("Auto");
        let png_deflate = args.get(3).and_then(|v| v.as_i64()).unwrap_or(9) as u8;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Error loading image: {}", e)))?;

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        match format_str {
            "JPEG" => {
                let encoder = JpegEncoder::new_with_quality(&mut cursor, jpeg_quality);
                encoder
                    .write_image(
                        img.as_bytes(),
                        img.width(),
                        img.height(),
                        img.color().into(),
                    )
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Error encoding JPEG: {}", e))
                    })?;
            }
            "PNG" => {
                let filter_type = match png_filter_str {
                    "None" => FilterType::NoFilter,
                    "Sub" => FilterType::Sub,
                    "Up" => FilterType::Up,
                    "Average" => FilterType::Avg,
                    "Paeth" => FilterType::Paeth,
                    _ => FilterType::Adaptive,
                };
                let compression = match png_deflate {
                    0 => CompressionType::Default, // Best Speed
                    1..=3 => CompressionType::Fast,
                    4..=6 => CompressionType::Default,
                    7..=9 => CompressionType::Best,
                    _ => CompressionType::Default,
                };
                let encoder = PngEncoder::new_with_quality(&mut cursor, compression, filter_type);
                encoder
                    .write_image(
                        img.as_bytes(),
                        img.width(),
                        img.height(),
                        img.color().into(),
                    )
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Error encoding PNG: {}", e))
                    })?;
            }
            "BMP" => {
                img.write_to(&mut cursor, ImageFormat::Bmp).map_err(|e| {
                    OperationError::ProcessingError(format!("Error encoding BMP: {}", e))
                })?;
            }
            "TIFF" => {
                img.write_to(&mut cursor, ImageFormat::Tiff).map_err(|e| {
                    OperationError::ProcessingError(format!("Error encoding TIFF: {}", e))
                })?;
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Output Format".to_string(),
                    reason: format!("Unsupported output format: {}", format_str),
                })
            }
        }

        Ok(output)
    }
}
