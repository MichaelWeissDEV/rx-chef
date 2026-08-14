/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PGP Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PGP Decrypt operation
///
/// Decrypts a PGP-encrypted message using the recipient's private key.
/// The implementation is available when the `pgp` feature is enabled.
pub struct PGPDecrypt;

impl Operation for PGPDecrypt {
    fn name(&self) -> &'static str {
        "PGP Decrypt"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Decrypts a PGP-encrypted message using the recipient's ASCII-armoured private key. \
         Input: ASCII-armoured PGP message. Arguments: private key and optional passphrase."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Private key of recipient",
                description: "ASCII-armoured PGP private key",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Private key passphrase",
                description: "Passphrase for the private key (leave blank if none)",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn is_broken(&self) -> bool {
        !cfg!(feature = "pgp")
    }

    fn feature_requirements(&self) -> &'static [&'static str] {
        &["pgp"]
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let private_key = args.first().and_then(|v| v.as_str()).unwrap_or("");
        if private_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Private key of recipient".to_string(),
                reason: "Enter the private key of the recipient.".to_string(),
            });
        }
        let password = args.get(1).and_then(ArgValue::as_str).unwrap_or("");
        #[cfg(feature = "pgp")]
        return super::pgp::decrypt(&input, private_key, password, None)
            .map_err(|error| OperationError::ProcessingError(error.to_string()));
        #[cfg(not(feature = "pgp"))]
        {
            let _ = (input, password);
            Err(OperationError::ProcessingError(
                "PGP Decrypt requires --features pgp".to_string(),
            ))
        }
    }
}
