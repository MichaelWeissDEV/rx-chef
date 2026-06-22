/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse X.509 CRL operation.
 * -----------------------------------------------------------------------------
 */

use x509_parser::{pem::Pem, prelude::*};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse X.509 CRL operation
pub struct ParseX509CRL;

impl Operation for ParseX509CRL {
    fn name(&self) -> &'static str {
        "Parse X.509 CRL"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Parse Certificate Revocation List (CRL)"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "Input format of the CRL",
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

        let (_, crl) = CertificateRevocationList::from_der(&der_data)
            .map_err(|e| OperationError::ProcessingError(format!("CRL parse error: {}", e)))?;

        let mut out = String::new();
        out.push_str("Certificate Revocation List (CRL):\n");
        out.push_str(&format!(
            "    Version: {}\n",
            match crl.version() {
                Some(X509Version::V1) => "1 (0x0)",
                Some(X509Version::V2) => "2 (0x1)",
                _ => "Unknown",
            }
        ));
        out.push_str(&format!(
            "    Signature Algorithm: {}\n",
            crl.signature_algorithm.algorithm
        ));
        out.push_str(&format!("    Issuer:\n        {}\n", crl.issuer()));
        out.push_str(&format!(
            "    Last Update: {}\n",
            crl.tbs_cert_list
                .this_update
                .to_rfc2822()
                .unwrap_or_else(|_| "Invalid".to_string())
        ));
        if let Some(next_update) = &crl.tbs_cert_list.next_update {
            out.push_str(&format!(
                "    Next Update: {}\n",
                next_update
                    .to_rfc2822()
                    .unwrap_or_else(|_| "Invalid".to_string())
            ));
        }

        out.push_str("\nRevoked Certificates:\n");
        let revoked = crl.iter_revoked_certificates().collect::<Vec<_>>();
        if revoked.is_empty() {
            out.push_str("    No Revoked Certificates.\n");
        } else {
            for r in revoked {
                out.push_str(&format!(
                    "    Serial Number: {}\n",
                    r.user_certificate.to_str_radix(16).to_uppercase()
                ));
                out.push_str(&format!(
                    "        Revocation Date: {}\n",
                    r.revocation_date
                        .to_rfc2822()
                        .unwrap_or_else(|_| "Invalid".to_string())
                ));
            }
        }

        out.push_str("\nSignature Value:\n");
        out.push_str(&format!("    {}", hex::encode(crl.signature_value.data)));

        Ok(out.into_bytes())
    }
}
