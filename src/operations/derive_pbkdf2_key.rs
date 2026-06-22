/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Derive PBKDF2 key operation.
 * -----------------------------------------------------------------------------
 */

use hmac::Hmac;
use md5::Md5;
use pbkdf2::pbkdf2;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Derive PBKDF2 key operation
pub struct DerivePBKDF2Key;

impl Operation for DerivePBKDF2Key {
    fn name(&self) -> &'static str {
        "Derive PBKDF2 key"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "PBKDF2 is a password-based key derivation function. It is part of RSA Laboratories' Public-Key Cryptography Standards (PKCS) series, specifically PKCS #5 v2.0, also published as Internet Engineering Task Force's RFC 2898.<br><br>In many applications of cryptography, user security is ultimately dependent on a password, and because a password usually can't be used directly as a cryptographic key, some processing is required.<br><br>A salt provides a large set of keys for any given password, and an iteration count increases the cost of producing keys from a password, thereby also increasing the difficulty of attack.<br><br>If you leave the salt argument empty, a random salt will be generated."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Passphrase",
                description: "The passphrase to derive the key from",
                default_value: "",
            },
            ArgSchema {
                name: "Key size",
                description: "The size of the derived key in bits",
                default_value: "128",
            },
            ArgSchema {
                name: "Iterations",
                description: "The number of iterations to perform",
                default_value: "1",
            },
            ArgSchema {
                name: "Hashing function",
                description: "The hashing function to use",
                default_value: "SHA256",
            },
            ArgSchema {
                name: "Salt",
                description: "The salt to use (if empty, a random one will be generated)",
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

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let passphrase = self.parse_arg_bytes(args.first())?;
        let key_size_bits = args.get(1).and_then(|v| v.as_usize()).unwrap_or(128);
        let iterations = args.get(2).and_then(|v| v.as_usize()).unwrap_or(1);
        let hash_func = args.get(3).and_then(|v| v.as_str()).unwrap_or("SHA256");
        let mut salt = self.parse_arg_bytes(args.get(4))?;

        if salt.is_empty() {
            salt = vec![0u8; key_size_bits / 8];
            getrandom::getrandom(&mut salt).map_err(|e| {
                OperationError::ProcessingError(format!("Failed to generate random salt: {}", e))
            })?;
        }

        let key_size_bytes = key_size_bits / 8;
        let mut derived_key = vec![0u8; key_size_bytes];

        match hash_func {
            "SHA1" => pbkdf2::<Hmac<Sha1>>(&passphrase, &salt, iterations as u32, &mut derived_key)
                .map_err(|e| OperationError::ProcessingError(e.to_string()))?,
            "SHA256" => {
                pbkdf2::<Hmac<Sha256>>(&passphrase, &salt, iterations as u32, &mut derived_key)
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?
            }
            "SHA384" => {
                pbkdf2::<Hmac<Sha384>>(&passphrase, &salt, iterations as u32, &mut derived_key)
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?
            }
            "SHA512" => {
                pbkdf2::<Hmac<Sha512>>(&passphrase, &salt, iterations as u32, &mut derived_key)
                    .map_err(|e| OperationError::ProcessingError(e.to_string()))?
            }
            "MD5" => pbkdf2::<Hmac<Md5>>(&passphrase, &salt, iterations as u32, &mut derived_key)
                .map_err(|e| OperationError::ProcessingError(e.to_string()))?,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Hashing function".to_string(),
                    reason: format!("Unsupported hashing function: {}", hash_func),
                })
            }
        }

        Ok(hex::encode(derived_key).into_bytes())
    }
}

impl DerivePBKDF2Key {
    fn parse_arg_bytes(&self, arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else if s.starts_with("0x") {
                    hex::decode(&s[2..]).map_err(|e| OperationError::InvalidArgument {
                        name: "Argument".to_string(),
                        reason: e.to_string(),
                    })
                } else {
                    Ok(s.as_bytes().to_vec())
                }
            }
            Some(ArgValue::Bytes(b)) => Ok(b.clone()),
            _ => Ok(vec![]),
        }
    }
}
