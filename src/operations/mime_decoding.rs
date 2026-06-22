/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the MIME Decoding operation.
 * -----------------------------------------------------------------------------
 */

use encoding_rs::Encoding;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// MIME Decoding operation
pub struct MIMEDecoding;

impl Operation for MIMEDecoding {
    fn name(&self) -> &'static str {
        "MIME Decoding"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Enables the decoding of MIME message header extensions for non-ASCII text"
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
        let input_str = String::from_utf8_lossy(&input).replace("\r\n", "\n");
        let decoded = decode_headers(&input_str)?;
        Ok(decoded.into_bytes())
    }
}

fn decode_headers(header_string: &str) -> Result<String, OperationError> {
    if !header_string.contains("=?") {
        return Ok(header_string.to_string());
    }

    let mut decoded_headers = String::new();

    // RFC 2047: =?charset?encoding?encoded-text?=
    let re = Regex::new(r"(?i)=\?([^?]+)\?([BQ])\?([^?]*)\?=").unwrap();

    let mut last_end = 0;
    let mut is_between_words = false;

    for caps in re.captures_iter(header_string) {
        let full_match = caps.get(0).unwrap();
        let start = full_match.start();
        let end = full_match.end();

        let charset = &caps[1];
        let encoding = &caps[2].to_lowercase();
        let encoded_text = &caps[3];

        // Add text before the match
        let before = &header_string[last_end..start];
        if !is_between_words || before.trim().len() > 0 {
            decoded_headers.push_str(before);
        }

        let decoded_bytes = if encoding == "b" {
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded_text)
                .map_err(|_| {
                    OperationError::ProcessingError("Invalid Base64 in MIME header".to_string())
                })?
        } else {
            // Q-encoding is similar to quoted-printable but with some differences
            // In Q encoding, '_' represents ' '
            let q_text = encoded_text.replace('_', " ");
            quoted_printable::decode(q_text, quoted_printable::ParseMode::Robust).map_err(|_| {
                OperationError::ProcessingError(
                    "Invalid Quoted-Printable in MIME header".to_string(),
                )
            })?
        };

        let charset_encoding = Encoding::for_label(charset.as_bytes()).ok_or_else(|| {
            OperationError::ProcessingError(format!("Unsupported charset: {}", charset))
        })?;

        let (decoded_text, _, _) = charset_encoding.decode(&decoded_bytes);
        decoded_headers.push_str(&decoded_text);

        last_end = end;
        is_between_words = true;
    }

    decoded_headers.push_str(&header_string[last_end..]);

    Ok(decoded_headers)
}
