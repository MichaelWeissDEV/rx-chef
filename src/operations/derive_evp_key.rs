/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Derive EVP key operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Derive EVP key operation
///
/// This operation performs a password-based key derivation function (PBKDF) used extensively in OpenSSL.
/// In many applications of cryptography, user security is ultimately dependent on a password, and because
/// a password usually can't be used directly as a cryptographic key, some processing is required.
///
/// A salt provides a large set of keys for any given password, and an iteration count increases the cost
/// of producing keys from a password, thereby also increasing the difficulty of attack.
///
/// If you leave the salt argument empty, a random salt will be generated.
pub struct DeriveEvpKey;

impl Operation for DeriveEvpKey {
    fn name(&self) -> &'static str {
        "Derive EVP key"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "This operation performs a password-based key derivation function (PBKDF) used extensively in OpenSSL. In many applications of cryptography, user security is ultimately dependent on a password, and because a password usually can't be used directly as a cryptographic key, some processing is required.<br><br>A salt provides a large set of keys for any given password, and an iteration count increases the cost of producing keys from a password, thereby also increasing the difficulty of attack.<br><br>If you leave the salt argument empty, a random salt will be generated."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Passphrase",
                description: "The passphrase to derive the key from.",
                default_value: "",
            },
            ArgSchema {
                name: "Key size",
                description: "The length of the key to generate in bits.",
                default_value: "128",
            },
            ArgSchema {
                name: "Iterations",
                description: "The number of times the hash function is applied.",
                default_value: "1",
            },
            ArgSchema {
                name: "Hashing function",
                description: "The hash function to use.",
                default_value: "MD5",
            },
            ArgSchema {
                name: "Salt",
                description: "The salt to use. If empty, a random salt will be generated.",
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
        let passphrase = Self::parse_arg_bytes(args.first())?;

        let key_size_bits = match args.get(1) {
            Some(ArgValue::Num(n)) => *n as usize,
            Some(ArgValue::Str(s)) => s.parse().unwrap_or(128),
            _ => 128,
        };

        let iterations = match args.get(2) {
            Some(ArgValue::Num(n)) => *n as usize,
            Some(ArgValue::Str(s)) => s.parse().unwrap_or(1),
            _ => 1,
        }
        .max(1);

        let hasher = args.get(3).and_then(|a| a.as_str()).unwrap_or("MD5");

        let mut salt = Self::parse_arg_bytes(args.get(4))?;
        if salt.is_empty() {
            salt = vec![0u8; 8];
            getrandom::getrandom(&mut salt).map_err(|e| {
                OperationError::ProcessingError(format!("Failed to generate random salt: {}", e))
            })?;
        }

        let key_len = key_size_bits / 8;

        let derived_key = match hasher {
            "SHA1" => Self::derive_evp::<sha1::Sha1>(&passphrase, &salt, key_len, iterations),
            "SHA256" => Self::derive_evp::<sha2::Sha256>(&passphrase, &salt, key_len, iterations),
            "SHA384" => Self::derive_evp::<sha2::Sha384>(&passphrase, &salt, key_len, iterations),
            "SHA512" => Self::derive_evp::<sha2::Sha512>(&passphrase, &salt, key_len, iterations),
            "MD5" => Self::derive_evp::<md5::Md5>(&passphrase, &salt, key_len, iterations),
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Hashing function".to_string(),
                    reason: format!("Unsupported hashing function: {}", hasher),
                })
            }
        };

        Ok(hex::encode(derived_key).into_bytes())
    }
}

impl DeriveEvpKey {
    /// Parse an ArgValue as bytes (supports Hex and UTF8)
    fn parse_arg_bytes(arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
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

    fn derive_evp<D: sha2::Digest>(
        passphrase: &[u8],
        salt: &[u8],
        key_len: usize,
        iterations: usize,
    ) -> Vec<u8> {
        let mut derived_key = Vec::new();
        let mut block = Vec::new();

        while derived_key.len() < key_len {
            let mut hasher = D::new();
            hasher.update(&block);
            hasher.update(passphrase);
            if !salt.is_empty() {
                hasher.update(salt);
            }
            block = hasher.finalize().to_vec();

            for _ in 1..iterations {
                let mut hasher = D::new();
                hasher.update(&block);
                block = hasher.finalize().to_vec();
            }

            derived_key.extend_from_slice(&block);
        }

        derived_key.truncate(key_len);
        derived_key
    }
}
