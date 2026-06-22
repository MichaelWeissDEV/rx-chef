/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Bech32 operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Bech32 operation
pub struct FromBech32;

impl Operation for FromBech32 {
    fn name(&self) -> &'static str {
        "From Bech32"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Bech32 is an encoding scheme primarily used for Bitcoin SegWit addresses (BIP-0173). It uses a 32-character alphabet that excludes easily confused characters (1, b, i, o) and includes a checksum for error detection.\n\nBech32m (BIP-0350) is an updated version used for Bitcoin Taproot addresses.\n\nAuto-detect will attempt Bech32 first, then Bech32m if the checksum fails.\n\nOutput format options allow you to see the Human-Readable Part (HRP) along with the decoded data."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Encoding",
                description: "The Bech32 encoding variant",
                default_value: "Auto-detect",
            },
            ArgSchema {
                name: "Output Format",
                description: "The format of the output",
                default_value: "Raw",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input).trim().to_string();

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let encoding_arg = args
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("Auto-detect");
        let output_format = args.get(1).and_then(|v| v.as_str()).unwrap_or("Raw");

        let decoded = decode_bech32(&input_str, encoding_arg)?;

        match output_format {
            "Raw" => Ok(decoded.data),
            "Hex" => Ok(hex::encode(&decoded.data).into_bytes()),
            "Bitcoin scriptPubKey" => {
                if let Some(version) = decoded.witness_version {
                    let mut script = Vec::new();
                    let op_code = if version == 0 { 0x00 } else { 0x50 + version };
                    script.push(op_code);
                    // Witness program is the data minus the version byte
                    let witness_program = &decoded.data[1..];
                    script.push(witness_program.len() as u8);
                    script.extend_from_slice(witness_program);
                    Ok(hex::encode(script).into_bytes())
                } else {
                    Ok(hex::encode(&decoded.data).into_bytes())
                }
            }
            "HRP: Hex" => {
                let res = format!("{}: {}", decoded.hrp, hex::encode(&decoded.data));
                Ok(res.into_bytes())
            }
            "JSON" => {
                let json = serde_json::json!({
                    "hrp": decoded.hrp,
                    "encoding": decoded.encoding,
                    "data": hex::encode(&decoded.data)
                });
                Ok(serde_json::to_string_pretty(&json).unwrap().into_bytes())
            }
            _ => Ok(hex::encode(&decoded.data).into_bytes()),
        }
    }
}

struct Bech32Decoded {
    hrp: String,
    data: Vec<u8>,
    encoding: String,
    witness_version: Option<u8>,
}

const CHARSET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

fn get_charset_rev(c: char) -> Option<u8> {
    CHARSET.iter().position(|&x| x == c as u8).map(|x| x as u8)
}

fn polymod(values: &[u8]) -> u32 {
    let mut chk: u32 = 1;
    let generator: [u32; 5] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
    for &v in values {
        let top = chk >> 25;
        chk = ((chk & 0x1ffffff) << 5) ^ (v as u32);
        for i in 0..5 {
            if (top >> i) & 1 == 1 {
                chk ^= generator[i];
            }
        }
    }
    chk
}

fn hrp_expand(hrp: &str) -> Vec<u8> {
    let mut res = Vec::new();
    for c in hrp.chars() {
        res.push((c as u8) >> 5);
    }
    res.push(0);
    for c in hrp.chars() {
        res.push((c as u8) & 31);
    }
    res
}

fn verify_checksum(hrp: &str, data: &[u8], encoding: &str) -> bool {
    let constant = if encoding == "Bech32m" { 0x2bc830a3 } else { 1 };
    let mut combined = hrp_expand(hrp);
    combined.extend_from_slice(data);
    polymod(&combined) == constant
}

fn from_words(words: &[u8]) -> Result<Vec<u8>, OperationError> {
    let mut value: u32 = 0;
    let mut bits: u32 = 0;
    let mut res = Vec::new();

    for &w in words {
        value = (value << 5) | (w as u32);
        bits += 5;
        while bits >= 8 {
            bits -= 8;
            res.push(((value >> bits) & 0xff) as u8);
        }
    }

    if bits >= 5 {
        return Err(OperationError::ProcessingError(
            "Invalid padding: too many bits remaining".to_string(),
        ));
    }
    if bits > 0 {
        let padding_value = (value << (8 - bits)) & 0xff;
        if padding_value != 0 {
            return Err(OperationError::ProcessingError(
                "Invalid padding: non-zero bits in padding".to_string(),
            ));
        }
    }

    Ok(res)
}

fn decode_bech32(s: &str, encoding_pref: &str) -> Result<Bech32Decoded, OperationError> {
    if s.len() > 90 {
        return Err(OperationError::InvalidInput(
            "Invalid Bech32 string: exceeds maximum length of 90 characters".to_string(),
        ));
    }

    let has_upper = s.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = s.chars().any(|c| c.is_ascii_lowercase());
    if has_upper && has_lower {
        return Err(OperationError::InvalidInput(
            "Invalid Bech32 string: mixed case is not allowed".to_string(),
        ));
    }

    let s = s.to_lowercase();
    let sep_index = s.rfind('1').ok_or_else(|| {
        OperationError::InvalidInput("Invalid Bech32 string: no separator '1' found".to_string())
    })?;

    if sep_index == 0 {
        return Err(OperationError::InvalidInput(
            "Invalid Bech32 string: Human-Readable Part (HRP) cannot be empty".to_string(),
        ));
    }

    if sep_index + 7 > s.len() {
        return Err(OperationError::InvalidInput(
            "Invalid Bech32 string: data part is too short".to_string(),
        ));
    }

    let hrp = &s[..sep_index];
    let data_part = &s[sep_index + 1..];

    let mut data = Vec::new();
    for c in data_part.chars() {
        let val = get_charset_rev(c).ok_or_else(|| {
            OperationError::InvalidInput(format!("Invalid character '{}' in Bech32 string", c))
        })?;
        data.push(val);
    }

    let used_encoding = match encoding_pref {
        "Bech32" => {
            if !verify_checksum(hrp, &data, "Bech32") {
                return Err(OperationError::ProcessingError(
                    "Invalid Bech32 checksum".to_string(),
                ));
            }
            "Bech32"
        }
        "Bech32m" => {
            if !verify_checksum(hrp, &data, "Bech32m") {
                return Err(OperationError::ProcessingError(
                    "Invalid Bech32m checksum".to_string(),
                ));
            }
            "Bech32m"
        }
        _ => {
            if verify_checksum(hrp, &data, "Bech32") {
                "Bech32"
            } else if verify_checksum(hrp, &data, "Bech32m") {
                "Bech32m"
            } else {
                return Err(OperationError::ProcessingError(
                    "Invalid Bech32/Bech32m string: checksum verification failed".to_string(),
                ));
            }
        }
    };

    let words = &data[..data.len() - 6];

    let segwit_hrps = ["bc", "tb", "ltc", "tltc", "bcrt"];
    let could_be_segwit = segwit_hrps.contains(&hrp) && !words.is_empty() && words[0] <= 16;

    let mut final_bytes;
    let mut witness_version = None;

    if could_be_segwit {
        witness_version = Some(words[0]);
        let program_words = &words[1..];
        match from_words(program_words) {
            Ok(program_bytes) => {
                let valid_v0 = witness_version == Some(0)
                    && (program_bytes.len() == 20 || program_bytes.len() == 32);
                let valid_other = witness_version != Some(0)
                    && program_bytes.len() >= 2
                    && program_bytes.len() <= 40;

                if valid_v0 || valid_other {
                    final_bytes = vec![words[0]];
                    final_bytes.extend(program_bytes);
                } else {
                    witness_version = None;
                    final_bytes = from_words(words)?;
                }
            }
            Err(_) => {
                witness_version = None;
                final_bytes = from_words(words)?;
            }
        }
    } else {
        final_bytes = from_words(words)?;
    }

    Ok(Bech32Decoded {
        hrp: hrp.to_string(),
        data: final_bytes,
        encoding: used_encoding.to_string(),
        witness_version,
    })
}
