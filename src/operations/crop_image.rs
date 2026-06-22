/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Crop Image operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use image::{DynamicImage, GenericImageView, ImageFormat, Rgba};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Crop Image operation
pub struct CropImage;

impl Operation for CropImage {
    fn name(&self) -> &'static str {
        "Crop Image"
    }

    fn module(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "Crops an image to the specified region, or automatically crops edges.<br><br><b><u>Autocrop</u></b><br>Automatically crops same-colour borders from the image.<br><br><u>Autocrop tolerance</u><br>A percentage value for the tolerance of colour difference between pixels.<br><br><u>Only autocrop frames</u><br>Only crop real frames (all sides must have the same border)<br><br><u>Symmetric autocrop</u><br>Force autocrop to be symmetric (top/bottom and left/right are cropped by the same amount)<br><br><u>Autocrop keep border</u><br>The number of pixels of border to leave around the image."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "X Position",
                description: "The x-coordinate of the top-left corner of the crop area",
                default_value: "0",
            },
            ArgSchema {
                name: "Y Position",
                description: "The y-coordinate of the top-left corner of the crop area",
                default_value: "0",
            },
            ArgSchema {
                name: "Width",
                description: "The width of the crop area",
                default_value: "10",
            },
            ArgSchema {
                name: "Height",
                description: "The height of the crop area",
                default_value: "10",
            },
            ArgSchema {
                name: "Autocrop",
                description: "Whether to automatically crop borders",
                default_value: "false",
            },
            ArgSchema {
                name: "Autocrop tolerance (%)",
                description: "The tolerance for color difference when autocropping",
                default_value: "2",
            },
            ArgSchema {
                name: "Only autocrop frames",
                description: "Only crop if all sides have the same border",
                default_value: "true",
            },
            ArgSchema {
                name: "Symmetric autocrop",
                description: "Force autocrop to be symmetric",
                default_value: "false",
            },
            ArgSchema {
                name: "Autocrop keep border (px)",
                description: "The number of pixels of border to leave",
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
        if input.is_empty() {
            return Ok(input);
        }

        let x_pos = args.first().and_then(|v| v.as_i64()).unwrap_or(0) as u32;
        let y_pos = args.get(1).and_then(|v| v.as_i64()).unwrap_or(0) as u32;
        let width = args.get(2).and_then(|v| v.as_i64()).unwrap_or(10) as u32;
        let height = args.get(3).and_then(|v| v.as_i64()).unwrap_or(10) as u32;
        let autocrop = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);
        let auto_tolerance = args.get(5).and_then(|v| v.as_f64()).unwrap_or(2.0);
        let auto_frames = args.get(6).and_then(|v| v.as_bool()).unwrap_or(true);
        let auto_symmetric = args.get(7).and_then(|v| v.as_bool()).unwrap_or(false);
        let auto_border = args.get(8).and_then(|v| v.as_i64()).unwrap_or(0) as u32;

        let format = image::guess_format(&input).map_err(|e| {
            OperationError::InvalidInput(format!("Unsupported image format: {}", e))
        })?;

        let mut img = image::load_from_memory(&input)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to load image: {}", e)))?;

        if autocrop {
            img = self.do_autocrop(
                img,
                auto_tolerance,
                auto_frames,
                auto_symmetric,
                auto_border,
            )?;
        } else {
            img = img.crop(x_pos, y_pos, width, height);
        };

        let mut output = Vec::new();
        let mut cursor = Cursor::new(&mut output);

        let out_format = if format == ImageFormat::Gif {
            ImageFormat::Png
        } else {
            format
        };

        img.write_to(&mut cursor, out_format).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to write image: {}", e))
        })?;

        Ok(output)
    }
}

impl CropImage {
    fn do_autocrop(
        &self,
        mut img: DynamicImage,
        tolerance: f64,
        only_frames: bool,
        symmetric: bool,
        keep_border: u32,
    ) -> Result<DynamicImage, OperationError> {
        let (w, h) = img.dimensions();
        if w == 0 || h == 0 {
            return Ok(img);
        }

        let border_color = img.get_pixel(0, 0);
        let tolerance_val = (tolerance / 100.0 * 255.0) as u32;

        let is_similar = |p1: Rgba<u8>, p2: Rgba<u8>| {
            let diff = (p1[0] as i32 - p2[0] as i32).abs() as u32
                + (p1[1] as i32 - p2[1] as i32).abs() as u32
                + (p1[2] as i32 - p2[2] as i32).abs() as u32
                + (p1[3] as i32 - p2[3] as i32).abs() as u32;
            diff <= tolerance_val * 4
        };

        let mut top = 0;
        'top: while top < h {
            for x in 0..w {
                if !is_similar(img.get_pixel(x, top), border_color) {
                    break 'top;
                }
            }
            top += 1;
        }

        let mut bottom = 0;
        'bottom: while bottom < h - top {
            for x in 0..w {
                if !is_similar(img.get_pixel(x, h - 1 - bottom), border_color) {
                    break 'bottom;
                }
            }
            bottom += 1;
        }

        let mut left = 0;
        'left: while left < w {
            for y in 0..h {
                if !is_similar(img.get_pixel(left, y), border_color) {
                    break 'left;
                }
            }
            left += 1;
        }

        let mut right = 0;
        'right: while right < w - left {
            for y in 0..h {
                if !is_similar(img.get_pixel(w - 1 - right, y), border_color) {
                    break 'right;
                }
            }
            right += 1;
        }

        if only_frames {
            let max_border = top.min(bottom).min(left).min(right);
            top = max_border;
            bottom = max_border;
            left = max_border;
            right = max_border;
        }

        if symmetric {
            top = top.min(bottom);
            bottom = top;
            left = left.min(right);
            right = left;
        }

        let final_top = top.saturating_sub(keep_border);
        let final_left = left.saturating_sub(keep_border);
        let final_bottom = bottom.saturating_sub(keep_border);
        let final_right = right.saturating_sub(keep_border);

        let crop_w = w.saturating_sub(final_left).saturating_sub(final_right);
        let crop_h = h.saturating_sub(final_top).saturating_sub(final_bottom);

        if crop_w == 0 || crop_h == 0 {
            return Ok(img);
        }

        img = img.crop(final_left, final_top, crop_w, crop_h);
        Ok(img)
    }
}
