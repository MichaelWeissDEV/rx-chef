/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Microsoft Script Decoder operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Microsoft Script Decoder operation
pub struct MicrosoftScriptDecoder;

impl Operation for MicrosoftScriptDecoder {
    fn name(&self) -> &'static str {
        "Microsoft Script Decoder"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Decodes Microsoft Encoded Script files that have been encoded with Microsoft's custom encoding. These are often VBS (Visual Basic Script) files that are encoded and renamed with a '.vbe' extention or JS (JScript) files renamed with a '.jse' extention."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        let re = Regex::new(r"(?i)#@~\^.{6}==(.+).{6}==\^#~@").unwrap();
        if let Some(caps) = re.captures(&input_str) {
            let encoded_data = &caps[1];
            let decoded = decode(encoded_data);
            Ok(decoded.into_bytes())
        } else {
            Ok(Vec::new())
        }
    }
}

fn decode(data: &str) -> String {
    let data = data
        .replace("@&", "\n")
        .replace("@#", "\r")
        .replace("@*", ">")
        .replace("@!", "<")
        .replace("@$", "@");

    let mut result = String::new();
    let mut index = -1i32;

    for c in data.chars() {
        let byte = c as u32;
        if byte < 128 {
            index += 1;
        }

        if (byte == 9 || (byte > 31 && byte < 128)) && byte != 60 && byte != 62 && byte != 64 {
            let combination_idx = D_COMBINATION[(index % 64) as usize] as usize;
            let decoded_char = D_DECODE[byte as usize]
                .chars()
                .nth(combination_idx)
                .unwrap_or(c);
            result.push(decoded_char);
        } else {
            result.push(c);
        }
    }
    result
}

const D_DECODE: &[&str] = &[
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "\x57\x6E\x7B",
    "\x4A\x4C\x41",
    "\x0B\x0B\x0B",
    "\x0C\x0C\x0C",
    "\x4A\x4C\x41",
    "\x0E\x0E\x0E",
    "\x0F\x0F\x0F",
    "\x10\x10\x10",
    "\x11\x11\x11",
    "\x12\x12\x12",
    "\x13\x13\x13",
    "\x14\x14\x14",
    "\x15\x15\x15",
    "\x16\x16\x16",
    "\x17\x17\x17",
    "\x18\x18\x18",
    "\x19\x19\x19",
    "\x1A\x1A\x1A",
    "\x1B\x1B\x1B",
    "\x1C\x1C\x1C",
    "\x1D\x1D\x1D",
    "\x1E\x1E\x1E",
    "\x1F\x1F\x1F",
    "\x2E\x2D\x32",
    "\x47\x75\x30",
    "\x7A\x52\x21",
    "\x56\x60\x29",
    "\x42\x71\x5B",
    "\x6A\x5E\x38",
    "\x2F\x49\x33",
    "\x26\x5C\x3D",
    "\x49\x62\x58",
    "\x41\x7D\x3A",
    "\x34\x29\x35",
    "\x32\x36\x65",
    "\x5B\x20\x39",
    "\x76\x7C\x5C",
    "\x72\x7A\x56",
    "\x43\x7F\x73",
    "\x38\x6B\x66",
    "\x39\x63\x4E",
    "\x70\x33\x45",
    "\x45\x2B\x6B",
    "\x68\x68\x62",
    "\x71\x51\x59",
    "\x4F\x66\x78",
    "\x09\x76\x5E",
    "\x62\x31\x7D",
    "\x44\x64\x4A",
    "\x23\x54\x6D",
    "\x75\x43\x71",
    "\x4A\x4C\x41",
    "\x7E\x3A\x60",
    "\x4A\x4C\x41",
    "\x5E\x7E\x53",
    "\x40\x4C\x40",
    "\x77\x45\x42",
    "\x4A\x2C\x27",
    "\x61\x2A\x48",
    "\x5D\x74\x72",
    "\x22\x27\x75",
    "\x4B\x37\x31",
    "\x6F\x44\x37",
    "\x4E\x79\x4D",
    "\x3B\x59\x52",
    "\x4C\x2F\x22",
    "\x50\x6F\x54",
    "\x67\x26\x6A",
    "\x2A\x72\x47",
    "\x7D\x6A\x64",
    "\x74\x39\x2D",
    "\x54\x7B\x20",
    "\x2B\x3F\x7F",
    "\x2D\x38\x2E",
    "\x2C\x77\x4C",
    "\x30\x67\x5D",
    "\x6E\x53\x7E",
    "\x6B\x47\x6C",
    "\x66\x34\x6F",
    "\x35\x78\x79",
    "\x25\x5D\x74",
    "\x21\x30\x43",
    "\x64\x23\x26",
    "\x4D\x5A\x76",
    "\x52\x5B\x25",
    "\x63\x6C\x24",
    "\x3F\x48\x2B",
    "\x7B\x55\x28",
    "\x78\x70\x23",
    "\x29\x69\x41",
    "\x28\x2E\x34",
    "\x73\x4C\x09",
    "\x59\x21\x2A",
    "\x33\x24\x44",
    "\x7F\x4E\x3F",
    "\x6D\x50\x77",
    "\x55\x09\x3B",
    "\x53\x56\x55",
    "\x7C\x73\x69",
    "\x3A\x35\x61",
    "\x5F\x61\x63",
    "\x65\x4B\x50",
    "\x46\x58\x67",
    "\x58\x3B\x51",
    "\x31\x57\x49",
    "\x69\x22\x4F",
    "\x6C\x6D\x46",
    "\x5A\x4D\x68",
    "\x48\x25\x7C",
    "\x27\x28\x36",
    "\x5C\x46\x70",
    "\x3D\x4A\x6E",
    "\x24\x32\x7A",
    "\x79\x41\x2F",
    "\x37\x3D\x5F",
    "\x60\x5F\x4B",
    "\x51\x4F\x5A",
    "\x20\x42\x2C",
    "\x36\x65\x57",
];

const D_COMBINATION: &[u8] = &[
    0, 1, 2, 0, 1, 2, 1, 2, 2, 1, 2, 1, 0, 2, 1, 2, 0, 2, 1, 2, 0, 0, 1, 2, 2, 1, 0, 2, 1, 2, 2, 1,
    0, 0, 2, 1, 2, 1, 2, 0, 2, 0, 0, 1, 2, 0, 2, 1, 0, 2, 1, 2, 0, 0, 1, 2, 2, 0, 0, 1, 2, 0, 2, 1,
];
