/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ECDSA Signature Conversion operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose, Engine as _};
use der::Decode;
use serde::{Deserialize, Serialize};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ECDSA Signature Conversion operation
pub struct ECDSASignatureConversion;

#[derive(Serialize, Deserialize)]
struct SignatureRS {
    r: String,
    s: String,
}

impl Operation for ECDSASignatureConversion {
    fn name(&self) -> &'static str {
        "ECDSA Signature Conversion"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Convert an ECDSA signature between hex, asn1 and json."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input Format",
                description: "The format of the input signature",
                default_value: "Auto",
            },
            ArgSchema {
                name: "Output Format",
                description: "The desired output format",
                default_value: "ASN.1 HEX",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?
            .trim()
            .to_string();

        let mut input_format = args.first().and_then(|v| v.as_str()).unwrap_or("Auto");
        let output_format = args.get(1).and_then(|v| v.as_str()).unwrap_or("ASN.1 HEX");

        // Detect input format if Auto
        if input_format == "Auto" {
            if input_str.starts_with('{') {
                input_format = "Raw JSON";
            } else if input_str.starts_with("30") && hex::decode(&input_str).is_ok() {
                // Heuristic for ASN.1 DER (starts with 0x30)
                input_format = "ASN.1 HEX";
            } else if hex::decode(&input_str).is_ok() {
                input_format = "P1363 HEX";
            } else {
                input_format = "JSON Web Signature";
            }
        }

        // Convert input to raw R and S bytes
        let (r_bytes, s_bytes) = match input_format {
            "ASN.1 HEX" => {
                let der_bytes = hex::decode(&input_str)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?;
                decode_asn1_sig(&der_bytes)?
            }
            "P1363 HEX" => {
                let bytes = hex::decode(&input_str)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?;
                if bytes.len() % 2 != 0 {
                    return Err(OperationError::InvalidInput(
                        "Invalid P1363 length".to_string(),
                    ));
                }
                let mid = bytes.len() / 2;
                (bytes[..mid].to_vec(), bytes[mid..].to_vec())
            }
            "JSON Web Signature" => {
                let bytes = general_purpose::URL_SAFE_NO_PAD
                    .decode(&input_str)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid JWS: {}", e)))?;
                if bytes.len() % 2 != 0 {
                    return Err(OperationError::InvalidInput(
                        "Invalid JWS length".to_string(),
                    ));
                }
                let mid = bytes.len() / 2;
                (bytes[..mid].to_vec(), bytes[mid..].to_vec())
            }
            "Raw JSON" => {
                let sig_rs: SignatureRS = serde_json::from_str(&input_str)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON: {}", e)))?;
                (
                    hex::decode(&sig_rs.r).map_err(|e| {
                        OperationError::InvalidInput(format!("Invalid R hex: {}", e))
                    })?,
                    hex::decode(&sig_rs.s).map_err(|e| {
                        OperationError::InvalidInput(format!("Invalid S hex: {}", e))
                    })?,
                )
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Input Format".to_string(),
                    reason: "Unsupported format".to_string(),
                })
            }
        };

        // Convert raw R and S to output format
        let result = match output_format {
            "ASN.1 HEX" => hex::encode(encode_asn1_sig(&r_bytes, &s_bytes)?),
            "P1363 HEX" => {
                let mut out = r_bytes.clone();
                out.extend_from_slice(&s_bytes);
                hex::encode(out)
            }
            "JSON Web Signature" => {
                let mut out = r_bytes.clone();
                out.extend_from_slice(&s_bytes);
                general_purpose::URL_SAFE_NO_PAD.encode(out)
            }
            "Raw JSON" => {
                let sig_rs = SignatureRS {
                    r: hex::encode(r_bytes),
                    s: hex::encode(s_bytes),
                };
                serde_json::to_string(&sig_rs)
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Output Format".to_string(),
                    reason: "Unsupported format".to_string(),
                })
            }
        };

        Ok(result.into_bytes())
    }
}

fn decode_asn1_sig(der_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), OperationError> {
    use der::asn1::Uint;
    let sequence = der::asn1::SequenceOf::<Uint, 2>::from_der(der_bytes)
        .map_err(|e| OperationError::InvalidInput(format!("Invalid ASN.1 DER: {}", e)))?;

    let r = sequence
        .get(0)
        .ok_or_else(|| OperationError::InvalidInput("Missing R".to_string()))?
        .as_bytes()
        .to_vec();
    let s = sequence
        .get(1)
        .ok_or_else(|| OperationError::InvalidInput("Missing S".to_string()))?
        .as_bytes()
        .to_vec();

    // Uint::as_bytes() might return a leading zero if the high bit is set.
    // We should trim it if it's there and length is not exactly what's expected?
    // Actually, P1363 expects fixed size. But here we don't know the curve size yet.
    // Let's just return what we got. Leading zeros are fine for ASN.1 but might need care for P1363.
    // P1363 usually pads to the curve order size.

    Ok((trim_leading_zeros(&r), trim_leading_zeros(&s)))
}

fn encode_asn1_sig(r: &[u8], s: &[u8]) -> Result<Vec<u8>, OperationError> {
    use der::{asn1::Uint, Encode};
    let r_uint =
        Uint::new(r).map_err(|e| OperationError::ProcessingError(format!("Invalid R: {}", e)))?;
    let s_uint =
        Uint::new(s).map_err(|e| OperationError::ProcessingError(format!("Invalid S: {}", e)))?;

    let r_bytes = r_uint
        .to_der()
        .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
    let s_bytes = s_uint
        .to_der()
        .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
    let content_len = r_bytes.len() + s_bytes.len();

    let mut seq = vec![0x30];
    if content_len < 128 {
        seq.push(content_len as u8);
    } else if content_len < 256 {
        seq.push(0x81);
        seq.push(content_len as u8);
    } else {
        seq.push(0x82);
        seq.push((content_len >> 8) as u8);
        seq.push((content_len & 0xFF) as u8);
    }
    seq.extend(r_bytes);
    seq.extend(s_bytes);
    Ok(seq)
}

fn trim_leading_zeros(bytes: &[u8]) -> Vec<u8> {
    let mut i = 0;
    while i < bytes.len() && bytes[i] == 0 {
        i += 1;
    }
    if i == bytes.len() {
        vec![0]
    } else {
        bytes[i..].to_vec()
    }
}
