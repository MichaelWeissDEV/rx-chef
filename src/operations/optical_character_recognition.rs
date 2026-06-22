/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Optical Character Recognition mit Tesseract (optional Feature)
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Optical Character Recognition operation
pub struct OpticalCharacterRecognition;

impl Operation for OpticalCharacterRecognition {
    fn name(&self) -> &'static str {
        "Optical Character Recognition"
    }

    fn module(&self) -> &'static str {
        "OCR"
    }

    fn description(&self) -> &'static str {
        "Optical character recognition or optical character reader (OCR) is the mechanical or electronic 
        conversion of images of typed, handwritten or printed text into machine-encoded text.\n\n
        Supported image formats: png, jpg, bmp, pbm.\n\n
        Requires Tesseract library. Enable with: --features tesseract"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Show confidence",
                description: "Whether to show the confidence level of the OCR",
                default_value: "true",
            },
            ArgSchema {
                name: "OCR Engine Mode",
                description: "The OCR engine mode to use",
                default_value: "LSTM only",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn is_broken(&self) -> bool {
        #[cfg(not(feature = "tesseract"))]
        return true;
        #[cfg(feature = "tesseract")]
        return false;
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        #[cfg(feature = "tesseract")]
        {
            self.run_tesseract(input, args)
        }
        #[cfg(not(feature = "tesseract"))]
        {
            let _ = (input, args);
            Err(OperationError::ProcessingError(
                "Optical Character Recognition requires --features tesseract. \
                 Enable with: cargo build --features tesseract. \
                 Also requires Tesseract OCR and Leptonica libraries installed on system."
                    .to_string(),
            ))
        }
    }
}

#[cfg(feature = "tesseract")]
impl OpticalCharacterRecognition {
    fn run_tesseract(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        use image::DynamicImage;
        use leptonica::Pix;
        use tesseract::TessApi;

        let show_confidence = args
            .get(0)
            .and_then(|a| a.as_str())
            .map(|s| s.to_lowercase() == "true")
            .unwrap_or(true);

        let _engine_mode = args.get(1).and_then(|a| a.as_str()).unwrap_or("LSTM only");

        // Load image
        let img = image::load_from_memory(&input)
            .map_err(|e| OperationError::InvalidInput(format!("Failed to load image: {}", e)))?;

        // Convert to RGB8
        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();

        // Create Leptonica Pix image
        let mut pix = Pix::create(w as i32, h as i32, 8, leptonica::ColorSpace::RGB);
        for y in 0..h {
            for x in 0..w {
                let p = rgb.get_pixel(x, y);
                let gray = (p[0] as u32 + p[1] as u32 + p[2] as u32) / 3;
                unsafe {
                    pix.set_pixel(x as i32, y as i32, gray as u32);
                }
            }
        }

        // Initialize Tesseract
        let api = TessApi::new(None, "eng").map_err(|e| {
            OperationError::ProcessingError(format!("Failed to initialize Tesseract: {}", e))
        })?;

        // Set image for OCR
        api.set_image_from_pix(&pix)
            .map_err(|e| OperationError::ProcessingError(format!("Failed to set image: {}", e)))?;

        // Get text
        let text = api.get_utf8_text().map_err(|e| {
            OperationError::ProcessingError(format!("Failed to extract text: {}", e))
        })?;

        let text = text.trim();

        if show_confidence {
            let conf = api.mean_confidence().map_err(|e| {
                OperationError::ProcessingError(format!("Failed to get confidence: {}", e))
            })?;
            Ok(format!("{}\n\nConfidence: {:.2}%", text, conf).into_bytes())
        } else {
            Ok(text.to_string().into_bytes())
        }
    }
}
