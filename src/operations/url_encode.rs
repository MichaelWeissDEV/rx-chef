/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the URL Encode operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// URL Encode operation
pub struct URLEncode;

// Standard encodeURI set (don't encode these)
// A-Z a-z 0-9 ; , / ? : @ & = + $ - _ . ! ~ * ' ( ) #
const ENCODE_URI_EXCEPTIONS: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789;,/?:@&=+$-_.!~*'()#";

// Standard encodeURIComponent set (don't encode these)
// A-Z a-z 0-9 - _ . ! ~ * ' ( )
const _ENCODE_URI_COMPONENT_EXCEPTIONS: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.!~*'()";

impl Operation for URLEncode {
    fn name(&self) -> &'static str {
        "URL Encode"
    }

    fn module(&self) -> &'static str {
        "URL"
    }

    fn description(&self) -> &'static str {
        "Encodes problematic characters into percent-encoding, a format supported by URIs/URLs.<br><br>e.g. <code>=</code> becomes <code>%3d</code>"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Encode all special chars",
            description: "Encode all characters including those usually allowed in URLs",
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
        let encode_all = args.first().and_then(|v| v.as_bool()).unwrap_or(false);

        let result = if encode_all {
            // Encode everything except A-Z a-z 0-9
            let mut encoded = String::new();
            for b in input {
                if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || (b >= b'0' && b <= b'9')
                {
                    encoded.push(b as char);
                } else {
                    encoded.push_str(&format!("%{:02x}", b));
                }
            }
            encoded
        } else {
            // JS encodeURI behavior
            let mut encoded = String::new();
            for b in input {
                if ENCODE_URI_EXCEPTIONS.contains(&b) {
                    encoded.push(b as char);
                } else {
                    encoded.push_str(&format!("%{:02x}", b));
                }
            }
            encoded
        };

        Ok(result.into_bytes())
    }
}
