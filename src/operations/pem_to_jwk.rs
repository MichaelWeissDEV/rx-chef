/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PEM to JWK operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rsa::{pkcs8::DecodePublicKey, traits::PublicKeyParts};
use serde_json::{json, Value};
use x509_parser::prelude::*;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PEM to JWK operation
pub struct PEMToJWK;

impl Operation for PEMToJWK {
    fn name(&self) -> &'static str {
        "PEM to JWK"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Converts Keys in PEM format to a JSON Web Key format."
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
        let mut output = String::new();

        let pem_regex = regex::Regex::new(r"-----BEGIN (?P<type>[A-Z][A-Z ]+[A-Z])-----\s*(?P<data>[^-]+)-----END (?P<type2>[A-Z][A-Z ]+[A-Z])-----").unwrap();

        for cap in pem_regex.captures_iter(&input_str) {
            let pem_type = &cap["type"];
            let data_str = &cap["data"].replace(|c: char| c.is_whitespace(), "");
            let data = base64::engine::general_purpose::STANDARD
                .decode(data_str)
                .map_err(|_| {
                    OperationError::ProcessingError("Failed to decode base64 in PEM".to_string())
                })?;

            let jwk = if pem_type.contains("CERTIFICATE") {
                let (_, cert) = X509Certificate::from_der(&data).map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to parse certificate: {}", e))
                })?;
                key_to_jwk(cert.public_key().raw)?
            } else if pem_type.contains("PUBLIC KEY") {
                key_to_jwk(&data)?
            } else if pem_type.contains("PRIVATE KEY") {
                // Private key parsing is more complex and depends on PKCS#1 or PKCS#8
                // For now, let's try to support public key from private key if possible,
                // but CyberChef says "Only PKCS#8 is supported" for RSA.
                // We'll skip private for now or implement if easy.
                return Err(OperationError::ProcessingError(
                    "Private key to JWK not fully supported yet".to_string(),
                ));
            } else {
                continue;
            };

            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&serde_json::to_string(&jwk).unwrap());
        }

        Ok(output.into_bytes())
    }
}

fn key_to_jwk(key_der: &[u8]) -> Result<Value, OperationError> {
    // Try to parse as SubjectPublicKeyInfo
    let (_, spki) = SubjectPublicKeyInfo::from_der(key_der)
        .map_err(|e| OperationError::ProcessingError(format!("Failed to parse SPKI: {}", e)))?;

    let oid = spki.algorithm.algorithm.to_string();

    // RSA: 1.2.840.113549.1.1.1
    if oid == "1.2.840.113549.1.1.1" {
        let rsa_pub = rsa::RsaPublicKey::from_public_key_der(key_der).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to parse RSA key: {}", e))
        })?;

        return Ok(json!({
            "kty": "RSA",
            "n": URL_SAFE_NO_PAD.encode(rsa_pub.n().to_bytes_be()),
            "e": URL_SAFE_NO_PAD.encode(rsa_pub.e().to_bytes_be()),
        }));
    }

    // EC: 1.2.840.10045.2.1
    if oid == "1.2.840.10045.2.1" {
        // EC keys are more complex as we need the curve.
        // For simplicity, we'll just say it's an EC key but we might not have the curve easily from OID here.
        return Err(OperationError::ProcessingError(
            "EC keys to JWK support pending more robust OID handling".to_string(),
        ));
    }

    Err(OperationError::ProcessingError(format!(
        "Unsupported key algorithm OID: {}",
        oid
    )))
}
