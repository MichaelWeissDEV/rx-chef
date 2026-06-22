/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the From Punycode operation.
 * -----------------------------------------------------------------------------
 */

use idna::domain_to_unicode;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// From Punycode operation - decodes Punycode to Unicode.
///
/// e.g. "mnchen-3ya" decodes to "muenchen" (with umlaut)
pub struct FromPunycode;

impl Operation for FromPunycode {
    fn name(&self) -> &'static str {
        "From Punycode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Punycode is a way to represent Unicode with the limited character subset of ASCII supported by the Domain Name System. e.g. 'mnchen-3ya' decodes to 'muenchen'."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Internationalised domain name",
            description: "Treat input as a full IDN domain name (xn-- labels)",
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
            // Treat as full domain name - decode each label
            let (unicode, result) = domain_to_unicode(&input_str);
            if result.is_err() {
                return Err(OperationError::ProcessingError(
                    "Failed to decode IDN domain".to_string(),
                ));
            }
            Ok(unicode.into_bytes())
        } else {
            // Treat as raw punycode label (without xn-- prefix)
            // Prepend xn-- so domain_to_unicode can handle it
            let with_prefix = if input_str.starts_with("xn--") {
                input_str.clone()
            } else {
                format!("xn--{}", input_str.trim())
            };
            let (unicode, result) = domain_to_unicode(&with_prefix);
            if result.is_err() {
                return Err(OperationError::ProcessingError(
                    "Failed to decode punycode".to_string(),
                ));
            }
            Ok(unicode.into_bytes())
        }
    }
}
