/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PGP Verify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PGP Verify operation
///
/// Verifies a PGP-signed message using the signer's public key.
/// The implementation is available when the `pgp` feature is enabled.
pub struct PGPVerify;

impl Operation for PGPVerify {
    fn name(&self) -> &'static str {
        "PGP Verify"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Verifies a PGP clearsigned or signed+encrypted message using the signer's public key. \
         Input: ASCII-armoured signed PGP message. Arguments: public key of the signer."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Public key of signer",
            description: "ASCII-armoured PGP public key of the signer",
            default_value: "",
            kind: crate::operation::ArgKind::String,
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

    fn is_broken(&self) -> bool {
        !cfg!(feature = "pgp")
    }

    fn feature_requirements(&self) -> &'static [&'static str] {
        &["pgp"]
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let public_key = args.first().and_then(|v| v.as_str()).unwrap_or("");
        if public_key.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "Public key of signer".to_string(),
                reason: "Enter the public key of the signer.".to_string(),
            });
        }
        #[cfg(feature = "pgp")]
        return super::pgp::verify(&input, public_key)
            .map_err(|error| OperationError::ProcessingError(error.to_string()));
        #[cfg(not(feature = "pgp"))]
        {
            let _ = input;
            Err(OperationError::ProcessingError(
                "PGP Verify requires --features pgp".to_string(),
            ))
        }
    }
}
