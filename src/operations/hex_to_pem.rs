/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Hex to PEM operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::STANDARD, Engine};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Hex to PEM operation
///
/// Converts a hexadecimal DER string into PEM (Privacy Enhanced Mail) format.
pub struct HexToPEM;

impl Operation for HexToPEM {
    fn name(&self) -> &'static str {
        "Hex to PEM"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Converts a hexadecimal DER (Distinguished Encoding Rules) string into PEM (Privacy Enhanced Mail) format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Header string",
            description: "PEM header/footer label (e.g. CERTIFICATE, RSA PRIVATE KEY)",
            default_value: "CERTIFICATE",
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
        let header = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("CERTIFICATE");

        let hex_str = String::from_utf8_lossy(&input);
        let hex_clean: String = hex_str.chars().filter(|c| !c.is_whitespace()).collect();

        let der_bytes = hex::decode(&hex_clean)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?;

        let b64 = STANDARD.encode(&der_bytes);
        // Wrap at 64 characters per line
        let mut pem = format!("-----BEGIN {}-----\n", header);
        for chunk in b64.as_bytes().chunks(64) {
            pem.push_str(std::str::from_utf8(chunk).unwrap_or(""));
            pem.push('\n');
        }
        pem.push_str(&format!("-----END {}-----\n", header));

        Ok(pem.into_bytes())
    }
}
