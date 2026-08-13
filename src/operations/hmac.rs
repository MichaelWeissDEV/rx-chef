/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the HMAC operation.
 * -----------------------------------------------------------------------------
 */

use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// HMAC operation
pub struct HMAC;

impl Operation for HMAC {
    fn name(&self) -> &'static str {
        "HMAC"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Keyed-Hash Message Authentication Codes (HMAC) are a mechanism for message authentication using cryptographic hash functions."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description:
                    "The secret key as UTF-8 text, or explicitly prefixed with hex:/0x or base64:",
                default_value: "",
            },
            ArgSchema {
                name: "Hashing function",
                description: "Hashing algorithm (MD5, SHA-1, SHA-256, SHA-384, SHA-512)",
                default_value: "SHA-256",
            },
            ArgSchema {
                name: "Output encoding",
                description: "Output encoding (Hex, Base64)",
                default_value: "Hex",
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

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key = args.first().and_then(|a| a.as_str()).unwrap_or("");
        let hash_func = args.get(1).and_then(|a| a.as_str()).unwrap_or("SHA-256");
        let output_encoding = args.get(2).and_then(|a| a.as_str()).unwrap_or("Hex");

        let key_bytes = decode_key(key)?;

        let digest = compute_hmac(&key_bytes, hash_func, &input)?;

        let result = match output_encoding {
            "Hex" => hex::encode(&digest),
            "Base64" => data_encoding::BASE64.encode(&digest),
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported output encoding: {}",
                    output_encoding
                )));
            }
        };

        Ok(result.into_bytes())
    }
}

fn decode_key(key: &str) -> Result<Vec<u8>, OperationError> {
    if key.is_empty() {
        return Ok(vec![]);
    }

    if let Some(encoded) = key.strip_prefix("0x").or_else(|| key.strip_prefix("hex:")) {
        return hex::decode(encoded).map_err(|e| OperationError::InvalidArgument {
            name: "Key".to_string(),
            reason: format!("Invalid hex: {}", e),
        });
    }

    if let Some(encoded) = key.strip_prefix("base64:") {
        return data_encoding::BASE64
            .decode(encoded.as_bytes())
            .map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid Base64: {}", e),
            });
    }

    Ok(key.as_bytes().to_vec())
}

fn compute_hmac(key: &[u8], hash_func: &str, input: &[u8]) -> Result<Vec<u8>, OperationError> {
    let hash_name = hash_func.to_uppercase().replace("-", "").replace("_", "");

    match hash_name.as_str() {
        "MD5" => {
            let mut mac = Hmac::<Md5>::new_from_slice(key).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid key for MD5: {}", e),
            })?;
            mac.update(input);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA1" => {
            let mut mac = Hmac::<Sha1>::new_from_slice(key).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid key for SHA-1: {}", e),
            })?;
            mac.update(input);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA224" => {
            let mut mac = Hmac::<Sha224>::new_from_slice(key).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid key for SHA-224: {}", e),
            })?;
            mac.update(input);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA256" => {
            let mut mac = Hmac::<Sha256>::new_from_slice(key).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid key for SHA-256: {}", e),
            })?;
            mac.update(input);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA384" => {
            let mut mac = Hmac::<Sha384>::new_from_slice(key).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid key for SHA-384: {}", e),
            })?;
            mac.update(input);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        "SHA512" => {
            let mut mac = Hmac::<Sha512>::new_from_slice(key).map_err(|e| OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: format!("Invalid key for SHA-512: {}", e),
            })?;
            mac.update(input);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        _ => Err(OperationError::InvalidArgument {
            name: "Hashing function".to_string(),
            reason: format!(
                "Unsupported hash function: {}. Supported: MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512",
                hash_func
            ),
        }),
    }
}
