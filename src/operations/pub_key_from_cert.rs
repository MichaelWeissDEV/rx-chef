/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Public Key from Certificate operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose, Engine as _};
use x509_cert::{
    der::{Decode, Encode},
    Certificate,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Public Key from Certificate operation
pub struct PubKeyFromCert;

impl Operation for PubKeyFromCert {
    fn name(&self) -> &'static str {
        "Public Key from Certificate"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Extracts the Public Key from a Certificate."
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

        let begin_marker = "-----BEGIN CERTIFICATE-----";
        let end_marker = "-----END CERTIFICATE-----";

        let mut start = 0;
        while let Some(begin_idx) = input_str[start..].find(begin_marker) {
            let actual_begin = start + begin_idx;
            if let Some(end_idx) = input_str[actual_begin..].find(end_marker) {
                let actual_end = actual_begin + end_idx + end_marker.len();
                let pem_content = &input_str[actual_begin..actual_end];

                let base64_data = pem_content
                    .replace(begin_marker, "")
                    .replace(end_marker, "")
                    .replace("\r", "")
                    .replace("\n", "")
                    .replace(" ", "");

                let der = general_purpose::STANDARD.decode(base64_data).map_err(|e| {
                    OperationError::ProcessingError(format!("Invalid base64 in PEM: {}", e))
                })?;

                let cert = Certificate::from_der(&der).map_err(|e| {
                    OperationError::ProcessingError(format!("Failed to parse certificate: {}", e))
                })?;

                let spki_der = cert
                    .tbs_certificate
                    .subject_public_key_info
                    .to_der()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode public key: {}",
                            e
                        ))
                    })?;

                let b64 = general_purpose::STANDARD.encode(spki_der);

                output.push_str("-----BEGIN PUBLIC KEY-----\n");
                for chunk in b64.as_bytes().chunks(64) {
                    output.push_str(std::str::from_utf8(chunk).unwrap());
                    output.push('\n');
                }
                output.push_str("-----END PUBLIC KEY-----\n");

                start = actual_end;
            } else {
                break;
            }
        }

        if output.is_empty() {
            // Try parsing input as raw DER if no PEM markers found
            if let Ok(cert) = Certificate::from_der(&input) {
                let spki_der = cert
                    .tbs_certificate
                    .subject_public_key_info
                    .to_der()
                    .map_err(|e| {
                        OperationError::ProcessingError(format!(
                            "Failed to encode public key: {}",
                            e
                        ))
                    })?;
                let b64 = general_purpose::STANDARD.encode(spki_der);
                output.push_str("-----BEGIN PUBLIC KEY-----\n");
                for chunk in b64.as_bytes().chunks(64) {
                    output.push_str(std::str::from_utf8(chunk).unwrap());
                    output.push('\n');
                }
                output.push_str("-----END PUBLIC KEY-----\n");
            }
        }

        Ok(output.into_bytes())
    }
}
