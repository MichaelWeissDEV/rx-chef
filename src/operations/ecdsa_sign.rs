/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ECDSA Sign operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose, Engine as _};
use k256::ecdsa::{Signature as K256Signature, SigningKey as K256SigningKey};
use p256::{
    ecdsa::{signature::Signer, Signature as P256Signature, SigningKey as P256SigningKey},
    pkcs8::DecodePrivateKey,
};
use serde::Serialize;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ECDSA Sign operation
pub struct ECDSASign;

#[derive(Serialize)]
struct SignatureRS {
    r: String,
    s: String,
}

impl Operation for ECDSASign {
    fn name(&self) -> &'static str {
        "ECDSA Sign"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Sign a plaintext message with a PEM encoded EC key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "ECDSA Private Key (PEM)",
                description: "The PEM encoded ECDSA private key",
                default_value: "-----BEGIN EC PRIVATE KEY-----",
            },
            ArgSchema {
                name: "Message Digest Algorithm",
                description: "The hash algorithm to use",
                default_value: "SHA-256",
            },
            ArgSchema {
                name: "Output Format",
                description: "The format of the output signature",
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
        let key_pem = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let _md_algo = args.get(1).and_then(|v| v.as_str()).unwrap_or("SHA-256");
        let output_format = args.get(2).and_then(|v| v.as_str()).unwrap_or("ASN.1 HEX");

        if key_pem.is_empty() || key_pem == "-----BEGIN EC PRIVATE KEY-----" {
            return Err(OperationError::InvalidArgument {
                name: "ECDSA Private Key (PEM)".to_string(),
                reason: "Please enter a private key.".to_string(),
            });
        }

        // Try to parse the key for P-256
        if let Ok(signing_key) = P256SigningKey::from_pkcs8_pem(key_pem) {
            let sig: P256Signature = signing_key.sign(&input);
            return format_signature(sig.to_der().as_bytes(), output_format);
        }

        // Try to parse the key for secp256k1
        if let Ok(signing_key) = K256SigningKey::from_pkcs8_pem(key_pem) {
            let sig: K256Signature = signing_key.sign(&input);
            return format_signature(sig.to_der().as_bytes(), output_format);
        }

        // CyberChef supports more curves and hash algorithms.
        // For simplicity, we currently support P-256 and secp256k1 with their default hashes.
        // More complex hash dispatch would be needed for full compatibility.

        Err(OperationError::ProcessingError(
            "Unsupported key or curve. Currently only P-256 and secp256k1 are supported."
                .to_string(),
        ))
    }
}

fn format_signature(der_bytes: &[u8], format: &str) -> Result<Vec<u8>, OperationError> {
    match format {
        "ASN.1 HEX" => Ok(hex::encode(der_bytes).into_bytes()),
        "P1363 HEX" => {
            let (r, s) = decode_asn1_sig(der_bytes)?;
            let mut out = r;
            out.extend_from_slice(&s);
            Ok(hex::encode(out).into_bytes())
        }
        "JSON Web Signature" => {
            let (r, s) = decode_asn1_sig(der_bytes)?;
            let mut out = r;
            out.extend_from_slice(&s);
            Ok(general_purpose::URL_SAFE_NO_PAD.encode(out).into_bytes())
        }
        "Raw JSON" => {
            let (r, s) = decode_asn1_sig(der_bytes)?;
            let sig_rs = SignatureRS {
                r: hex::encode(r),
                s: hex::encode(s),
            };
            serde_json::to_vec(&sig_rs).map_err(|e| OperationError::ProcessingError(e.to_string()))
        }
        _ => Err(OperationError::InvalidArgument {
            name: "Output Format".to_string(),
            reason: "Unsupported format".to_string(),
        }),
    }
}

fn decode_asn1_sig(der_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), OperationError> {
    use der::{asn1::Uint, Decode};
    let sequence = der::asn1::SequenceOf::<Uint, 2>::from_der(der_bytes)
        .map_err(|e| OperationError::InvalidInput(format!("Invalid ASN.1 DER: {}", e)))?;

    let r = sequence.get(0).unwrap().as_bytes().to_vec();
    let s = sequence.get(1).unwrap().as_bytes().to_vec();

    Ok((trim_leading_zeros(&r), trim_leading_zeros(&s)))
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
