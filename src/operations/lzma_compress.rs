/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LZMA Compress operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use xz2::read::XzEncoder;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub struct LZMACompress;

impl Operation for LZMACompress {
    fn name(&self) -> &'static str {
        "LZMA Compress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Compresses data using the Lempel\u{2013}Ziv\u{2013}Markov chain algorithm. Compression mode determines the speed and effectiveness of the compression: 1 is fastest and less effective, 9 is slowest and most effective"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Compression Mode",
                description: "Compression mode determines the speed and effectiveness of the compression: 1 is fastest and less effective, 9 is slowest and most effective",
                default_value: "7",
            }
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
        let mode_str = args.first().and_then(|v| v.as_str()).unwrap_or("7");
        let preset = mode_str
            .parse::<u32>()
            .map_err(|_| OperationError::InvalidArgument {
                name: "Compression Mode".to_string(),
                reason: format!("Invalid compression mode: {}", mode_str),
            })?;

        let mut encoder = XzEncoder::new(&input[..], preset);

        let mut compressed = Vec::new();
        encoder.read_to_end(&mut compressed).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to compress LZMA: {}", e))
        })?;

        Ok(compressed)
    }
}
