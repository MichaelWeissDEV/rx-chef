/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Flask Session Decode operation.
 * -----------------------------------------------------------------------------
 */

use base64::{engine::general_purpose::URL_SAFE, Engine};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Flask Session Decode operation
///
/// Decodes the payload of a Flask session cookie (itsdangerous) into JSON.
/// Format: base64url(payload) . base64url(timestamp_be32) . base64url(hmac_signature)
/// No key is needed for decoding - just base64url-decode and parse the JSON payload.
pub struct FlaskSessionDecode;

impl Operation for FlaskSessionDecode {
    fn name(&self) -> &'static str {
        "Flask Session Decode"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Decodes the payload of a Flask session cookie (itsdangerous) into JSON."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "View Timestamp",
            description: "Include the timestamp in the output",
            default_value: "false",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let view_timestamp = args.first().and_then(|a| a.as_bool()).unwrap_or(false);

        let input_str = String::from_utf8_lossy(&input).trim().to_string();
        let parts: Vec<&str> = input_str.splitn(3, '.').collect();
        if parts.len() != 3 {
            return Err(OperationError::InvalidInput(
                "Invalid Flask token format. Expected payload.timestamp.signature".to_string(),
            ));
        }

        let payload_b64 = parts[0];
        let time_b64 = parts[1];

        // Decode the timestamp (big-endian 32-bit int, base64url-encoded without padding)
        let time_padded = pad_base64url(time_b64);
        let time_bytes = URL_SAFE.decode(&time_padded).map_err(|e| {
            OperationError::InvalidInput(format!("Invalid timestamp base64: {}", e))
        })?;

        let timestamp = if time_bytes.len() >= 4 {
            i32::from_be_bytes([time_bytes[0], time_bytes[1], time_bytes[2], time_bytes[3]])
        } else {
            return Err(OperationError::InvalidInput(
                "Timestamp bytes too short".to_string(),
            ));
        };

        // Decode the payload
        let payload_padded = pad_base64url(payload_b64);
        let payload_bytes = URL_SAFE
            .decode(&payload_padded)
            .map_err(|_| OperationError::InvalidInput("Invalid Base64 payload".to_string()))?;

        let payload_str = String::from_utf8(payload_bytes).map_err(|e| {
            OperationError::InvalidInput(format!("Payload is not valid UTF-8: {}", e))
        })?;

        let data: serde_json::Value = serde_json::from_str(&payload_str).map_err(|e| {
            OperationError::InvalidInput(format!("Unable to decode JSON payload: {}", e))
        })?;

        let output = if view_timestamp {
            serde_json::json!({ "payload": data, "timestamp": timestamp })
        } else {
            data
        };

        let pretty = serde_json::to_string_pretty(&output)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;

        Ok(pretty.into_bytes())
    }
}

/// Adds padding to a base64url string that may be missing '=' characters.
fn pad_base64url(s: &str) -> String {
    let remainder = s.len() % 4;
    if remainder == 0 {
        s.to_string()
    } else {
        let padding = 4 - remainder;
        format!("{}{}", s, "=".repeat(padding))
    }
}
