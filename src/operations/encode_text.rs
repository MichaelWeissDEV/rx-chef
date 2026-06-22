/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Encode text operation.
 * -----------------------------------------------------------------------------
 */

use encoding_rs::Encoding;

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::decode_text::CP500_DEC,
};

pub struct EncodeText;

impl Operation for EncodeText {
    fn name(&self) -> &'static str {
        "Encode text"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Encodes text into the chosen character encoding."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Encoding",
            description: "The character encoding to encode into.",
            default_value: "UTF-8 (65001)",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        let enc_name = args[0]
            .as_str()
            .ok_or_else(|| OperationError::InvalidArgument {
                name: "Encoding".to_string(),
                reason: "Encoding must be a string".to_string(),
            })?;

        if enc_name == "IBM EBCDIC International (500)" {
            let mut out = Vec::with_capacity(input_str.len());
            for c in input_str.chars() {
                let mut found = false;
                for (i, &dc) in CP500_DEC.iter().enumerate() {
                    if dc == c {
                        out.push(i as u8);
                        found = true;
                        break;
                    }
                }
                if !found {
                    out.push(b'?'); // Replacement character
                }
            }
            return Ok(out);
        }

        let label_part = enc_name.split(" (").next().unwrap_or(enc_name);

        let encoding = Encoding::for_label(label_part.as_bytes())
            .or_else(|| Encoding::for_label(label_part.replace("ISO ", "ISO-").as_bytes()))
            .or_else(|| {
                Encoding::for_label(label_part.replace("US-ASCII", "windows-1252").as_bytes())
            })
            .ok_or_else(|| OperationError::InvalidArgument {
                name: "Encoding".to_string(),
                reason: format!("Unsupported encoding: {}", enc_name),
            })?;

        let (encoded, _, _) = encoding.encode(&input_str);
        Ok(encoded.into_owned())
    }
}
