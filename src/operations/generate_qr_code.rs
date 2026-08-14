/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate QR Code operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate QR Code operation
pub struct GenerateQRCodeOp;

impl Operation for GenerateQRCodeOp {
    fn name(&self) -> &'static str {
        "Generate QR Code"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Generates a Quick Response (QR) code from the input text."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Image Format",
                description: "Format of the QR code image",
                default_value: "PNG",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Module size (px)",
                description: "Size of each module in pixels",
                default_value: "5",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Margin (num modules)",
                description: "Margin around the QR code in modules",
                default_value: "4",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Error correction",
                description: "Error correction level",
                default_value: "Medium",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(|a| a.as_str()).unwrap_or("PNG");
        let module_size = args.get(1).and_then(ArgValue::as_usize).unwrap_or(5);
        let margin = args.get(2).and_then(ArgValue::as_usize).unwrap_or(4);
        let correction = args.get(3).and_then(ArgValue::as_str).unwrap_or("Medium");
        if input.is_empty() {
            return Err(OperationError::InvalidInput("No input".to_string()));
        }
        if module_size == 0 || module_size > 256 {
            return Err(OperationError::InvalidArgument {
                name: "Module size (px)".to_string(),
                reason: "must be between 1 and 256".to_string(),
            });
        }
        if margin > 256 {
            return Err(OperationError::InvalidArgument {
                name: "Margin (num modules)".to_string(),
                reason: "must be between 0 and 256".to_string(),
            });
        }

        let level = match correction.to_ascii_lowercase().as_str() {
            "low" | "l" => qrcode::EcLevel::L,
            "medium" | "m" => qrcode::EcLevel::M,
            "quartile" | "q" => qrcode::EcLevel::Q,
            "high" | "h" => qrcode::EcLevel::H,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Error correction".to_string(),
                    reason: "expected Low, Medium, Quartile, or High".to_string(),
                })
            }
        };
        let code = qrcode::QrCode::with_error_correction_level(&input, level)
            .map_err(|error| OperationError::InvalidInput(error.to_string()))?;

        match format.to_ascii_uppercase().as_str() {
            "PNG" => render_png(&code, module_size, margin),
            "SVG" => Ok(render_svg(&code, module_size, margin).into_bytes()),
            _ => Err(OperationError::InvalidArgument {
                name: "Image Format".to_string(),
                reason: "expected PNG or SVG".to_string(),
            }),
        }
    }
}

fn render_png(
    code: &qrcode::QrCode,
    module_size: usize,
    margin: usize,
) -> Result<Vec<u8>, OperationError> {
    use image::{DynamicImage, GrayImage, ImageFormat, Luma};
    use std::io::Cursor;

    let qr_width = code.width();
    let image_width = (qr_width + margin * 2)
        .checked_mul(module_size)
        .and_then(|size| u32::try_from(size).ok())
        .ok_or_else(|| OperationError::InvalidInput("QR image is too large".to_string()))?;
    let mut image = GrayImage::from_pixel(image_width, image_width, Luma([255]));
    for (index, color) in code.to_colors().iter().enumerate() {
        if *color != qrcode::Color::Dark {
            continue;
        }
        let module_x = index % qr_width + margin;
        let module_y = index / qr_width + margin;
        for y in 0..module_size {
            for x in 0..module_size {
                image.put_pixel(
                    ((module_x * module_size) + x) as u32,
                    ((module_y * module_size) + y) as u32,
                    Luma([0]),
                );
            }
        }
    }

    let mut output = Cursor::new(Vec::new());
    DynamicImage::ImageLuma8(image)
        .write_to(&mut output, ImageFormat::Png)
        .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
    Ok(output.into_inner())
}

fn render_svg(code: &qrcode::QrCode, module_size: usize, margin: usize) -> String {
    use std::fmt::Write;

    let qr_width = code.width();
    let image_width = (qr_width + margin * 2) * module_size;
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{image_width}" height="{image_width}" viewBox="0 0 {image_width} {image_width}" shape-rendering="crispEdges"><rect width="100%" height="100%" fill="white"/><path fill="black" d=""#
    );
    for (index, color) in code.to_colors().iter().enumerate() {
        if *color == qrcode::Color::Dark {
            let x = (index % qr_width + margin) * module_size;
            let y = (index / qr_width + margin) * module_size;
            let _ = write!(svg, "M{x} {y}h{module_size}v{module_size}h-{module_size}z");
        }
    }
    svg.push_str(r#""/></svg>"#);
    svg
}
