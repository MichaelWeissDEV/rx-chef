/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Unzip operation.
 * -----------------------------------------------------------------------------
 */

use std::io::{Cursor, Read};

use zip::ZipArchive;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Unzip operation
pub struct Unzip;

impl Operation for Unzip {
    fn name(&self) -> &'static str {
        "Unzip"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data using the PKZIP algorithm and displays it per file, with support for passwords."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Password",
                description: "Password for encrypted zip files",
                default_value: "",
            },
            ArgSchema {
                name: "Verify result",
                description: "Verify result (ignored in this port)",
                default_value: "false",
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
            return Ok(b"No files found in zip.".to_vec());
        }

        let password = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let reader = Cursor::new(input);
        let mut archive = ZipArchive::new(reader).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to read zip archive: {}", e))
        })?;

        let mut output = Vec::new();

        for i in 0..archive.len() {
            let mut file = if password.is_empty() {
                archive.by_index(i).map_err(|e| {
                    OperationError::ProcessingError(format!(
                        "Failed to read zip entry {}: {}",
                        i, e
                    ))
                })?
            } else {
                archive
                    .by_index_decrypt(i, password.as_bytes())
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to decrypt zip entry {}: {}",
                            i, e
                        ))
                    })?
            };

            let name = file.name().to_string();
            if file.is_file() {
                let mut content = Vec::new();
                file.read_to_end(&mut content).map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to read entry content: {}", e))
                })?;

                output.extend_from_slice(format!("File: {}\n", name).as_bytes());
                output.extend_from_slice(b"--------------------------------------------------------------------------------\n");
                output.extend_from_slice(&content);
                output.extend_from_slice(b"\n\n");
            } else if file.is_dir() {
                output.extend_from_slice(format!("Directory: {}\n", name).as_bytes());
                output.extend_from_slice(b"--------------------------------------------------------------------------------\n\n");
            }
        }

        if output.is_empty() {
            Ok(b"No files found in zip.".to_vec())
        } else {
            Ok(output)
        }
    }
}
