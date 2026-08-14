/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Add Text To Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use font8x8::UnicodeFonts;
use image::{DynamicImage, ImageFormat, Rgba};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Add Text To Image operation
pub struct AddTextToImage;

impl Operation for AddTextToImage {
    fn name(&self) -> &'static str {
        "Add Text To Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Adds text onto an image.<br><br>Text can be horizontally or vertically aligned, or the position can be manually specified."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Text",
                description: "The text to add.",
                default_value: "",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Horizontal align",
                description: "None, Left, Center, Right",
                default_value: "None",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Vertical align",
                description: "None, Top, Middle, Bottom",
                default_value: "None",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "X position",
                description: "Manual X position",
                default_value: "0",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Y position",
                description: "Manual Y position",
                default_value: "0",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Size",
                description: "Font size",
                default_value: "32",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Red",
                description: "Red component (0-255)",
                default_value: "255",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Green",
                description: "Green component (0-255)",
                default_value: "255",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Blue",
                description: "Blue component (0-255)",
                default_value: "255",
                kind: crate::operation::ArgKind::Integer,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Alpha",
                description: "Alpha component (0-255)",
                default_value: "255",
                kind: crate::operation::ArgKind::Integer,
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
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let h_align = args.get(1).and_then(|v| v.as_str()).unwrap_or("None");
        let v_align = args.get(2).and_then(|v| v.as_str()).unwrap_or("None");
        let mut x_pos = args.get(3).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let mut y_pos = args.get(4).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let size = args.get(5).and_then(|v| v.as_f64()).unwrap_or(32.0) as f32;
        if size <= 0.0 {
            return Err(OperationError::InvalidArgument {
                name: "Size".into(),
                reason: "Font size must be positive".into(),
            });
        }
        let channel = |index| {
            args.get(index)
                .and_then(ArgValue::as_i64)
                .unwrap_or(255)
                .clamp(0, 255) as u8
        };
        let colour = Rgba([channel(6), channel(7), channel(8), channel(9)]);

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let mut img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let scale = (size / 8.0).round().max(1.0) as i32;
        let lines: Vec<_> = text.lines().collect();
        let text_width = lines
            .iter()
            .map(|line| line.chars().count() as i32 * 9 * scale)
            .max()
            .unwrap_or(0);
        let text_height = lines.len() as i32 * 9 * scale;
        let mut rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();
        match h_align {
            "Left" => x_pos = 0,
            "Center" => x_pos = (width as i32 - text_width) / 2,
            "Right" => x_pos = width as i32 - text_width,
            _ => {}
        }
        match v_align {
            "Top" => y_pos = 0,
            "Middle" => y_pos = (height as i32 - text_height) / 2,
            "Bottom" => y_pos = height as i32 - text_height,
            _ => {}
        }
        for (line_index, line) in lines.iter().enumerate() {
            for (column, character) in line.chars().enumerate() {
                let glyph = font8x8::BASIC_FONTS
                    .get(character)
                    .or_else(|| font8x8::BASIC_FONTS.get('?'))
                    .unwrap();
                draw_bitmap_glyph(
                    &mut rgba_img,
                    &glyph,
                    x_pos + column as i32 * 9 * scale,
                    y_pos + line_index as i32 * 9 * scale,
                    scale,
                    colour,
                );
            }
        }
        img = DynamicImage::ImageRgba8(rgba_img);

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let write_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        img.write_to(&mut cursor, write_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}

fn draw_bitmap_glyph(
    image: &mut image::RgbaImage,
    glyph: &[u8; 8],
    x: i32,
    y: i32,
    scale: i32,
    colour: Rgba<u8>,
) {
    for (row, bits) in glyph.iter().enumerate() {
        for column in 0..8 {
            if bits & (1 << column) == 0 {
                continue;
            }
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = x + column * scale + dx;
                    let py = y + row as i32 * scale + dy;
                    if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32
                    {
                        image.put_pixel(px as u32, py as u32, colour);
                    }
                }
            }
        }
    }
}
