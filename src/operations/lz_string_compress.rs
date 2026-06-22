/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
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
    fn compress(input: &str) -> String {
        if input.is_empty() {
            return "".to_string();
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
                                res.push(std::char::from_u32(data_val).unwrap());
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
                                res.push(std::char::from_u32(data_val).unwrap());
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
                                res.push(std::char::from_u32(data_val).unwrap());
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
                                res.push(std::char::from_u32(data_val).unwrap());
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
                            res.push(std::char::from_u32(data_val).unwrap());
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
                            res.push(std::char::from_u32(data_val).unwrap());
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
                            res.push(std::char::from_u32(data_val).unwrap());
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
                            res.push(std::char::from_u32(data_val).unwrap());
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
                            res.push(std::char::from_u32(data_val).unwrap());
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
                        res.push(std::char::from_u32(data_val).unwrap());
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
                res.push(std::char::from_u32(data_val).unwrap());
                data_val = 0;
            } else {
                data_position += 1;
            }
            value >>= 1;
        }

        loop {
            data_val <<= 1;
            if data_position == bits_per_char - 1 {
                res.push(std::char::from_u32(data_val).unwrap());
                break;
            } else {
                data_position += 1;
            }
        }
        res
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
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Compression Format",
            description: "default, UTF16, Base64",
            default_value: "default",
        }];
        SCHEMA
    }
    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        Ok(Self::compress(&input_str).into_bytes())
    }
}
