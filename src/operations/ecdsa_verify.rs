/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ECDSA Verify operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose, Engine as _};
use k256::ecdsa::{Signature as K256Signature, VerifyingKey as K256VerifyingKey};
use p256::{
    ecdsa::{signature::Verifier, Signature as P256Signature, VerifyingKey as P256VerifyingKey},
    pkcs8::DecodePublicKey,
};
use serde::Deserialize;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ECDSA Verify operation
pub struct ECDSAVerify;

#[derive(Deserialize)]
struct SignatureRS {
    r: String,
    s: String,
}

impl Operation for ECDSAVerify {
    fn name(&self) -> &'static str {
        "ECDSA Verify"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Verify a message against a signature and a public PEM encoded EC key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input Format",
                description: "The format of the input signature",
                default_value: "Auto",
            },
            ArgSchema {
                name: "Message Digest Algorithm",
                description: "The hash algorithm to use",
                default_value: "SHA-256",
            },
            ArgSchema {
                name: "ECDSA Public Key (PEM)",
                description: "The PEM encoded ECDSA public key",
                default_value: "-----BEGIN PUBLIC KEY-----",
            },
            ArgSchema {
                name: "Message",
                description: "The message to verify",
                default_value: "",
            },
            ArgSchema {
                name: "Message format",
                description: "The format of the message",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?
            .trim()
            .to_string();

        let input_format = args.first().and_then(|v| v.as_str()).unwrap_or("Auto");
        let _md_algo = args.get(1).and_then(|v| v.as_str()).unwrap_or("SHA-256");
        let key_pem = args.get(2).and_then(|v| v.as_str()).unwrap_or("");
        let msg = args.get(3).and_then(|v| v.as_str()).unwrap_or("");
        let msg_format = args.get(4).and_then(|v| v.as_str()).unwrap_or("Raw");

        if key_pem.is_empty() || key_pem == "-----BEGIN PUBLIC KEY-----" {
            return Err(OperationError::InvalidArgument {
                name: "ECDSA Public Key (PEM)".to_string(),
                reason: "Please enter a public key.".to_string(),
            });
        }

        let message_bytes = convert_to_bytes(msg, msg_format)?;
        let der_signature = convert_to_der(&input_str, input_format)?;

        // Try P-256
        if let Ok(verifying_key) = P256VerifyingKey::from_public_key_pem(key_pem) {
            let sig = P256Signature::from_der(&der_signature).map_err(|e| {
                OperationError::InvalidInput(format!("Invalid signature for P-256: {}", e))
            })?;
            return if verifying_key.verify(&message_bytes, &sig).is_ok() {
                Ok("Verified OK".as_bytes().to_vec())
            } else {
                Ok("Verification Failure".as_bytes().to_vec())
            };
        }

        // Try secp256k1
        if let Ok(verifying_key) = K256VerifyingKey::from_public_key_pem(key_pem) {
            let sig = K256Signature::from_der(&der_signature).map_err(|e| {
                OperationError::InvalidInput(format!("Invalid signature for secp256k1: {}", e))
            })?;
            return if verifying_key.verify(&message_bytes, &sig).is_ok() {
                Ok("Verified OK".as_bytes().to_vec())
            } else {
                Ok("Verification Failure".as_bytes().to_vec())
            };
        }

        Err(OperationError::ProcessingError(
            "Unsupported key or curve. Currently only P-256 and secp256k1 are supported."
                .to_string(),
        ))
    }
}

fn convert_to_bytes(msg: &str, format: &str) -> Result<Vec<u8>, OperationError> {
    match format {
        "Raw" => Ok(msg.as_bytes().to_vec()),
        "Hex" => hex::decode(msg).map_err(|e| OperationError::InvalidArgument {
            name: "Message".to_string(),
            reason: format!("Invalid hex: {}", e),
        }),
        "Base64" => {
            general_purpose::STANDARD
                .decode(msg)
                .map_err(|e| OperationError::InvalidArgument {
                    name: "Message".to_string(),
                    reason: format!("Invalid base64: {}", e),
                })
        }
        _ => Err(OperationError::InvalidArgument {
            name: "Message format".to_string(),
            reason: "Unsupported format".to_string(),
        }),
    }
}

fn convert_to_der(input: &str, format: &str) -> Result<Vec<u8>, OperationError> {
    let mut detected_format = format;
    if format == "Auto" {
        if input.starts_with('{') {
            detected_format = "Raw JSON";
        } else if input.starts_with("30") && hex::decode(input).is_ok() {
            detected_format = "ASN.1 HEX";
        } else if hex::decode(input).is_ok() {
            detected_format = "P1363 HEX";
        } else {
            detected_format = "JSON Web Signature";
        }
    }

    let (r, s) = match detected_format {
        "ASN.1 HEX" => {
            let bytes = hex::decode(input)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?;
            return Ok(bytes);
        }
        "P1363 HEX" => {
            let bytes = hex::decode(input)
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
                .decode(input)
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
            let sig_rs: SignatureRS = serde_json::from_str(input)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid JSON: {}", e)))?;
            (
                hex::decode(&sig_rs.r)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid R hex: {}", e)))?,
                hex::decode(&sig_rs.s)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid S hex: {}", e)))?,
            )
        }
        _ => {
            return Err(OperationError::InvalidInput(
                "Unknown signature format".to_string(),
            ))
        }
    };

    encode_asn1_sig(&r, &s)
}

fn encode_asn1_sig(r: &[u8], s: &[u8]) -> Result<Vec<u8>, OperationError> {
    fn to_der_int(mut val: &[u8]) -> Vec<u8> {
        // Remove leading zeros
        while val.len() > 1 && val[0] == 0 {
            val = &val[1..];
        }
        let mut res = vec![0x02];
        if !val.is_empty() && val[0] & 0x80 != 0 {
            res.push((val.len() + 1) as u8);
            res.push(0x00);
            res.extend_from_slice(val);
        } else {
            res.push(val.len() as u8);
            res.extend_from_slice(val);
        }
        res
    }

    let r_der = to_der_int(r);
    let s_der = to_der_int(s);
    let mut der = vec![0x30];
    let len = r_der.len() + s_der.len();
    der.push(len as u8);
    der.extend(r_der);
    der.extend(s_der);
    Ok(der)
}
