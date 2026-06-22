/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Raw Deflate operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Write;

use flate2::{write::DeflateEncoder, Compression};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Raw Deflate operation
///
/// Compresses data using the raw deflate algorithm (no zlib or gzip headers).
pub struct RawDeflate;

fn compression_level(name: &str) -> Compression {
    match name.to_lowercase().as_str() {
        "no compression" | "none" | "0" => Compression::none(),
        "best speed" | "fast" | "1" => Compression::fast(),
        "best compression" | "best" | "9" => Compression::best(),
        _ => Compression::default(),
    }
}

impl Operation for RawDeflate {
    fn name(&self) -> &'static str {
        "Raw Deflate"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Compresses data using the raw deflate algorithm without any headers (RFC 1951)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Compression type",
            description:
                "Compression level: Dynamic (default), Best speed, Best compression, No compression",
            default_value: "Dynamic",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let compression_name = args.first().and_then(|a| a.as_str()).unwrap_or("Dynamic");

        let level = compression_level(compression_name);
        let mut encoder = DeflateEncoder::new(Vec::new(), level);
        encoder.write_all(&input).map_err(|e| {
            OperationError::ProcessingError(format!("Raw deflate write failed: {}", e))
        })?;
        let compressed = encoder.finish().map_err(|e| {
            OperationError::ProcessingError(format!("Raw deflate finish failed: {}", e))
        })?;
        Ok(compressed)
    }
}
