/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Render Image operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::guess_format;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Render Image operation
pub struct RenderImageOp;

impl Operation for RenderImageOp {
    fn name(&self) -> &'static str {
        "Render Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Displays the input as an image. Supports the following formats: jpg/jpeg, png, gif, webp, bmp, ico."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "Raw, Base64, or Hex",
            default_value: "Raw",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_format = args.first().and_then(|v| v.as_str()).unwrap_or("Raw");

        let data = match input_format {
            "Hex" => {
                let s = String::from_utf8_lossy(&input)
                    .replace(' ', "")
                    .replace('\n', "")
                    .replace('\r', "");
                hex::decode(s).map_err(|e| OperationError::InvalidArgument {
                    name: "Input".to_string(),
                    reason: format!("Invalid hex: {}", e),
                })?
            }
            "Base64" => {
                let s = String::from_utf8_lossy(&input)
                    .replace(' ', "")
                    .replace('\n', "")
                    .replace('\r', "");
                BASE64
                    .decode(s)
                    .map_err(|e| OperationError::InvalidArgument {
                        name: "Input".to_string(),
                        reason: format!("Invalid base64: {}", e),
                    })?
            }
            _ => input,
        };

        if data.is_empty() {
            return Ok(Vec::new());
        }

        // Check if it's a valid image format
        let format = guess_format(&data)
            .map_err(|_| OperationError::InvalidInput("Invalid image format".to_string()))?;

        // Match CyberChef behavior: return an <img> tag with a data URI
        let mime = match format {
            image::ImageFormat::Png => "image/png",
            image::ImageFormat::Jpeg => "image/jpeg",
            image::ImageFormat::Gif => "image/gif",
            image::ImageFormat::WebP => "image/webp",
            image::ImageFormat::Bmp => "image/bmp",
            image::ImageFormat::Ico => "image/x-icon",
            _ => "image/unknown",
        };

        let b64 = BASE64.encode(&data);
        let html = format!("<img src='data:{};base64,{}'>", mime, b64);

        Ok(html.into_bytes())
    }
}
