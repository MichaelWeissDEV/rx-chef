/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Tar operation.
 * -----------------------------------------------------------------------------
 */

use std::time::SystemTime;

use tar::{Builder, Header};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Tar operation
pub struct Tar;

impl Operation for Tar {
    fn name(&self) -> &'static str {
        "Tar"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Packs the input into a tarball.<br><br>No support for multiple files at this time."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Filename",
            description: "Name of the file to be packed",
            default_value: "file.txt",
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
        let filename = args.first().and_then(|v| v.as_str()).unwrap_or("file.txt");

        let mut builder = Builder::new(Vec::new());
        let mut header = Header::new_ustar();
        header
            .set_path(filename)
            .map_err(|e| OperationError::InvalidArgument {
                name: "Filename".to_string(),
                reason: e.to_string(),
            })?;
        header.set_size(input.len() as u64);
        header.set_mode(0o664);

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        header.set_mtime(now);
        header.set_cksum();

        builder
            .append(&header, &input[..])
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        let result = builder
            .into_inner()
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        Ok(result)
    }
}
