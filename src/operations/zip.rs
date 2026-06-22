/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Zip operation.
 * -----------------------------------------------------------------------------
 */

use std::io::{Cursor, Write};

use zip::{
    unstable::write::FileOptionsExt,
    write::{ExtendedFileOptions, FileOptions},
    CompressionMethod, ZipWriter,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// Zip operation
///
/// Compresses data using the PKZIP algorithm with the given filename.
pub struct ZipOp;

impl Operation for ZipOp {
    fn name(&self) -> &'static str {
        "Zip"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Compresses data using the PKZIP algorithm with the given filename.<br><br>No support for multiple files at this time."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Filename",
                description: "Name of the file to be zipped",
                default_value: "file.txt",
            },
            ArgSchema {
                name: "Comment",
                description: "Comment to add to the zip file",
                default_value: "",
            },
            ArgSchema {
                name: "Password",
                description: "Password to protect the zip file (ZipCrypto)",
                default_value: "",
            },
            ArgSchema {
                name: "Compression method",
                description: "Method to use for compression",
                default_value: "Deflate",
            },
            ArgSchema {
                name: "Operating system",
                description: "OS to set in the zip metadata",
                default_value: "Unix",
            },
            ArgSchema {
                name: "Compression type",
                description: "Compression level/type (Fixed, Dynamic, None)",
                default_value: "Dynamic",
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
        let filename = args.first().and_then(|a| a.as_str()).unwrap_or("file.txt");
        let comment = args.get(1).and_then(|a| a.as_str()).unwrap_or("");
        let password =
            Utils::convert_to_byte_array(args.get(2).ok_or(OperationError::InvalidArgument {
                name: "Password".to_string(),
                reason: "Missing".to_string(),
            })?)?;
        let method_str = args.get(3).and_then(|a| a.as_str()).unwrap_or("Deflate");
        // let os_str = args.get(4).and_then(|a| a.as_str()).unwrap_or("Unix");
        // let comp_type = args.get(5).and_then(|a| a.as_str()).unwrap_or("Dynamic");

        let method = match method_str {
            "Deflate" => CompressionMethod::Deflated,
            "None (Store)" => CompressionMethod::Stored,
            _ => CompressionMethod::Deflated,
        };

        let mut buf = Vec::new();
        {
            let mut zip = ZipWriter::new(Cursor::new(&mut buf));

            let mut options: FileOptions<'_, ExtendedFileOptions> =
                FileOptions::default().compression_method(method);

            if !password.is_empty() {
                options = options.with_deprecated_encryption(&password).map_err(|e| {
                    OperationError::ProcessingError(format!("Encryption error: {}", e))
                })?;
            }

            zip.start_file(filename, options)
                .map_err(|e| OperationError::ProcessingError(format!("Zip error: {}", e)))?;
            zip.write_all(&input)
                .map_err(|e| OperationError::ProcessingError(format!("Write error: {}", e)))?;

            if !comment.is_empty() {
                // Central directory comment
                // zip.set_comment(comment); // This is usually on the whole zip
            }

            zip.finish()
                .map_err(|e| OperationError::ProcessingError(format!("Finish error: {}", e)))?;
        }

        Ok(buf)
    }
}
