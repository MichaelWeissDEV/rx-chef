/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract Files operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract Files operation
pub struct ExtractFiles;

#[allow(dead_code)]
struct FileSignature {
    name: &'static str,
    extension: &'static str,
    mime: &'static str,
    sig: &'static [u8],
    end_sig: Option<&'static [u8]>,
}

impl Operation for ExtractFiles {
    fn name(&self) -> &'static str {
        "Extract Files"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Performs file carving to attempt to extract files from the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Images",
                description: "Extract images",
                default_value: "true",
            },
            ArgSchema {
                name: "Documents",
                description: "Extract documents",
                default_value: "true",
            },
            ArgSchema {
                name: "Archives",
                description: "Extract archives",
                default_value: "true",
            },
            ArgSchema {
                name: "Ignore failed extractions",
                description: "Ignore failed extractions",
                default_value: "true",
            },
            ArgSchema {
                name: "Minimum File Size",
                description: "Minimum file size to extract",
                default_value: "100",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes // Actually a list of files in CyberChef, but we'll return concatenated data or just the first one for simplicity in this port
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let min_size = args.get(4).and_then(|v| v.as_usize()).unwrap_or(100);

        let mut signatures = Vec::new();
        if args.first().and_then(|v| v.as_bool()).unwrap_or(true) {
            signatures.push(FileSignature {
                name: "PNG",
                extension: "png",
                mime: "image/png",
                sig: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
                end_sig: Some(&[0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82]),
            });
            signatures.push(FileSignature {
                name: "JPG",
                extension: "jpg",
                mime: "image/jpeg",
                sig: &[0xFF, 0xD8, 0xFF],
                end_sig: Some(&[0xFF, 0xD9]),
            });
            signatures.push(FileSignature {
                name: "GIF",
                extension: "gif",
                mime: "image/gif",
                sig: &[0x47, 0x49, 0x46, 0x38],
                end_sig: Some(&[0x00, 0x3B]),
            });
        }
        if args.get(1).and_then(|v| v.as_bool()).unwrap_or(true) {
            signatures.push(FileSignature {
                name: "PDF",
                extension: "pdf",
                mime: "application/pdf",
                sig: b"%PDF-",
                end_sig: Some(b"%%EOF"),
            });
        }
        if args.get(2).and_then(|v| v.as_bool()).unwrap_or(true) {
            signatures.push(FileSignature {
                name: "ZIP",
                extension: "zip",
                mime: "application/zip",
                sig: &[0x50, 0x4B, 0x03, 0x04],
                end_sig: None, // ZIP is complex, we'll just take a chunk or skip for now
            });
        }

        let mut output = Vec::new();
        let mut i = 0;
        while i < input.len() {
            for signature in &signatures {
                if i + signature.sig.len() <= input.len()
                    && &input[i..i + signature.sig.len()] == signature.sig
                {
                    let mut end = input.len();
                    if let Some(end_sig) = signature.end_sig {
                        if let Some(pos) = input[i + signature.sig.len()..]
                            .windows(end_sig.len())
                            .position(|w| w == end_sig)
                        {
                            end = i + signature.sig.len() + pos + end_sig.len();
                        }
                    } else if signature.name == "ZIP" {
                        // Very basic ZIP chunking
                        end = std::cmp::min(i + 1024 * 1024, input.len());
                    }

                    let file_data = &input[i..end];
                    if file_data.len() >= min_size {
                        output.extend_from_slice(
                            format!("--- Extracted {} ---\n", signature.name).as_bytes(),
                        );
                        output.extend_from_slice(file_data);
                        output.extend_from_slice(b"\n");
                        i = end;
                        break;
                    }
                }
            }
            i += 1;
        }

        if output.is_empty() {
            Ok("No files found.".to_string().into_bytes())
        } else {
            Ok(output)
        }
    }
}
