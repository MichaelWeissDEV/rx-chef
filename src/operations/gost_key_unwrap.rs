/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Key Unwrap operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};
use cipher::{BlockCipher, BlockDecrypt, BlockEncrypt, BlockSizeUser, KeyInit};
use generic_array::GenericArray;
use kuznyechik::Kuznyechik;
use magma::Magma;
use subtle::ConstantTimeEq;

use super::gost_mac::{diversify_kek_64, gost_cmac};

/// GOST Key Unwrap operation
pub struct GOSTKeyUnwrapOp;

impl Operation for GOSTKeyUnwrapOp {
    fn name(&self) -> &'static str {
        "GOST Key Unwrap"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "A decryptor for keys wrapped using one of the GOST block ciphers, per RFC 4357 (\"NO\" and \"CP\"/CryptoPro key wrapping). User Key Material (UKM) must match the value used to wrap the key, and must be exactly one block long. \"CP\" (CryptoPro) key wrapping is only supported for 64-bit block ciphers (GOST 28147 (1989), which this operation implements as an alias for GOST R 34.12 (Magma, 2015), and GOST R 34.12 (Magma, 2015) itself); it is not supported for Kuznyechik. \"SC\" (SignalCom) key wrapping is not implemented."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The decryption key.",
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
                description: "UKM",
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
                description: "Type of input data",
                default_value: "Hex",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Output type",
                description: "Type of output data",
                default_value: "Raw",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Algorithm",
                description: "GOST version",
                default_value: "GOST R 34.12 (Magma, 2015)",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "sBox",
                description: "S-Box to use (1989 only)",
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
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let kek = parse_arg_bytes(args.first())?;
        let ukm = parse_arg_bytes(args.get(1))?;
        let input_type = args.get(2).and_then(ArgValue::as_str).unwrap_or("Hex");
        let output_type = args.get(3).and_then(ArgValue::as_str).unwrap_or("Raw");
        let algorithm = args
            .get(4)
            .and_then(ArgValue::as_str)
            .unwrap_or("GOST R 34.12 (Magma, 2015)");
        let key_wrapping = args.get(6).and_then(ArgValue::as_str).unwrap_or("NO");
        if kek.len() != 32 {
            return Err(OperationError::InvalidArgument {
                name: "Key".into(),
                reason: "KEK must be 32 bytes".into(),
            });
        }
        let wrapped = if input_type == "Hex" {
            hex::decode(&input).map_err(|error| OperationError::InvalidInput(error.to_string()))?
        } else {
            input
        };
        let result = match algorithm {
            "GOST 28147 (1989)" | "GOST R 34.12 (Magma, 2015)" => {
                unwrap::<Magma>(&kek, &ukm, &wrapped, key_wrapping)?
            }
            "GOST R 34.12 (Kuznyechik, 2015)" => {
                unwrap::<Kuznyechik>(&kek, &ukm, &wrapped, key_wrapping)?
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Algorithm".into(),
                    reason: format!("Unsupported algorithm: {algorithm}"),
                })
            }
        };
        if output_type == "Hex" {
            Ok(hex::encode(result).into_bytes())
        } else {
            Ok(result)
        }
    }
}

fn parse_arg_bytes(arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
    match arg {
        Some(ArgValue::Bytes(value)) => Ok(value.clone()),
        Some(ArgValue::Str(value)) if value.starts_with("0x") => {
            hex::decode(&value[2..]).map_err(|error| OperationError::InvalidArgument {
                name: "Key".into(),
                reason: error.to_string(),
            })
        }
        Some(ArgValue::Str(value)) => Ok(value.as_bytes().to_vec()),
        _ => Ok(Vec::new()),
    }
}

/// Inverse of `GostKeyWrap::wrap`: see that function's docs and the module
/// docs in `gost_mac` for the reference this is ported from.
fn unwrap<C>(
    kek: &[u8],
    ukm: &[u8],
    wrapped: &[u8],
    key_wrapping: &str,
) -> Result<Vec<u8>, OperationError>
where
    C: BlockCipher + BlockSizeUser + KeyInit + BlockEncrypt + BlockDecrypt,
{
    let block_size = C::block_size();
    let mac_size = block_size / 2;
    if wrapped.len() <= mac_size || (wrapped.len() - mac_size) % block_size != 0 {
        return Err(OperationError::InvalidInput(format!(
            "Wrapped key must contain whole {block_size}-byte blocks followed by a {mac_size}-byte MAC"
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

    let split = wrapped.len() - mac_size;
    let (encrypted, expected_mac) = wrapped.split_at(split);
    let cipher = C::new(GenericArray::from_slice(&dek));
    let mut cek = Vec::with_capacity(encrypted.len());
    for chunk in encrypted.chunks_exact(block_size) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        cek.extend_from_slice(&block);
    }

    let mac = gost_cmac::<C>(&dek, Some(ukm), &cek);
    if !bool::from(mac[..mac_size].ct_eq(expected_mac)) {
        return Err(OperationError::ProcessingError(
            "GOST wrapped-key MAC verification failed".into(),
        ));
    }
    Ok(cek)
}
