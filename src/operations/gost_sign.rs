/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Sign operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockCipher, BlockEncrypt, BlockSizeUser, KeyInit};
use kuznyechik::Kuznyechik;
use magma::Magma;

use super::gost_mac::gost_cmac;
use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// GOST Sign operation
pub struct GostSign;

impl Operation for GostSign {
    fn name(&self) -> &'static str {
        "GOST Sign"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Sign a plaintext message (calculate MAC) using one of the GOST block ciphers, using the GOST R 34.13-2015 CMAC-style MAC construction. \"GOST 28147 (1989)\" is implemented as an alias for GOST R 34.12 (Magma, 2015) (matching this crate's GOST Encrypt/Decrypt behaviour); the original GOST 28147-89 round-reduced imitovstavka construction with selectable S-boxes is not implemented."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The encryption key.",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "IV",
                description: "The initialization vector.",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Input type",
                description: "Input encoding (Raw, Hex)",
                default_value: "Raw",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["Raw", "Hex"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output type",
                description: "Output encoding (Hex, Raw)",
                default_value: "Hex",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["Hex", "Raw"],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Algorithm",
                description: "The GOST algorithm to use.",
                default_value: "GOST 28147 (1989)",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "sBox",
                description: "The sBox to use (only for GOST 28147 (1989)).",
                default_value: "E-TEST",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "MAC length",
                description: "The length of the MAC in bits.",
                default_value: "32",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key = Self::parse_arg_bytes(args.first())?;
        let iv = Self::parse_arg_bytes(args.get(1))?;
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let algorithm = args
            .get(4)
            .and_then(|a| a.as_str())
            .unwrap_or("GOST 28147 (1989)");
        let mac_length_bits = args.get(6).and_then(|a| a.as_str()).unwrap_or("32");
        let mac_length = mac_length_bits.parse::<usize>().unwrap_or(32) / 8;

        let input_bytes = if input_type == "Hex" {
            hex::decode(&input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        let result = match algorithm {
            "GOST 28147 (1989)" | "GOST R 34.12 (Magma, 2015)" => {
                if key.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "Key must be 32 bytes".to_string(),
                    });
                }
                self.calculate_mac::<Magma>(&key, &iv, &input_bytes, mac_length)?
            }
            "GOST R 34.12 (Kuznyechik, 2015)" => {
                if key.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "Key must be 32 bytes".to_string(),
                    });
                }
                self.calculate_mac::<Kuznyechik>(&key, &iv, &input_bytes, mac_length)?
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Algorithm".to_string(),
                    reason: format!("Unsupported algorithm: {}", algorithm),
                })
            }
        };

        let output_bytes = if output_type == "Hex" {
            hex::encode(result).into_bytes()
        } else {
            result
        };

        Ok(output_bytes)
    }
}

impl GostSign {
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

    /// Computes the GOST R 34.13-2015 MAC of `input` under `key`, with `iv`
    /// as the initial register (defaults to an all-zero block, matching the
    /// reference implementation). See `gost_mac` for the algorithm this is
    /// ported from and its documented divergence from the original GOST
    /// 28147-89 round-reduced "imitovstavka" construction.
    fn calculate_mac<C>(
        &self,
        key: &[u8],
        iv: &[u8],
        input: &[u8],
        mac_length: usize,
    ) -> Result<Vec<u8>, OperationError>
    where
        C: BlockCipher + BlockSizeUser + KeyInit + BlockEncrypt,
    {
        let block_size = C::block_size();
        if !iv.is_empty() && iv.len() != block_size {
            return Err(OperationError::InvalidArgument {
                name: "IV".to_string(),
                reason: format!("IV must be {} bytes", block_size),
            });
        }
        let iv_opt = if iv.is_empty() { None } else { Some(iv) };
        let mut mac = gost_cmac::<C>(key, iv_opt, input);
        mac.truncate(mac_length.min(mac.len()));
        Ok(mac)
    }
}
