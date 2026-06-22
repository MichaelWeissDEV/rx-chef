/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Derive HKDF key operation.
 * -----------------------------------------------------------------------------
 */

use ring::{hkdf, hkdf::KeyType};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Derive HKDF key operation
pub struct DeriveHKDFKey;

impl Operation for DeriveHKDFKey {
    fn name(&self) -> &'static str {
        "Derive HKDF key"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "A simple Hashed Message Authenticaton Code (HMAC)-based key derivation function (HKDF), defined in RFC5869."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Salt",
                description: "The salt to use",
                default_value: "",
            },
            ArgSchema {
                name: "Info",
                description: "The info to use",
                default_value: "",
            },
            ArgSchema {
                name: "Hashing function",
                description: "The hashing function to use (SHA1, SHA256, SHA384, SHA512)",
                default_value: "SHA256",
            },
            ArgSchema {
                name: "Extract mode",
                description: "The extract mode (with salt, no salt, skip)",
                default_value: "with salt",
            },
            ArgSchema {
                name: "L (number of output octets)",
                description: "The number of output octets",
                default_value: "16",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let salt_bytes = self.parse_arg_bytes(args.first())?;
        let info_bytes = self.parse_arg_bytes(args.get(1))?;
        let hash_func = args.get(2).and_then(|v| v.as_str()).unwrap_or("SHA256");
        let extract_mode = args.get(3).and_then(|v| v.as_str()).unwrap_or("with salt");
        let l = args.get(4).and_then(|v| v.as_usize()).unwrap_or(16);

        let algorithm = match hash_func {
            "SHA1" => hkdf::HKDF_SHA1_FOR_LEGACY_USE_ONLY,
            "SHA256" => hkdf::HKDF_SHA256,
            "SHA384" => hkdf::HKDF_SHA384,
            "SHA512" => hkdf::HKDF_SHA512,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Hashing function".to_string(),
                    reason: format!(
                        "Unsupported hashing function: {}. Supported: SHA1, SHA256, SHA384, SHA512",
                        hash_func
                    ),
                })
            }
        };

        let prk = match extract_mode {
            "with salt" => {
                let salt = hkdf::Salt::new(algorithm, &salt_bytes);
                salt.extract(&input)
            }
            "no salt" => {
                let salt = hkdf::Salt::new(algorithm, &vec![0u8; algorithm.len()]);
                salt.extract(&input)
            }
            "skip" => {
                // In "skip" mode, the input is already the PRK
                hkdf::Prk::new_less_safe(algorithm, &input)
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Extract mode".to_string(),
                    reason: format!("Unsupported extract mode: {}", extract_mode),
                })
            }
        };

        let mut okm = vec![0u8; l];
        let info = [info_bytes.as_slice()];
        prk.expand(&info, CustomOkm(l))
            .map_err(|_| OperationError::ProcessingError("HKDF expansion failed".to_string()))?
            .fill(&mut okm)
            .map_err(|_| OperationError::ProcessingError("HKDF fill failed".to_string()))?;

        Ok(hex::encode(okm).into_bytes())
    }
}

impl DeriveHKDFKey {
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

struct CustomOkm(usize);

impl hkdf::KeyType for CustomOkm {
    fn len(&self) -> usize {
        self.0
    }
}
