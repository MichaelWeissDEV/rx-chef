/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Decode text operation.
 * -----------------------------------------------------------------------------
 */

use encoding_rs::Encoding;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

pub const CP500_DEC: [char; 256] = [
    '\0', '\u{0001}', '\u{0002}', '\u{0003}', '\u{009c}', '\t', '\u{0086}', '\u{007f}', '\u{0097}',
    '\u{008d}', '\u{008e}', '\u{000b}', '\u{000c}', '\r', '\u{000e}', '\u{000f}', '\u{0010}',
    '\u{0011}', '\u{0012}', '\u{0013}', '\u{009d}', '\u{0085}', '\u{0008}', '\u{0087}', '\u{0018}',
    '\u{0019}', '\u{0092}', '\u{008f}', '\u{001c}', '\u{001d}', '\u{001e}', '\u{001f}', '\u{0080}',
    '\u{0081}', '\u{0082}', '\u{0083}', '\u{0084}', '\n', '\u{0017}', '\u{001b}', '\u{0088}',
    '\u{0089}', '\u{008a}', '\u{008b}', '\u{008c}', '\u{0005}', '\u{0006}', '\u{0007}', '\u{0090}',
    '\u{0091}', '\u{0016}', '\u{0093}', '\u{0094}', '\u{0095}', '\u{0096}', '\u{0004}', '\u{0098}',
    '\u{0099}', '\u{009a}', '\u{009b}', '\u{0014}', '\u{0015}', '\u{009e}', '\u{001a}', ' ',
    '\u{00a0}', '\u{00e2}', '\u{00e4}', '\u{00e0}', '\u{00e1}', '\u{00e3}', '\u{00e5}', '\u{00e7}',
    '\u{00f1}', '[', '.', '<', '(', '+', '!', '&', '\u{00e9}', '\u{00ea}', '\u{00eb}', '\u{00e8}',
    '\u{00ed}', '\u{00ee}', '\u{00ef}', '\u{00ec}', '\u{00df}', ']', '$', '*', ')', ';', '^', '-',
    '/', '\u{00c2}', '\u{00c4}', '\u{00c0}', '\u{00c1}', '\u{00c3}', '\u{00c5}', '\u{00c7}',
    '\u{00d1}', '\u{00a6}', ',', '%', '_', '>', '?', '\u{00f8}', '\u{00c9}', '\u{00ca}',
    '\u{00cb}', '\u{00c8}', '\u{00cd}', '\u{00ce}', '\u{00cf}', '\u{00cc}', '`', ':', '#', '@',
    '\'', '=', '"', '\u{00d8}', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', '\u{00ab}',
    '\u{00bb}', '\u{00f0}', '\u{00fd}', '\u{00fe}', '\u{00b1}', '\u{00b0}', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', '\u{00aa}', '\u{00ba}', '\u{00e6}', '\u{00b8}', '\u{00c6}',
    '\u{00a4}', '\u{00b5}', '~', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '\u{00a1}', '\u{00bf}',
    '\u{00d0}', '\u{00dd}', '\u{00de}', '\u{00ae}', '\u{00a2}', '\u{00a3}', '\u{00a5}', '\u{00b7}',
    '\u{00a9}', '\u{00a7}', '\u{00b6}', '\u{00bc}', '\u{00bd}', '\u{00be}', '\u{00ac}', '|',
    '\u{00af}', '\u{00a8}', '\u{00b4}', '\u{00d7}', '{', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', '\u{00ad}', '\u{00f4}', '\u{00f6}', '\u{00f2}', '\u{00f3}', '\u{00f5}', '}', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', '\u{00b9}', '\u{00fb}', '\u{00fc}', '\u{00f9}', '\u{00fa}',
    '\u{00ff}', '\\', '\u{00f7}', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '\u{00b2}', '\u{00d4}',
    '\u{00d6}', '\u{00d2}', '\u{00d3}', '\u{00d5}', '0', '1', '2', '3', '4', '5', '6', '7', '8',
    '9', '\u{00b3}', '\u{00db}', '\u{00dc}', '\u{00d9}', '\u{00da}', '\u{009f}',
];

pub struct DecodeText;

impl Operation for DecodeText {
    fn name(&self) -> &'static str {
        "Decode text"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Decodes text from the chosen character encoding."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[ArgSchema {
            name: "Encoding",
            description: "The character encoding to decode from.",
            default_value: "UTF-8 (65001)",
        }]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let enc_name = args[0]
            .as_str()
            .ok_or_else(|| OperationError::InvalidArgument {
                name: "Encoding".to_string(),
                reason: "Encoding must be a string".to_string(),
            })?;

        if enc_name == "IBM EBCDIC International (500)" {
            let mut out = String::with_capacity(input.len());
            for &b in &input {
                out.push(CP500_DEC[b as usize]);
            }
            return Ok(out.into_bytes());
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

        let (decoded, _, _) = encoding.decode(&input);
        Ok(decoded.into_owned().into_bytes())
    }
}
