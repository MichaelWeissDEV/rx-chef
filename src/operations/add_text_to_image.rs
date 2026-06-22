/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Add Text To Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use image::{DynamicImage, ImageFormat, Rgba};
use imageproc::drawing::draw_text_mut;

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
            },
            ArgSchema {
                name: "Horizontal align",
                description: "None, Left, Center, Right",
                default_value: "None",
            },
            ArgSchema {
                name: "Vertical align",
                description: "None, Top, Middle, Bottom",
                default_value: "None",
            },
            ArgSchema {
                name: "X position",
                description: "Manual X position",
                default_value: "0",
            },
            ArgSchema {
                name: "Y position",
                description: "Manual Y position",
                default_value: "0",
            },
            ArgSchema {
                name: "Size",
                description: "Font size",
                default_value: "32",
            },
            ArgSchema {
                name: "Red",
                description: "Red component (0-255)",
                default_value: "255",
            },
            ArgSchema {
                name: "Green",
                description: "Green component (0-255)",
                default_value: "255",
            },
            ArgSchema {
                name: "Blue",
                description: "Blue component (0-255)",
                default_value: "255",
            },
            ArgSchema {
                name: "Alpha",
                description: "Alpha component (0-255)",
                default_value: "255",
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
        let r = args.get(6).and_then(|v| v.as_i64()).unwrap_or(255) as u8;
        let g = args.get(7).and_then(|v| v.as_i64()).unwrap_or(255) as u8;
        let b = args.get(8).and_then(|v| v.as_i64()).unwrap_or(255) as u8;
        let a = args.get(9).and_then(|v| v.as_i64()).unwrap_or(255) as u8;

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let mut img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        // Try to load a font. Using a hardcoded path for now or a placeholder.
        // In a real scenario, we'd want a way to provide font data.
        let font_data = std::fs::read("/Users/michaelweiss/space/decode/CyberChef/src/web/static/fonts/MaterialIcons-Regular.ttf").ok();

        if let Some(data) = font_data {
            let font = FontArc::try_from_vec(data).map_err(|e| {
                OperationError::ProcessingError(format!("Failed to parse font: {}", e))
            })?;
            let scale = PxScale::from(size);
            let scaled_font = font.as_scaled(scale);

            let mut rgba_img = img.to_rgba8();

            // Calculate alignments
            let (width, height) = rgba_img.dimensions();

            // Simple text width estimation
            let ascent = scaled_font.ascent();
            let descent = scaled_font.descent();
            let text_height = (ascent - descent) as i32;

            let mut text_width = 0.0;
            let mut last_glyph_id = None;
            for c in text.chars() {
                let glyph_id = font.glyph_id(c);
                if let Some(last_id) = last_glyph_id {
                    text_width += scaled_font.kern(last_id, glyph_id);
                }
                text_width += scaled_font.h_advance(glyph_id);
                last_glyph_id = Some(glyph_id);
            }
            let text_width_i32 = text_width as i32;

            match h_align {
                "Left" => x_pos = 0,
                "Center" => x_pos = (width as i32 - text_width_i32) / 2,
                "Right" => x_pos = width as i32 - text_width_i32,
                _ => {}
            }

            match v_align {
                "Top" => y_pos = 0,
                "Middle" => y_pos = (height as i32 - text_height) / 2,
                "Bottom" => y_pos = height as i32 - text_height,
                _ => {}
            }

            draw_text_mut(
                &mut rgba_img,
                Rgba([r, g, b, a]),
                x_pos,
                y_pos,
                scale,
                &font,
                text,
            );
            img = DynamicImage::ImageRgba8(rgba_img);
        }

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
