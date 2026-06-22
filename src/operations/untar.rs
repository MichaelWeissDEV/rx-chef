/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Untar operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Read;

use tar::Archive;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Untar operation
pub struct Untar;

impl Operation for Untar {
    fn name(&self) -> &'static str {
        "Untar"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Unpacks a tarball and displays it per file."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut archive = Archive::new(&input[..]);
        let mut output = Vec::new();

        let entries = archive.entries().map_err(|e| {
            OperationError::ProcessingError(format!("Failed to read tar entries: {}", e))
        })?;

        for entry in entries {
            let mut entry = entry.map_err(|e| {
                OperationError::ProcessingError(format!("Failed to read tar entry: {}", e))
            })?;

            let header = entry.header();
            let path = entry.path().map_err(|e| {
                OperationError::ProcessingError(format!("Failed to get entry path: {}", e))
            })?;
            let path_str = path.to_string_lossy().into_owned();

            if header.entry_type().is_file() {
                let mut content = Vec::new();
                entry.read_to_end(&mut content).map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to read entry content: {}", e))
                })?;

                output.extend_from_slice(format!("File: {}\n", path_str).as_bytes());
                output.extend_from_slice(b"--------------------------------------------------------------------------------\n");
                output.extend_from_slice(&content);
                output.extend_from_slice(b"\n\n");
            } else if header.entry_type().is_dir() {
                output.extend_from_slice(format!("Directory: {}\n", path_str).as_bytes());
                output.extend_from_slice(b"--------------------------------------------------------------------------------\n\n");
            }
        }

        if output.is_empty() {
            Ok(b"No files found in tarball.".to_vec())
        } else {
            Ok(output)
        }
    }
}
