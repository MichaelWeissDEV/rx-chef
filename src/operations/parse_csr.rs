/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse CSR operation.
 * -----------------------------------------------------------------------------
 */

use x509_cert::{der::Decode, request::CertReq as CertificationRequest};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse CSR operation
pub struct ParseCSR;

impl Operation for ParseCSR {
    fn name(&self) -> &'static str {
        "Parse CSR"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Parse Certificate Signing Request (CSR) for an X.509 certificate"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "Input format",
            default_value: "PEM",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok(b"No input".to_vec());
        }

        let input_str = String::from_utf8_lossy(&input);
        let der_bytes = if input_str.contains("-----BEGIN") {
            // Very basic PEM decoding
            let lines: Vec<&str> = input_str
                .lines()
                .filter(|line| !line.starts_with("-----"))
                .collect();
            let b64 = lines.concat();
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64.trim())
                .map_err(|e| OperationError::InvalidInput(format!("Invalid PEM: {}", e)))?
        } else {
            input
        };

        let csr = CertificationRequest::from_der(&der_bytes)
            .map_err(|e| OperationError::InvalidInput(format!("Failed to parse CSR: {}", e)))?;

        let info = &csr.info;

        let mut output = String::new();
        output.push_str("Subject\n");
        output.push_str(&format!("  {}\n", info.subject));

        output.push_str("Public Key\n");
        output.push_str(&format!(
            "  Algorithm:      {:?}\n",
            info.public_key.algorithm.oid
        ));
        output.push_str(&format!(
            "  Public Key:     {}\n",
            hex::encode(info.public_key.subject_public_key.raw_bytes())
        ));

        output.push_str("Signature\n");
        output.push_str(&format!("  Algorithm:      {:?}\n", csr.algorithm.oid));
        output.push_str(&format!(
            "  Signature:      {}\n",
            hex::encode(csr.signature.raw_bytes())
        ));

        // Requested Extensions
        output.push_str("Requested Extensions\n");
        for attr in info.attributes.iter() {
            output.push_str(&format!("  Attribute:      {:?}\n", attr.oid));
            // In a real implementation we would parse the attribute values (e.g. extensionRequest)
        }

        Ok(output.into_bytes())
    }
}
