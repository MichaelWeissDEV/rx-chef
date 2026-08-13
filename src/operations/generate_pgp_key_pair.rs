/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate PGP Key Pair operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate PGP Key Pair operation
///
/// Generates a new public/private PGP key pair. Supports RSA and ECC key types.
/// The implementation is available when the `pgp` feature is enabled.
pub struct GeneratePGPKeyPair;

impl Operation for GeneratePGPKeyPair {
    fn name(&self) -> &'static str {
        "Generate PGP Key Pair"
    }

    fn module(&self) -> &'static str {
        "PGP"
    }

    fn description(&self) -> &'static str {
        "Generates a new public/private PGP key pair. \
         Supports RSA (1024/2048/4096) and ECC (256/384/521) key types. \
         Arguments: key type, optional password, optional name, optional email."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key type",
                description:
                    "Key type and size: RSA-2048, RSA-4096, ECC-256, ECC-384, ECC-521 (RSA-1024 is rejected as insecure)",
                default_value: "RSA-2048",
            },
            ArgSchema {
                name: "Password (optional)",
                description: "Passphrase to protect the private key",
                default_value: "",
            },
            ArgSchema {
                name: "Name (optional)",
                description: "User name for the key identity",
                default_value: "",
            },
            ArgSchema {
                name: "Email (optional)",
                description: "User email for the key identity",
                default_value: "",
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

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        #[cfg(feature = "pgp")]
        {
            let key_type = args
                .first()
                .and_then(ArgValue::as_str)
                .unwrap_or("RSA-2048");
            let password = args.get(1).and_then(ArgValue::as_str).unwrap_or("");
            let name = args.get(2).and_then(ArgValue::as_str).unwrap_or("");
            let email = args.get(3).and_then(ArgValue::as_str).unwrap_or("");
            let (public, private) = super::pgp::generate(key_type, password, name, email)
                .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
            return serde_json::to_vec_pretty(&serde_json::json!({
                "publicKey": String::from_utf8_lossy(&public),
                "privateKey": String::from_utf8_lossy(&private)
            }))
            .map_err(|error| OperationError::ProcessingError(error.to_string()));
        }
        #[cfg(not(feature = "pgp"))]
        {
            let _ = args;
            Err(OperationError::ProcessingError(
                "Generate PGP Key Pair requires --features pgp".to_string(),
            ))
        }
    }
}
