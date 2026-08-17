/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the LZString Compress operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::HashMap;

use crate::operation::{ArgSchema, ArgValue, Operation, OperationError};

/// LZString Compress operation
pub struct LZStringCompress;

impl LZStringCompress {
    /// Emit one 16-bit code unit of the packed bit stream.
    ///
    /// The LZ-String bit stream is a sequence of UTF-16 code units, and the
    /// reference JavaScript implementation stores them with
    /// `String.fromCharCode`, which accepts unpaired surrogates. A Rust `char`
    /// cannot hold a surrogate, so those values are reported instead of
    /// panicking inside `char::from_u32().unwrap()`.
    fn push_code_unit(res: &mut String, data_val: u32) -> Result<(), OperationError> {
        match std::char::from_u32(data_val) {
            Some(character) => {
                res.push(character);
                Ok(())
            }
            None => Err(OperationError::ProcessingError(format!(
                "LZString compression produced the UTF-16 surrogate code unit U+{data_val:04X}, \
                 which cannot be represented in UTF-8 output"
            ))),
        }
    }

    fn compress(input: &str) -> Result<String, OperationError> {
        if input.is_empty() {
            return Ok(String::new());
        }
        let mut res = String::new();
        let mut dictionary: HashMap<String, u32> = HashMap::new();
        let mut dictionary_to_create: HashMap<String, bool> = HashMap::new();
        let mut c: String;
        let mut wc: String;
        let mut w = String::new();
        let mut enlarge_in = 2.0;
        let mut dict_size = 3;
        let mut num_bits = 2;
        let mut data_val = 0;
        let mut data_position = 0;
        let bits_per_char = 16;

        for character in input.chars() {
            c = character.to_string();
            if !dictionary.contains_key(&c) {
                dictionary.insert(c.clone(), dict_size);
                dict_size += 1;
                dictionary_to_create.insert(c.clone(), true);
            }
            wc = w.clone() + &c;
            if dictionary.contains_key(&wc) {
                w = wc;
            } else {
                if dictionary_to_create.contains_key(&w) {
                    let char_code = w.chars().next().unwrap() as u32;
                    if char_code < 256 {
                        for _ in 0..num_bits {
                            data_val <<= 1;
                            if data_position == bits_per_char - 1 {
                                data_position = 0;
                                Self::push_code_unit(&mut res, data_val)?;
                                data_val = 0;
                            } else {
                                data_position += 1;
                            }
                        }
                        let mut value = char_code;
                        for _ in 0..8 {
                            data_val = (data_val << 1) | (value & 1);
                            if data_position == bits_per_char - 1 {
                                data_position = 0;
                                Self::push_code_unit(&mut res, data_val)?;
                                data_val = 0;
                            } else {
                                data_position += 1;
                            }
                            value >>= 1;
                        }
                    } else {
                        let mut value = 1;
                        for _ in 0..num_bits {
                            data_val = (data_val << 1) | value;
                            if data_position == bits_per_char - 1 {
                                data_position = 0;
                                Self::push_code_unit(&mut res, data_val)?;
                                data_val = 0;
                            } else {
                                data_position += 1;
                            }
                            value = 0;
                        }
                        value = char_code;
                        for _ in 0..16 {
                            data_val = (data_val << 1) | (value & 1);
                            if data_position == bits_per_char - 1 {
                                data_position = 0;
                                Self::push_code_unit(&mut res, data_val)?;
                                data_val = 0;
                            } else {
                                data_position += 1;
                            }
                            value >>= 1;
                        }
                    }
                    enlarge_in -= 1.0;
                    if enlarge_in == 0.0 {
                        enlarge_in = 2.0f64.powi(num_bits as i32);
                        let _ = enlarge_in;
                        num_bits += 1;
                    }
                    dictionary_to_create.remove(&w);
                } else {
                    let mut value = *dictionary.get(&w).unwrap();
                    for _ in 0..num_bits {
                        data_val = (data_val << 1) | (value & 1);
                        if data_position == bits_per_char - 1 {
                            data_position = 0;
                            Self::push_code_unit(&mut res, data_val)?;
                            data_val = 0;
                        } else {
                            data_position += 1;
                        }
                        value >>= 1;
                    }
                }
                enlarge_in -= 1.0;
                if enlarge_in == 0.0 {
                    enlarge_in = 2.0f64.powi(num_bits as i32);
                    let _ = enlarge_in;
                    num_bits += 1;
                }
                dictionary.insert(wc, dict_size);
                dict_size += 1;
                w = c;
            }
        }

        if !w.is_empty() {
            if dictionary_to_create.contains_key(&w) {
                let char_code = w.chars().next().unwrap() as u32;
                if char_code < 256 {
                    for _ in 0..num_bits {
                        data_val <<= 1;
                        if data_position == bits_per_char - 1 {
                            data_position = 0;
                            Self::push_code_unit(&mut res, data_val)?;
                            data_val = 0;
                        } else {
                            data_position += 1;
                        }
                    }
                    let mut value = char_code;
                    for _ in 0..8 {
                        data_val = (data_val << 1) | (value & 1);
                        if data_position == bits_per_char - 1 {
                            data_position = 0;
                            Self::push_code_unit(&mut res, data_val)?;
                            data_val = 0;
                        } else {
                            data_position += 1;
                        }
                        value >>= 1;
                    }
                } else {
                    let mut value = 1;
                    for _ in 0..num_bits {
                        data_val = (data_val << 1) | value;
                        if data_position == bits_per_char - 1 {
                            data_position = 0;
                            Self::push_code_unit(&mut res, data_val)?;
                            data_val = 0;
                        } else {
                            data_position += 1;
                        }
                        value = 0;
                    }
                    value = char_code;
                    for _ in 0..16 {
                        data_val = (data_val << 1) | (value & 1);
                        if data_position == bits_per_char - 1 {
                            data_position = 0;
                            Self::push_code_unit(&mut res, data_val)?;
                            data_val = 0;
                        } else {
                            data_position += 1;
                        }
                        value >>= 1;
                    }
                }
                enlarge_in -= 1.0;
                if enlarge_in == 0.0 {
                    enlarge_in = 2.0f64.powi(num_bits as i32);
                    let _ = enlarge_in;
                    num_bits += 1;
                }
                dictionary_to_create.remove(&w);
            } else {
                let mut value = *dictionary.get(&w).unwrap();
                for _ in 0..num_bits {
                    data_val = (data_val << 1) | (value & 1);
                    if data_position == bits_per_char - 1 {
                        data_position = 0;
                        Self::push_code_unit(&mut res, data_val)?;
                        data_val = 0;
                    } else {
                        data_position += 1;
                    }
                    value >>= 1;
                }
            }
        }

        let mut value = 2;
        for _ in 0..num_bits {
            data_val = (data_val << 1) | (value & 1);
            if data_position == bits_per_char - 1 {
                data_position = 0;
                Self::push_code_unit(&mut res, data_val)?;
                data_val = 0;
            } else {
                data_position += 1;
            }
            value >>= 1;
        }

        loop {
            data_val <<= 1;
            if data_position == bits_per_char - 1 {
                Self::push_code_unit(&mut res, data_val)?;
                break;
            } else {
                data_position += 1;
            }
        }
        Ok(res)
    }
}

impl Operation for LZStringCompress {
    fn name(&self) -> &'static str {
        "LZString Compress"
    }
    fn module(&self) -> &'static str {
        "Compression"
    }
    fn description(&self) -> &'static str {
        "Compress the input with lz-string."
    }
    fn args_schema(&self) -> &'static [ArgSchema] {
        // These choices must stay identical to `LZString Decompress`, so a
        // value that compresses can also be decompressed. Only "Standard" is
        // implemented on this side; the others are declared so the mismatch is
        // reported rather than silently producing unreadable output.
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Compression Format",
            description: "Output format. Only 'Standard' is currently implemented.",
            default_value: "Standard",
            kind: crate::operation::ArgKind::Enum,
            required: false,
            choices: &["Standard", "Base64", "UTF16", "EncodedURIComponent"],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }
    /// Matches upstream CyberChef byte for byte on the recorded
    /// differential case.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(|v| v.as_str()).unwrap_or("Standard");
        // Only the standard 16-bit-per-character stream is implemented. The
        // argument used to be ignored entirely, so requesting Base64 or UTF16
        // silently returned a standard-format result that `LZString
        // Decompress` could not read back.
        match format {
            "Standard" => {}
            "Base64" | "UTF16" | "EncodedURIComponent" => {
                return Err(OperationError::InvalidArgument {
                    name: "Compression Format".to_string(),
                    reason: format!(
                        "the '{format}' output format is not implemented yet; only 'Standard' is available"
                    ),
                })
            }
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Compression Format".to_string(),
                    reason: format!("Unsupported format: {other}"),
                })
            }
        }
        let input_str = String::from_utf8_lossy(&input);
        Ok(Self::compress(&input_str)?.into_bytes())
    }
}
