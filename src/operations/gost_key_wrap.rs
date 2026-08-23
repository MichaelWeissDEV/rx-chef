/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Key Wrap operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockCipher, BlockEncrypt, BlockSizeUser, KeyInit};
use generic_array::GenericArray;
use kuznyechik::Kuznyechik;
use magma::Magma;

use super::gost_mac::{diversify_kek_64, gost_cmac};
use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// GOST Key Wrap operation
pub struct GostKeyWrap;

impl Operation for GostKeyWrap {
    fn name(&self) -> &'static str {
        "GOST Key Wrap"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A key wrapping algorithm for protecting keys in untrusted storage using one of the GOST block ciphers, per RFC 4357 (\"NO\" and \"CP\"/CryptoPro key wrapping). The content-encryption key must be a non-empty multiple of the cipher's block size, and User Key Material (UKM) must be exactly one block long. \"CP\" (CryptoPro) key wrapping is only supported for 64-bit block ciphers (GOST 28147 (1989), which this operation implements as an alias for GOST R 34.12 (Magma, 2015), and GOST R 34.12 (Magma, 2015) itself); it is not supported for Kuznyechik. \"SC\" (SignalCom) key wrapping is not implemented."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The Key Encryption Key (KEK).",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "User Key Material",
                description: "User Key Material (UKM).",
                default_value: "",
                kind: crate::operation::ArgKind::String,
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
                name: "Key wrapping",
                description: "The key wrapping mode. \"CP\" (CryptoPro diversification) is only supported for 64-bit block ciphers. \"SC\" is not implemented.",
                default_value: "NO",
                kind: crate::operation::ArgKind::Enum,
                required: false,
                choices: &["NO", "CP", "SC"],
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
        let kek = Self::parse_arg_bytes(args.first())?;
        let ukm = Self::parse_arg_bytes(args.get(1))?;
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Raw");
        let output_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let algorithm = args
            .get(4)
            .and_then(|a| a.as_str())
            .unwrap_or("GOST 28147 (1989)");
        let key_wrapping = args.get(6).and_then(|a| a.as_str()).unwrap_or("NO");

        let input_bytes = if input_type == "Hex" {
            hex::decode(&input).map_err(|e| OperationError::InvalidInput(e.to_string()))?
        } else {
            input
        };

        let result = match algorithm {
            "GOST 28147 (1989)" | "GOST R 34.12 (Magma, 2015)" => {
                if kek.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "KEK must be 32 bytes".to_string(),
                    });
                }
                self.wrap::<Magma>(&kek, &ukm, &input_bytes, key_wrapping)?
            }
            "GOST R 34.12 (Kuznyechik, 2015)" => {
                if kek.len() != 32 {
                    return Err(OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: "KEK must be 32 bytes".to_string(),
                    });
                }
                self.wrap::<Kuznyechik>(&kek, &ukm, &input_bytes, key_wrapping)?
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

impl GostKeyWrap {
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

    /// Wraps `cek` under `kek`, following RFC 4357 section 6.1 (`key_wrapping
    /// == "NO"`) or section 6.3 (`key_wrapping == "CP"`, CryptoPro key
    /// diversification). The wrapped output is `CEK_ENC || CEK_MAC`, matching
    /// the reference implementation (UKM is a required side parameter, not
    /// embedded in the returned bytes — see the module-level docs in
    /// `gost_mac`).
    fn wrap<C>(
        &self,
        kek: &[u8],
        ukm: &[u8],
        cek: &[u8],
        key_wrapping: &str,
    ) -> Result<Vec<u8>, OperationError>
    where
        C: BlockCipher + BlockSizeUser + KeyInit + BlockEncrypt,
    {
        let block_size = C::block_size();
        if cek.is_empty() || cek.len() % block_size != 0 {
            return Err(OperationError::InvalidInput(format!(
                "Content-encryption key must be a non-empty multiple of {block_size} bytes"
            )));
        }
        if ukm.len() != block_size {
            return Err(OperationError::InvalidArgument {
                name: "User Key Material".to_string(),
                reason: format!("UKM must be {block_size} bytes for this algorithm"),
            });
        }

        let dek: Vec<u8> = match key_wrapping {
            "NO" | "" => kek.to_vec(),
            "CP" => diversify_kek_64::<C>(kek, ukm)?,
            "SC" => {
                return Err(OperationError::InvalidArgument {
                    name: "Key wrapping".to_string(),
                    reason: "SC (SignalCom) key wrapping is not implemented".to_string(),
                })
            }
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Key wrapping".to_string(),
                    reason: format!("Unsupported key wrapping mode: {other}"),
                })
            }
        };

        let cipher = C::new(GenericArray::from_slice(&dek));
        let mut encrypted_cek = Vec::with_capacity(cek.len());
        for chunk in cek.chunks(block_size) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block(&mut block);
            encrypted_cek.extend_from_slice(block.as_slice());
        }

        let mac = gost_cmac::<C>(&dek, Some(ukm), cek);

        let mut result = encrypted_cek;
        result.extend_from_slice(&mac[..block_size / 2]);
        Ok(result)
    }
}
