/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract Audio Metadata operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use lofty::{prelude::*, probe::Probe};
use serde::Serialize;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct ExtractAudioMetadata;

#[derive(Serialize)]
struct AudioMetadata {
    artifact: Artifact,
    tags: Tags,
}

#[derive(Serialize)]
struct Artifact {
    filename: Option<String>,
    byte_length: usize,
}

#[derive(Serialize)]
struct Tags {
    common: HashMap<String, String>,
}

use std::collections::HashMap;

impl Operation for ExtractAudioMetadata {
    fn name(&self) -> &'static str {
        "Extract Audio Metadata"
    }

    fn module(&self) -> &'static str {
        "Media"
    }

    fn description(&self) -> &'static str {
        "Extract common audio metadata across MP3, WAV, FLAC, OGG, etc. Outputs normalized JSON."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Filename (optional)",
                description: "Filename",
                default_value: "",
            },
            ArgSchema {
                name: "Max embedded text bytes",
                description: "Max text bytes",
                default_value: "524288",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Binary
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let filename = args.first().and_then(|a| a.as_str()).map(|s| s.to_string());

        let cursor = Cursor::new(&input);
        let tagged_file = Probe::new(cursor)
            .guess_file_type()
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?
            .read()
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let mut common_tags = HashMap::new();
        if let Some(tag) = tagged_file.primary_tag() {
            if let Some(artist) = tag.artist() {
                common_tags.insert("artist".to_string(), artist.to_string());
            }
            if let Some(title) = tag.title() {
                common_tags.insert("title".to_string(), title.to_string());
            }
            if let Some(album) = tag.album() {
                common_tags.insert("album".to_string(), album.to_string());
            }
            if let Some(genre) = tag.genre() {
                common_tags.insert("genre".to_string(), genre.to_string());
            }
        }

        let metadata = AudioMetadata {
            artifact: Artifact {
                filename,
                byte_length: input.len(),
            },
            tags: Tags {
                common: common_tags,
            },
        };

        serde_json::to_vec(&metadata).map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}
