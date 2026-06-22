/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate HOTP operation.
 * -----------------------------------------------------------------------------
 */

use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate HOTP operation
pub struct GenerateHOTPOp;

impl Operation for GenerateHOTPOp {
    fn name(&self) -> &'static str {
        "Generate HOTP"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "The HMAC-based One-Time Password algorithm (HOTP) is an algorithm that computes a one-time password from a shared secret key and an incrementing counter."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Name",
                description: "Label for the HOTP",
                default_value: "",
            },
            ArgSchema {
                name: "Code length",
                description: "Number of digits in the code",
                default_value: "6",
            },
            ArgSchema {
                name: "Counter",
                description: "Counter value",
                default_value: "0",
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
        let name = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let digits = args.get(1).and_then(|a| a.as_usize()).unwrap_or(6);
        let counter = args.get(2).and_then(|a| a.as_i64()).unwrap_or(0);

        let secret = if input.is_empty() {
            Vec::new()
        } else {
            let input_str = String::from_utf8_lossy(&input);
            let cleaned = input_str
                .to_uppercase()
                .replace(|c: char| c.is_whitespace(), "");
            data_encoding::BASE32_NOPAD
                .decode(cleaned.as_bytes())
                .unwrap_or_else(|_| input.clone())
        };

        if secret.is_empty() && !input.is_empty() {
            return Err(OperationError::ProcessingError(
                "Invalid secret".to_string(),
            ));
        }

        let mut hmac = Hmac::<Sha1>::new_from_slice(&secret).map_err(|_| {
            OperationError::ProcessingError("Invalid secret length for HMAC-SHA1".to_string())
        })?;

        hmac.update(&counter.to_be_bytes());
        let result = hmac.finalize().into_bytes();

        let offset = (result[19] & 0xf) as usize;
        let bin_code = (((result[offset] & 0x7f) as u32) << 24)
            | (((result[offset + 1] & 0xff) as u32) << 16)
            | (((result[offset + 2] & 0xff) as u32) << 8)
            | ((result[offset + 3] & 0xff) as u32);

        let code = bin_code % 10u32.pow(digits as u32);
        let code_str = format!("{:0width$}", code, width = digits);

        let secret_b32 = data_encoding::BASE32_NOPAD.encode(&secret);
        let uri = format!(
            "otpauth://hotp/{}?secret={}&counter={}&digits={}&algorithm=SHA1",
            name, secret_b32, counter, digits
        );

        Ok(format!("URI: {}\n\nPassword: {}", uri, code_str).into_bytes())
    }
}
