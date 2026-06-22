/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LZString Decompress operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// LZString Decompress operation
pub struct LZStringDecompress;

impl Operation for LZStringDecompress {
    fn name(&self) -> &'static str {
        "LZString Decompress"
    }

    fn module(&self) -> &'static str {
        "Compression"
    }

    fn description(&self) -> &'static str {
        "Decompresses data that was compressed with lz-string."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Compression Format",
            description: "The format the data was compressed in.",
            default_value: "Standard",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(|v| v.as_str()).unwrap_or("Standard");
        let input_str = String::from_utf8_lossy(&input);

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let result = match format {
            "Standard" => decompress(&input_str),
            "Base64" => decompress_from_base64(&input_str),
            "UTF16" => decompress_from_utf16(&input_str),
            "EncodedURIComponent" => decompress_from_encoded_uri_component(&input_str),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Compression Format".to_string(),
                    reason: format!("Unsupported format: {}", format),
                })
            }
        };

        match result {
            Some(s) => Ok(s.into_bytes()),
            None => Err(OperationError::ProcessingError(
                "LZString decompression failed".to_string(),
            )),
        }
    }
}

fn decompress_from_base64(input: &str) -> Option<String> {
    let key_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    let input = input.trim_end_matches('=');
    decompress_generic(input.len(), 32, |idx| {
        let c = input.chars().nth(idx)?;
        key_str.find(c).map(|pos| pos as u32)
    })
}

fn decompress_from_utf16(input: &str) -> Option<String> {
    decompress_generic(input.len(), 16384, |idx| {
        input.chars().nth(idx).map(|c| (c as u32).wrapping_sub(32))
    })
}

fn decompress_from_encoded_uri_component(input: &str) -> Option<String> {
    let input = input.replace(' ', "+");
    let key_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";
    decompress_generic(input.len(), 32, |idx| {
        let c = input.chars().nth(idx)?;
        key_str.find(c).map(|pos| pos as u32)
    })
}

fn decompress(input: &str) -> Option<String> {
    decompress_generic(input.len(), 32768, |idx| {
        input.chars().nth(idx).map(|c| c as u32)
    })
}

fn decompress_generic<F>(length: usize, reset_value: u32, mut get_next_value: F) -> Option<String>
where
    F: FnMut(usize) -> Option<u32>,
{
    let mut dictionary: Vec<String> = Vec::new();
    for i in 0..3 {
        dictionary.push(i.to_string());
    }

    let mut data_val = get_next_value(0).unwrap_or(0);
    let mut data_position = reset_value;
    let mut data_index = 1;

    let mut read_bits = |n: u32, val: &mut u32, pos: &mut u32, idx: &mut usize| -> u32 {
        let mut res = 0;
        let mut power = 1;
        for _ in 0..n {
            let bit = *val & *pos;
            *pos >>= 1;
            if *pos == 0 {
                *pos = reset_value;
                *val = get_next_value(*idx).unwrap_or(0);
                *idx += 1;
            }
            if bit > 0 {
                res |= power;
            }
            power <<= 1;
        }
        res
    };

    let bits = read_bits(2, &mut data_val, &mut data_position, &mut data_index);
    let first_c = match bits {
        0 => read_bits(8, &mut data_val, &mut data_position, &mut data_index),
        1 => read_bits(16, &mut data_val, &mut data_position, &mut data_index),
        2 => return Some("".to_string()),
        _ => return None,
    };

    let c_str = std::char::from_u32(first_c)?.to_string();
    dictionary.push(c_str.clone());
    let mut w = c_str.clone();
    let mut result = c_str;

    let mut enlarge_in = 4;
    let mut num_bits = 3;

    loop {
        if data_index > length + 2 {
            return Some(result);
        }

        let cc = read_bits(num_bits, &mut data_val, &mut data_position, &mut data_index);
        let mut current_c = cc;

        match current_c {
            0 => {
                let val = read_bits(8, &mut data_val, &mut data_position, &mut data_index);
                let s = std::char::from_u32(val)?.to_string();
                dictionary.push(s);
                current_c = (dictionary.len() - 1) as u32;
                enlarge_in -= 1;
            }
            1 => {
                let val = read_bits(16, &mut data_val, &mut data_position, &mut data_index);
                let s = std::char::from_u32(val)?.to_string();
                dictionary.push(s);
                current_c = (dictionary.len() - 1) as u32;
                enlarge_in -= 1;
            }
            2 => return Some(result),
            _ => {}
        }

        if enlarge_in == 0 {
            enlarge_in = 2u32.pow(num_bits);
            num_bits += 1;
        }

        let entry = if (current_c as usize) < dictionary.len() {
            dictionary[current_c as usize].clone()
        } else if current_c == (dictionary.len()) as u32 {
            w.clone() + &w.chars().next().unwrap().to_string()
        } else {
            return None;
        };

        result.push_str(&entry);
        dictionary.push(w.clone() + &entry.chars().next().unwrap().to_string());
        enlarge_in -= 1;
        w = entry;

        if enlarge_in == 0 {
            enlarge_in = 2u32.pow(num_bits);
            num_bits += 1;
        }
    }
}
