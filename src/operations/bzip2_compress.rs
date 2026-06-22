/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bzip2 Compress operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use bzip2::{read::BzEncoder, Compression};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bzip2 Compress operation
pub struct Bzip2Compress;

impl Operation for Bzip2Compress {
    fn name(&self) -> &'static str {
        "Bzip2 Compress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Bzip2 is a compression library developed by Julian Seward (of GHC fame) that uses the Burrows-Wheeler algorithm. It only supports compressing single files and its compression is slow, however is more effective than Deflate (.gz & .zip)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Block size (100s of kb)",
                description: "Block size for compression (1-9)",
                default_value: "9",
            },
            ArgSchema {
                name: "Work factor",
                description: "Effort spent on difficult data (0-250, 30 is default)",
                default_value: "30",
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
            return Err(OperationError::InvalidInput(
                "Please provide an input.".to_string(),
            ));
        }

        let block_size = if let Some(arg) = args.first() {
            arg.as_f64().unwrap_or(9.0) as u32
        } else {
            9
        };

        // Bzip2 compression level is 0-9. CyberChef uses 1-9.
        let compression = match block_size {
            0..=9 => Compression::new(block_size),
            _ => Compression::best(),
        };

        let mut encoder = BzEncoder::new(&input[..], compression);
        let mut compressed = Vec::new();
        encoder.read_to_end(&mut compressed).map_err(|e| {
            OperationError::InvalidInput(format!("Bzip2 compression failed: {}", e))
        })?;

        Ok(compressed)
    }
}
