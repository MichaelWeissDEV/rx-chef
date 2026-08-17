/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the To Punycode operation.
 * -----------------------------------------------------------------------------
 */

use idna::{domain_to_ascii, punycode};

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
            kind: crate::operation::ArgKind::Boolean,
            required: false,
            choices: &[],
            minimum: None,
            maximum: None,
            sensitive: false,
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    /// Matches upstream CyberChef byte for byte on the recorded
    /// differential case.
    fn parity(&self) -> crate::operation::ParityStatus {
        crate::operation::ParityStatus::Exact
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
            // Encode as a single raw punycode label.
            //
            // This previously went through `domain_to_ascii` and stripped the
            // `xn--` prefix. For all-ASCII input that function returns the
            // string unchanged, so no delimiter was emitted and the result was
            // not decodable: "foobar" encoded to "foobar", which `From
            // Punycode` then decoded as an extended sequence and turned into
            // unrelated characters. RFC 3492 section 6.3 requires the literal
            // portion to be terminated by the delimiter, so "foobar" must
            // encode to "foobar-".
            let result = punycode::encode_str(input_str.trim()).ok_or_else(|| {
                OperationError::ProcessingError("Failed to encode as punycode".to_string())
            })?;
            Ok(result.into_bytes())
        }
    }
}
