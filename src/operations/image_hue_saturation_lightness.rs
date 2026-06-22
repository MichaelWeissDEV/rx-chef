/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Image Hue/Saturation/Lightness operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{DynamicImage, ImageFormat};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Image Hue/Saturation/Lightness operation
pub struct ImageHueSaturationLightness;

impl Operation for ImageHueSaturationLightness {
    fn name(&self) -> &'static str {
        "Image Hue/Saturation/Lightness"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Adjusts the hue / saturation / lightness (HSL) values of an image."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Hue",
                description: "The amount to adjust the hue by (degrees).",
                default_value: "0",
            },
            ArgSchema {
                name: "Saturation",
                description: "The amount to adjust the saturation by (%).",
                default_value: "0",
            },
            ArgSchema {
                name: "Lightness",
                description: "The amount to adjust the lightness by (%).",
                default_value: "0",
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
        let hue_adj = args
            .get(0)
            .and_then(|v| match v {
                ArgValue::Num(n) => Some(*n),
                ArgValue::Str(s) => s.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or(0.0);

        let sat_adj = args
            .get(1)
            .and_then(|v| match v {
                ArgValue::Num(n) => Some(*n),
                ArgValue::Str(s) => s.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or(0.0)
            / 100.0;

        let lig_adj = args
            .get(2)
            .and_then(|v| match v {
                ArgValue::Num(n) => Some(*n),
                ArgValue::Str(s) => s.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or(0.0)
            / 100.0;

        if input.is_empty() {
            return Ok(input);
        }

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        let mut rgba = img.to_rgba8();

        for pixel in rgba.pixels_mut() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;

            let (mut h, mut s, mut l) = rgb_to_hsl(r, g, b);

            // Adjust HSL
            h = (h + hue_adj as f32) % 360.0;
            if h < 0.0 {
                h += 360.0;
            }

            s = (s + sat_adj as f32).clamp(0.0, 1.0);
            l = (l + lig_adj as f32).clamp(0.0, 1.0);

            let (nr, ng, nb) = hsl_to_rgb(h, s, l);

            pixel[0] = (nr * 255.0) as u8;
            pixel[1] = (ng * 255.0) as u8;
            pixel[2] = (nb * 255.0) as u8;
        }

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let write_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        DynamicImage::ImageRgba8(rgba)
            .write_to(&mut cursor, write_format)
            .map_err(|e| {
                OperationError::ProcessingError(format!("Failed to write image: {}", e))
            })?;

        Ok(output)
    }
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };

    let mut h = if max == r {
        (g - b) / d + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    h *= 60.0;

    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let h_normalized = h / 360.0;

    let r = hue_to_rgb(p, q, h_normalized + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_normalized);
    let b = hue_to_rgb(p, q, h_normalized - 1.0 / 3.0);

    (r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}
