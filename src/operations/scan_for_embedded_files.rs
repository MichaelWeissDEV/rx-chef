/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Scan for Embedded Files operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Scan for Embedded Files operation
pub struct ScanForEmbeddedFiles;

struct FileSignature {
    name: &'static str,
    extension: &'static str,
    mime: &'static str,
    signature: Vec<(usize, Vec<u8>)>, // (offset, bytes)
}

impl Operation for ScanForEmbeddedFiles {
    fn name(&self) -> &'static str {
        "Scan for Embedded Files"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Scans the data for potential embedded files by looking for magic bytes at all offsets. This operation is prone to false positives."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Images",
                description: "Scan for image files",
                default_value: "true",
            },
            ArgSchema {
                name: "Video",
                description: "Scan for video files",
                default_value: "true",
            },
            ArgSchema {
                name: "Audio",
                description: "Scan for audio files",
                default_value: "true",
            },
            ArgSchema {
                name: "Documents",
                description: "Scan for document files",
                default_value: "true",
            },
            ArgSchema {
                name: "Applications",
                description: "Scan for application files",
                default_value: "true",
            },
            ArgSchema {
                name: "Archives",
                description: "Scan for archive files",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut output = "Scanning data for 'magic bytes' which may indicate embedded files. The following results may be false positives and should not be treated as reliable. Any sufficiently long file is likely to contain these magic bytes coincidentally.\n".to_string();

        let scan_images = args.first().and_then(|v| v.as_bool()).unwrap_or(true);
        let scan_video = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let scan_audio = args.get(2).and_then(|v| v.as_bool()).unwrap_or(true);
        let scan_docs = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);
        let scan_apps = args.get(4).and_then(|v| v.as_bool()).unwrap_or(true);
        let scan_archives = args.get(5).and_then(|v| v.as_bool()).unwrap_or(true);

        let mut signatures = Vec::new();

        if scan_images {
            signatures.push(FileSignature {
                name: "JPEG",
                extension: "jpg",
                mime: "image/jpeg",
                signature: vec![(0, vec![0xff, 0xd8, 0xff])],
            });
            signatures.push(FileSignature {
                name: "PNG",
                extension: "png",
                mime: "image/png",
                signature: vec![(0, vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])],
            });
            signatures.push(FileSignature {
                name: "GIF",
                extension: "gif",
                mime: "image/gif",
                signature: vec![(0, vec![0x47, 0x49, 0x46, 0x38])],
            });
        }
        if scan_video {
            signatures.push(FileSignature {
                name: "MPEG-4 video",
                extension: "mp4",
                mime: "video/mp4",
                signature: vec![(4, vec![0x66, 0x74, 0x79, 0x70])],
            });
        }
        if scan_audio {
            signatures.push(FileSignature {
                name: "MP3",
                extension: "mp3",
                mime: "audio/mpeg",
                signature: vec![(0, vec![0x49, 0x44, 0x33])],
            });
            signatures.push(FileSignature {
                name: "WAV",
                extension: "wav",
                mime: "audio/x-wav",
                signature: vec![
                    (0, vec![0x52, 0x49, 0x46, 0x46]),
                    (8, vec![0x57, 0x41, 0x56, 0x45]),
                ],
            });
        }
        if scan_docs {
            signatures.push(FileSignature {
                name: "PDF",
                extension: "pdf",
                mime: "application/pdf",
                signature: vec![(0, vec![0x25, 0x50, 0x44, 0x46])],
            });
        }
        if scan_apps {
            signatures.push(FileSignature {
                name: "Windows EXE",
                extension: "exe",
                mime: "application/x-msdownload",
                signature: vec![(0, vec![0x4d, 0x5a])],
            });
            signatures.push(FileSignature {
                name: "ELF",
                extension: "elf",
                mime: "application/x-executable",
                signature: vec![(0, vec![0x7f, 0x45, 0x4c, 0x46])],
            });
        }
        if scan_archives {
            signatures.push(FileSignature {
                name: "ZIP",
                extension: "zip",
                mime: "application/zip",
                signature: vec![(0, vec![0x50, 0x4b, 0x03, 0x04])],
            });
            signatures.push(FileSignature {
                name: "GZIP",
                extension: "gz",
                mime: "application/gzip",
                signature: vec![(0, vec![0x1f, 0x8b, 0x08])],
            });
            signatures.push(FileSignature {
                name: "TAR",
                extension: "tar",
                mime: "application/x-tar",
                signature: vec![(257, vec![0x75, 0x73, 0x74, 0x61, 0x72])],
            });
        }

        let mut found = 0;
        if !input.is_empty() {
            for i in 0..input.len() {
                for sig in &signatures {
                    let mut match_found = true;
                    for (offset, bytes) in &sig.signature {
                        if i + offset + bytes.len() > input.len() {
                            match_found = false;
                            break;
                        }
                        if &input[i + offset..i + offset + bytes.len()] != bytes.as_slice() {
                            match_found = false;
                            break;
                        }
                    }
                    if match_found {
                        found += 1;
                        output.push_str(&format!("\nOffset {} (0x{:x}):\n  File type:   {}\n  Extension:   {}\n  MIME type:   {}\n", i, i, sig.name, sig.extension, sig.mime));
                    }
                }
            }
        }

        if found == 0 {
            output.push_str("\nNo embedded files were found.");
        }

        Ok(output.into_bytes())
    }
}
