/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse X.509 certificate operation.
 * -----------------------------------------------------------------------------
 */

use md5::{Digest, Md5};
use sha1::Sha1;
use sha2::Sha256;
use x509_parser::{pem::Pem, prelude::*};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse X.509 certificate operation
pub struct ParseX509Certificate;

impl Operation for ParseX509Certificate {
    fn name(&self) -> &'static str {
        "Parse X.509 certificate"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "X.509 is an ITU-T standard for a public key infrastructure (PKI) and Privilege Management Infrastructure (PMI). It is commonly involved with SSL/TLS security.<br><br>This operation displays the contents of a certificate in a human readable format, similar to the openssl command line tool.<br><br>Tags: X509, server hello, handshake"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "Input format of the certificate",
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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        if input.is_empty() {
            return Ok("No input".to_string().into_bytes());
        }

        let input_format = args.first().and_then(|v| v.as_str()).unwrap_or("PEM");

        let der_data = match input_format {
            "DER Hex" => {
                let s = String::from_utf8_lossy(&input).replace(|c: char| c.is_whitespace(), "");
                hex::decode(s)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
            }
            "PEM" => {
                let pem = Pem::iter_from_buffer(&input)
                    .next()
                    .ok_or_else(|| OperationError::InvalidInput("No PEM block found".to_string()))?
                    .map_err(|e| OperationError::InvalidInput(format!("PEM error: {}", e)))?;
                pem.contents
            }
            "Base64" => {
                let s = String::from_utf8_lossy(&input).replace(|c: char| c.is_whitespace(), "");
                data_encoding::BASE64
                    .decode(s.as_bytes())
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid base64: {}", e)))?
            }
            "Raw" => input,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Input format".to_string(),
                    reason: "Unknown format".to_string(),
                })
            }
        };

        let (_, cert) = X509Certificate::from_der(&der_data)
            .map_err(|e| OperationError::ProcessingError(format!("X509 parse error: {}", e)))?;

        let mut out = String::new();
        out.push_str(&format!(
            "Version:          {} (0x{:x})\n",
            cert.version().0 + 1,
            cert.version().0
        ));
        let serial_hex = hex::encode(cert.raw_serial());
        out.push_str(&format!(
            "Serial number:    {} (0x{})\n",
            cert.serial, serial_hex
        ));
        out.push_str(&format!(
            "Algorithm ID:     {}\n",
            cert.signature_algorithm.algorithm
        ));

        out.push_str("Validity\n");
        out.push_str(&format!(
            "  Not Before:     {} (dd-mm-yyyy hh:mm:ss)\n",
            cert.validity()
                .not_before
                .to_rfc2822()
                .unwrap_or_else(|_| "Invalid".to_string())
        ));
        out.push_str(&format!(
            "  Not After:      {} (dd-mm-yyyy hh:mm:ss)\n",
            cert.validity()
                .not_after
                .to_rfc2822()
                .unwrap_or_else(|_| "Invalid".to_string())
        ));

        out.push_str("Issuer\n");
        out.push_str(&format!("  {}\n", cert.issuer()));

        out.push_str("Subject\n");
        out.push_str(&format!("  {}\n", cert.subject()));

        out.push_str("Fingerprints\n");
        let mut md5 = Md5::new();
        md5.update(&der_data);
        out.push_str(&format!(
            "  MD5:            {}\n",
            hex::encode(md5.finalize())
        ));

        let mut sha1 = Sha1::new();
        sha1.update(&der_data);
        out.push_str(&format!(
            "  SHA1:           {}\n",
            hex::encode(sha1.finalize())
        ));

        let mut sha256 = Sha256::new();
        sha256.update(&der_data);
        out.push_str(&format!(
            "  SHA256:         {}\n",
            hex::encode(sha256.finalize())
        ));

        out.push_str("Public Key\n");
        let pk = cert.public_key();
        out.push_str(&format!("  Algorithm:      {}\n", pk.algorithm.algorithm));
        // Note: Full detail of public key is complex to port exactly as CyberChef due to jsrsasign vs x509-parser differences.
        // We provide the raw key for now.
        out.push_str(&format!(
            "  Raw Key:        {}\n",
            hex::encode(&pk.subject_public_key.data)
        ));

        out.push_str("Certificate Signature\n");
        out.push_str(&format!(
            "  Algorithm:      {}\n",
            cert.signature_algorithm.algorithm
        ));
        out.push_str(&format!(
            "  Signature:      {}\n",
            hex::encode(&cert.signature_value.data)
        ));

        Ok(out.into_bytes())
    }
}
