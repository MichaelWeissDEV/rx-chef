/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the JWT Decode operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// JWT Decode operation
///
/// Decodes a JSON Web Token without verifying the signature.
pub struct JWTDecode;

impl Operation for JWTDecode {
    fn name(&self) -> &'static str {
        "JWT Decode"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Decodes a JSON Web Token without checking whether the provided secret / private key is valid. Use JWT Verify to check if the signature is valid as well."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let token = String::from_utf8_lossy(&input).trim().to_string();

        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(OperationError::InvalidInput(
                "JWT must have exactly 3 parts (header.payload.signature)".to_string(),
            ));
        }

        let header_bytes = URL_SAFE_NO_PAD
            .decode(parts[0])
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JWT header: {}", e)))?;
        let payload_bytes = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| OperationError::InvalidInput(format!("Invalid JWT payload: {}", e)))?;

        let header_json: serde_json::Value =
            serde_json::from_slice(&header_bytes).map_err(|e| {
                OperationError::InvalidInput(format!("Header is not valid JSON: {}", e))
            })?;
        let payload_json: serde_json::Value =
            serde_json::from_slice(&payload_bytes).map_err(|e| {
                OperationError::InvalidInput(format!("Payload is not valid JSON: {}", e))
            })?;

        let output = serde_json::json!({
            "header": header_json,
            "payload": payload_json,
            "signature": parts[2]
        });

        let pretty = serde_json::to_string_pretty(&output)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        Ok(pretty.into_bytes())
    }
}
