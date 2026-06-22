/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Punycode operation.
 * -----------------------------------------------------------------------------
 */

use idna::domain_to_ascii;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// To Punycode operation - encodes Unicode to Punycode.
///
/// e.g. "muenchen" (with umlaut) encodes to "mnchen-3ya"
pub struct ToPunycode;

impl Operation for ToPunycode {
    fn name(&self) -> &'static str {
        "To Punycode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Punycode is a way to represent Unicode with the limited character subset of ASCII supported by the Domain Name System."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Internationalised domain name",
            description: "Treat input as a full IDN domain name",
            default_value: "false",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        let idn = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        if idn {
            let ascii = domain_to_ascii(&input_str).map_err(|e| {
                OperationError::ProcessingError(format!("Failed to encode as IDN: {:?}", e))
            })?;
            Ok(ascii.into_bytes())
        } else {
            // Encode as a single label
            let ascii = domain_to_ascii(input_str.trim()).map_err(|e| {
                OperationError::ProcessingError(format!("Failed to encode as punycode: {:?}", e))
            })?;
            // Strip the xn-- prefix to match CyberChef's raw punycode behaviour
            let result = if ascii.starts_with("xn--") {
                ascii[4..].to_string()
            } else {
                ascii
            };
            Ok(result.into_bytes())
        }
    }
}
