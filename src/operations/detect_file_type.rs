/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Detect File Type operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Detect File Type operation
pub struct DetectFileType;

struct Signature {
    name: &'static str,
    extension: &'static str,
    mime: &'static str,
    sig: &'static [(usize, u8)],
}

impl Operation for DetectFileType {
    fn name(&self) -> &'static str {
        "Detect File Type"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Attempts to guess the MIME (Multipurpose Internet Mail Extensions) type of the data based on 'magic bytes'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Images",
                description: "Detect image files",
                default_value: "true",
            },
            ArgSchema {
                name: "Video",
                description: "Detect video files",
                default_value: "true",
            },
            ArgSchema {
                name: "Audio",
                description: "Detect audio files",
                default_value: "true",
            },
            ArgSchema {
                name: "Documents",
                description: "Detect document files",
                default_value: "true",
            },
            ArgSchema {
                name: "Applications",
                description: "Detect application files",
                default_value: "true",
            },
            ArgSchema {
                name: "Archives",
                description: "Detect archive files",
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
        let detect_images = args.first().and_then(|v| v.as_bool()).unwrap_or(true);
        let detect_video = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let detect_audio = args.get(2).and_then(|v| v.as_bool()).unwrap_or(true);
        let detect_docs = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);
        let detect_apps = args.get(4).and_then(|v| v.as_bool()).unwrap_or(true);
        let detect_archives = args.get(5).and_then(|v| v.as_bool()).unwrap_or(true);

        let mut signatures = Vec::new();

        if detect_images {
            signatures.push(Signature {
                name: "Joint Photographic Experts Group image",
                extension: "jpg",
                mime: "image/jpeg",
                sig: &[(0, 0xFF), (1, 0xD8), (2, 0xFF)],
            });
            signatures.push(Signature {
                name: "Graphics Interchange Format image",
                extension: "gif",
                mime: "image/gif",
                sig: &[(0, 0x47), (1, 0x49), (2, 0x46), (3, 0x38)],
            });
            signatures.push(Signature {
                name: "Portable Network Graphics image",
                extension: "png",
                mime: "image/png",
                sig: &[(0, 0x89), (1, 0x50), (2, 0x4E), (3, 0x47)],
            });
        }

        if detect_video {
            signatures.push(Signature {
                name: "MPEG-4 video",
                extension: "mp4",
                mime: "video/mp4",
                sig: &[(4, 0x66), (5, 0x74), (6, 0x79), (7, 0x70)],
            });
            signatures.push(Signature {
                name: "Flash Video",
                extension: "flv",
                mime: "video/x-flv",
                sig: &[(0, 0x46), (1, 0x4C), (2, 0x56), (3, 0x01)],
            });
        }

        if detect_audio {
            signatures.push(Signature {
                name: "Waveform Audio",
                extension: "wav",
                mime: "audio/x-wav",
                sig: &[
                    (0, 0x52),
                    (1, 0x49),
                    (2, 0x46),
                    (3, 0x46),
                    (8, 0x57),
                    (9, 0x41),
                    (10, 0x56),
                    (11, 0x45),
                ],
            });
            signatures.push(Signature {
                name: "MPEG-3 audio",
                extension: "mp3",
                mime: "audio/mpeg",
                sig: &[(0, 0x49), (1, 0x44), (2, 0x33)],
            });
        }

        if detect_docs {
            signatures.push(Signature {
                name: "Portable Document Format",
                extension: "pdf",
                mime: "application/pdf",
                sig: &[(0, 0x25), (1, 0x50), (2, 0x44), (3, 0x46)],
            });
            signatures.push(Signature {
                name: "Rich Text Format",
                extension: "rtf",
                mime: "application/rtf",
                sig: &[(0, 0x7B), (1, 0x5C), (2, 0x72), (3, 0x74)],
            });
        }

        if detect_apps {
            signatures.push(Signature {
                name: "Windows Portable Executable",
                extension: "exe",
                mime: "application/vnd.microsoft.portable-executable",
                sig: &[(0, 0x4D), (1, 0x5A)],
            });
            signatures.push(Signature {
                name: "Executable and Linkable Format",
                extension: "elf",
                mime: "application/x-executable",
                sig: &[(0, 0x7F), (1, 0x45), (2, 0x4C), (3, 0x46)],
            });
            signatures.push(Signature {
                name: "Java Class",
                extension: "class",
                mime: "application/java-vm",
                sig: &[(0, 0xCA), (1, 0xFE), (2, 0xBA), (3, 0xBE)],
            });
        }

        if detect_archives {
            signatures.push(Signature {
                name: "PKZIP archive",
                extension: "zip",
                mime: "application/zip",
                sig: &[(0, 0x50), (1, 0x4B), (2, 0x03), (3, 0x04)],
            });
            signatures.push(Signature {
                name: "Gzip",
                extension: "gz",
                mime: "application/gzip",
                sig: &[(0, 0x1F), (1, 0x8B), (2, 0x08)],
            });
            signatures.push(Signature {
                name: "7zip",
                extension: "7z",
                mime: "application/x-7z-compressed",
                sig: &[
                    (0, 0x37),
                    (1, 0x7A),
                    (2, 0xBC),
                    (3, 0xAF),
                    (4, 0x27),
                    (5, 0x1C),
                ],
            });
            signatures.push(Signature {
                name: "Roshal Archive",
                extension: "rar",
                mime: "application/x-rar-compressed",
                sig: &[(0, 0x52), (1, 0x61), (2, 0x72), (3, 0x21)],
            });
        }

        let mut results = Vec::new();
        for signature in signatures {
            let mut matches = true;
            for (offset, byte) in signature.sig {
                if input.get(*offset) != Some(byte) {
                    matches = false;
                    break;
                }
            }
            if matches {
                results.push(format!(
                    "File type:   {}\nExtension:   {}\nMIME type:   {}\n",
                    signature.name, signature.extension, signature.mime
                ));
            }
        }

        if results.is_empty() {
            Ok("Unknown file type.".to_string().into_bytes())
        } else {
            Ok(results.join("\n").into_bytes())
        }
    }
}
