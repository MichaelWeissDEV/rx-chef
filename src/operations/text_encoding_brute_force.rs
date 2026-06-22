/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Text Encoding Brute Force operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::BTreeMap;

use encoding_rs::*;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Text Encoding Brute Force operation
pub struct TextEncodingBruteForce;

impl Operation for TextEncodingBruteForce {
    fn name(&self) -> &'static str {
        "Text Encoding Brute Force"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Enumerates all supported text encodings for the input, allowing you to quickly spot the correct one."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Mode",
            description: "Mode (Encode or Decode)",
            default_value: "Decode",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mode = args.first().and_then(|a| a.as_str()).unwrap_or("Decode");
        let mut results = BTreeMap::new();

        let encodings = [
            (UTF_8, "UTF-8"),
            (UTF_16LE, "UTF-16LE"),
            (UTF_16BE, "UTF-16BE"),
            (WINDOWS_1252, "Windows-1252"),
            (ISO_8859_2, "ISO-8859-2"),
            (ISO_8859_3, "ISO-8859-3"),
            (ISO_8859_4, "ISO-8859-4"),
            (ISO_8859_5, "ISO-8859-5"),
            (ISO_8859_6, "ISO-8859-6"),
            (ISO_8859_7, "ISO-8859-7"),
            (ISO_8859_8, "ISO-8859-8"),
            (ISO_8859_10, "ISO-8859-10"),
            (ISO_8859_13, "ISO-8859-13"),
            (ISO_8859_14, "ISO-8859-14"),
            (ISO_8859_15, "ISO-8859-15"),
            (ISO_8859_16, "ISO-8859-16"),
            (WINDOWS_1250, "Windows-1250"),
            (WINDOWS_1251, "Windows-1251"),
            (WINDOWS_1253, "Windows-1253"),
            (WINDOWS_1254, "Windows-1254"),
            (WINDOWS_1255, "Windows-1255"),
            (WINDOWS_1256, "Windows-1256"),
            (WINDOWS_1257, "Windows-1257"),
            (WINDOWS_1258, "Windows-1258"),
            (KOI8_R, "KOI8-R"),
            (KOI8_U, "KOI8-U"),
            (SHIFT_JIS, "Shift-JIS"),
            (EUC_JP, "EUC-JP"),
            (EUC_KR, "EUC-KR"),
            (GBK, "GBK"),
            (BIG5, "Big5"),
        ];

        for (enc, name) in encodings {
            if mode == "Decode" {
                let (decoded, _, malformed) = enc.decode(&input);
                if malformed {
                    results.insert(name.to_string(), "Malformed data".to_string());
                } else {
                    results.insert(name.to_string(), decoded.into_owned());
                }
            } else {
                let input_str = String::from_utf8_lossy(&input);
                let (encoded, _, malformed) = enc.encode(&input_str);
                if malformed {
                    results.insert(name.to_string(), "Could not encode".to_string());
                } else {
                    // For Encode mode, we return the encoded bytes as a hex string or something?
                    // CyberChef returns Utils.arrayBufferToStr(cptable.utils.encode(...))
                    // Utils.arrayBufferToStr converts bytes to a string where each char is a byte value.
                    // Let's use hex for better readability in JSON if it's not valid UTF-8.
                    // Actually CyberChef just treats them as chars.
                    let encoded_str: String = encoded.iter().map(|&b| b as char).collect();
                    results.insert(name.to_string(), encoded_str);
                }
            }
        }

        serde_json::to_vec(&results).map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}
