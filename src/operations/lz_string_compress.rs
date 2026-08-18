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

/// Upstream lz-string's Base64 alphabet. Note it is not the RFC 4648 order:
/// `=` participates as a symbol, not only as padding.
const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

/// Upstream lz-string's URI-safe alphabet.
const URI_SAFE_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";

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
    /// Standard format: one UTF-16 code unit per 16 bits.
    ///
    /// The reference JavaScript stores these with `String.fromCharCode`, which
    /// accepts unpaired surrogates. A Rust `char` cannot hold one, so those
    /// values are reported instead of panicking inside `unwrap`.
    fn standard_char(data_val: u32) -> Result<char, OperationError> {
        std::char::from_u32(data_val).ok_or_else(|| {
            OperationError::ProcessingError(format!(
                "LZString compression produced the UTF-16 surrogate code unit U+{data_val:04X}, \
                 which cannot be represented in UTF-8 output"
            ))
        })
    }

    /// UTF16 format: 15 bits per character, offset by 32 to stay printable.
    fn utf16_char(data_val: u32) -> Result<char, OperationError> {
        std::char::from_u32(data_val + 32).ok_or_else(|| {
            OperationError::ProcessingError(format!(
                "LZString UTF16 compression produced an unrepresentable code unit U+{:04X}",
                data_val + 32
            ))
        })
    }

    /// Index into a 6-bit alphabet, for the Base64 and URI-safe formats.
    fn alphabet_char(alphabet: &str, data_val: u32) -> Result<char, OperationError> {
        alphabet.chars().nth(data_val as usize).ok_or_else(|| {
            OperationError::ProcessingError(format!(
                "LZString compression produced index {data_val}, which is outside the \
                 {}-symbol alphabet",
                alphabet.chars().count()
            ))
        })
    }

    /// The lz-string bit packer, parameterised exactly as upstream's
    /// `_compress(uncompressed, bitsPerChar, getCharFromInt)`.
    ///
    /// The four output formats differ only in how many bits are packed per
    /// output character and how an index is turned into a character; the
    /// dictionary construction is identical. Hardcoding 16 bits here is what
    /// limited this operation to the Standard format.
    fn compress(
        input: &str,
        bits_per_char: u32,
        char_from_int: &dyn Fn(u32) -> Result<char, OperationError>,
    ) -> Result<String, OperationError> {
        // Upstream's `_compress` returns early only for a null input, not for
        // an empty string: an empty input still emits the end-of-stream
        // marker. Short-circuiting here made `LZString Compress("")` return
        // nothing where upstream returns "Q===" (Base64) or "\u2020 " (UTF16).
        let mut res = String::new();
        let mut dictionary: HashMap<Vec<u16>, u32> = HashMap::new();
        let mut dictionary_to_create: HashMap<Vec<u16>, bool> = HashMap::new();
        let mut c: Vec<u16>;
        let mut wc: Vec<u16>;
        let mut w: Vec<u16> = Vec::new();
        let mut enlarge_in = 2.0;
        let mut dict_size = 3;
        let mut num_bits = 2;
        let mut data_val = 0;
        let mut data_position = 0;

        for unit in input.encode_utf16() {
            c = vec![unit];
            if !dictionary.contains_key(&c) {
                dictionary.insert(c.clone(), dict_size);
                dict_size += 1;
                dictionary_to_create.insert(c.clone(), true);
            }
            wc = w.iter().chain(c.iter()).copied().collect::<Vec<u16>>();
            if dictionary.contains_key(&wc) {
                w = wc;
            } else {
                if dictionary_to_create.contains_key(&w) {
                    let char_code = u32::from(w[0]);
                    if char_code < 256 {
                        for _ in 0..num_bits {
                            data_val <<= 1;
                            if data_position == bits_per_char - 1 {
                                data_position = 0;
                                res.push(char_from_int(data_val)?);
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
                                res.push(char_from_int(data_val)?);
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
                                res.push(char_from_int(data_val)?);
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
                                res.push(char_from_int(data_val)?);
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
                            res.push(char_from_int(data_val)?);
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
                let char_code = u32::from(w[0]);
                if char_code < 256 {
                    for _ in 0..num_bits {
                        data_val <<= 1;
                        if data_position == bits_per_char - 1 {
                            data_position = 0;
                            res.push(char_from_int(data_val)?);
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
                            res.push(char_from_int(data_val)?);
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
                            res.push(char_from_int(data_val)?);
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
                            res.push(char_from_int(data_val)?);
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
                        res.push(char_from_int(data_val)?);
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
                res.push(char_from_int(data_val)?);
                data_val = 0;
            } else {
                data_position += 1;
            }
            value >>= 1;
        }

        loop {
            data_val <<= 1;
            if data_position == bits_per_char - 1 {
                res.push(char_from_int(data_val)?);
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
    fn known_limitations(&self) -> &'static [&'static str] {
        &[
            "The Standard format emits raw UTF-16 code units, so compressing text \
             containing astral-plane characters produces lone surrogates that cannot \
             be represented in UTF-8 output and is reported as an error. The UTF16, \
             Base64 and EncodedURIComponent formats are unaffected.",
            "The EncodedURIComponent format is an lz-string feature that upstream \
             CyberChef does not expose, so it has no parity reference.",
        ]
    }

    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(|v| v.as_str()).unwrap_or("Standard");
        let input_str = String::from_utf8_lossy(&input);

        // Each format is upstream's `_compress` with a different bit width and
        // character mapping, plus the small per-format tail lz-string appends.
        let compressed = match format {
            "Standard" => Self::compress(&input_str, 16, &Self::standard_char)?,
            "UTF16" => {
                // compressToUTF16 packs 15 bits per character and terminates
                // the stream with a single space.
                let mut out = Self::compress(&input_str, 15, &Self::utf16_char)?;
                out.push(' ');
                out
            }
            "Base64" => {
                let mut out = Self::compress(&input_str, 6, &|value| {
                    Self::alphabet_char(BASE64_ALPHABET, value)
                })?;
                // compressToBase64 pads to a multiple of four so the result is
                // valid Base64.
                while out.len() % 4 != 0 {
                    out.push('=');
                }
                out
            }
            "EncodedURIComponent" => Self::compress(&input_str, 6, &|value| {
                Self::alphabet_char(URI_SAFE_ALPHABET, value)
            })?,
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Compression Format".to_string(),
                    reason: format!("Unsupported format: {other}"),
                })
            }
        };

        Ok(compressed.into_bytes())
    }
}
