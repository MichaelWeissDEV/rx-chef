/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate TOTP operation.
 * -----------------------------------------------------------------------------
 */

use std::time::{SystemTime, UNIX_EPOCH};

use totp_lite::{totp_custom, Sha1};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate TOTP operation
///
/// The Time-based One-Time Password algorithm (TOTP) computes a one-time password
/// from a shared secret key and the current time.
pub struct GenerateTOTP;

impl Operation for GenerateTOTP {
    fn name(&self) -> &'static str {
        "Generate TOTP"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "The Time-based One-Time Password algorithm (TOTP) is an algorithm that computes a one-time password from a shared secret key and the current time. It has been adopted as Internet Engineering Task Force standard RFC 6238, is the cornerstone of Initiative For Open Authentication (OAUTH), and is used in a number of two-factor authentication systems. A TOTP is an HOTP where the counter is the current time.\n\nEnter the secret as the input or leave it blank for a random secret to be generated. T0 and T1 are in seconds."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Name",
                description: "The name of the account",
                default_value: "",
            },
            ArgSchema {
                name: "Code length",
                description: "The number of digits in the generated code",
                default_value: "6",
            },
            ArgSchema {
                name: "Epoch offset (T0)",
                description: "The epoch offset in seconds",
                default_value: "0",
            },
            ArgSchema {
                name: "Interval (T1)",
                description: "The time interval in seconds",
                default_value: "30",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let name = args.first().and_then(|v| v.as_str()).unwrap_or("");
        let digits = args.get(1).and_then(|v| v.as_f64()).unwrap_or(6.0) as usize;
        let t0 = args.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0) as u64;
        let t1 = args.get(3).and_then(|v| v.as_f64()).unwrap_or(30.0) as u64;

        let secret_str = String::from_utf8_lossy(&input)
            .trim()
            .to_uppercase()
            .replace(' ', "");

        let secret = if secret_str.is_empty() {
            return Err(OperationError::InvalidInput(
                "Secret is required for TOTP generation in this implementation".to_string(),
            ));
        } else {
            match data_encoding::BASE32.decode(secret_str.as_bytes()) {
                Ok(s) => s,
                Err(_) => {
                    return Err(OperationError::InvalidInput(
                        "Invalid Base32 secret".to_string(),
                    ))
                }
            }
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| OperationError::ProcessingError("Time went backwards".to_string()))?
            .as_secs();

        // totp_custom in this dependency expects 4 arguments: step, digits, secret, time.
        let adjusted_time = if now >= t0 { now - t0 } else { 0 };
        let password = totp_custom::<Sha1>(t1, digits as u32, &secret, adjusted_time);

        // Simple manual URI construction as we don't have a URI library handy
        // and we want to avoid extra dependencies.
        let uri = format!(
            "otpauth://totp/{}?secret={}&algorithm=SHA1&digits={}&period={}",
            name.replace(' ', "%20"),
            secret_str,
            digits,
            t1
        );

        let output = format!("URI: {}\n\nPassword: {}", uri, password);
        Ok(output.into_bytes())
    }
}
