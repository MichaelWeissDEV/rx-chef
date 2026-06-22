/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Play Media operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Play Media operation
pub struct PlayMedia;

impl Operation for PlayMedia {
    fn name(&self) -> &'static str {
        "Play Media"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Plays the input as audio or video depending on the type.<br><br>Tags: sound, movie, mp3, mp4, mov, webm, wav, ogg"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "Input format of the media",
            default_value: "Raw",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        let input_format = args.first().and_then(|v| v.as_str()).unwrap_or("Raw");

        let data = match input_format {
            "Hex" => {
                let s = String::from_utf8_lossy(&input).replace(|c: char| c.is_whitespace(), "");
                hex::decode(s)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
            }
            "Base64" => {
                let s = String::from_utf8_lossy(&input).replace(|c: char| c.is_whitespace(), "");
                data_encoding::BASE64
                    .decode(s.as_bytes())
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid base64: {}", e)))?
            }
            "Raw" => input,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Input format".to_string(),
                    reason: "Unknown format".to_string(),
                })
            }
        };

        // Validate if it is audio or video
        // Common magic bytes:
        // MP4: [4..8] == "ftyp"
        // FLV: [0..3] == "FLV\x01"
        // WAV: [0..4] == "RIFF", [8..12] == "WAVE"
        // MP3: [0..3] == "ID3" or [0..2] == [0xFF, 0xFB]
        // OGG: [0..4] == "OggS"
        // WEBM/MKV: [0..4] == [0x1A, 0x45, 0xDF, 0xA3]

        let is_media = (data.len() > 8 && &data[4..8] == b"ftyp") || // MP4
            (data.len() > 4 && &data[0..4] == b"FLV\x01") || // FLV
            (data.len() > 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WAVE") || // WAV
            (data.len() > 3 && &data[0..3] == b"ID3") || // MP3 with ID3
            (data.len() > 2 && data[0] == 0xFF && (data[1] & 0xE0) == 0xE0) || // MP3 raw
            (data.len() > 4 && &data[0..4] == b"OggS") || // OGG
            (data.len() > 4 && &data[0..4] == &[0x1A, 0x45, 0xDF, 0xA3]); // WebM/MKV

        if !is_media {
            return Err(OperationError::InvalidInput(
                "Invalid or unrecognised file type".to_string(),
            ));
        }

        Ok(data)
    }
}
