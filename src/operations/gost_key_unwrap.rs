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
use cipher::{BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit};
use generic_array::GenericArray;
use kuznyechik::Kuznyechik;
use magma::Magma;
use subtle::ConstantTimeEq;

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
        "A decryptor for keys wrapped using one of the GOST block ciphers."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The decryption key.",
                default_value: "",
            },
            ArgSchema {
                name: "User Key Material",
                description: "UKM",
                default_value: "",
            },
            ArgSchema {
                name: "Input type",
                description: "Type of input data",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output type",
                description: "Type of output data",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Algorithm",
                description: "GOST version",
                default_value: "GOST R 34.12 (Magma, 2015)",
            },
            ArgSchema {
                name: "sBox",
                description: "S-Box to use (1989 only)",
                default_value: "E-TEST",
            },
            ArgSchema {
                name: "Key wrapping",
                description: "Key wrapping mode",
                default_value: "NO",
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
        let input_type = args.get(2).and_then(ArgValue::as_str).unwrap_or("Hex");
        let output_type = args.get(3).and_then(ArgValue::as_str).unwrap_or("Raw");
        let algorithm = args
            .get(4)
            .and_then(ArgValue::as_str)
            .unwrap_or("GOST R 34.12 (Magma, 2015)");
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
            "GOST 28147 (1989)" | "GOST R 34.12 (Magma, 2015)" => unwrap::<Magma>(&kek, &wrapped)?,
            "GOST R 34.12 (Kuznyechik, 2015)" => unwrap::<Kuznyechik>(&kek, &wrapped)?,
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

fn unwrap<C>(kek: &[u8], wrapped: &[u8]) -> Result<Vec<u8>, OperationError>
where
    C: BlockCipher + cipher::BlockSizeUser + KeyInit + BlockEncrypt + BlockDecrypt,
{
    let block_size = C::block_size();
    let mac_size = block_size / 2;
    if wrapped.len() <= mac_size || (wrapped.len() - mac_size) % block_size != 0 {
        return Err(OperationError::InvalidInput(format!(
            "Wrapped key must contain whole {block_size}-byte blocks followed by a {mac_size}-byte MAC"
        )));
    }
    let split = wrapped.len() - mac_size;
    let (encrypted, expected_mac) = wrapped.split_at(split);
    let cipher = C::new(GenericArray::from_slice(kek));
    let mut cek = Vec::with_capacity(encrypted.len());
    for chunk in encrypted.chunks_exact(block_size) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        cek.extend_from_slice(&block);
    }
    let mut register = vec![0_u8; block_size];
    for chunk in cek.chunks_exact(block_size) {
        let mut block = GenericArray::clone_from_slice(chunk);
        for (byte, previous) in block.iter_mut().zip(&register) {
            *byte ^= previous;
        }
        cipher.encrypt_block(&mut block);
        register.copy_from_slice(&block);
    }
    if !bool::from(register[..mac_size].ct_eq(expected_mac)) {
        return Err(OperationError::ProcessingError(
            "GOST wrapped-key MAC verification failed".into(),
        ));
    }
    Ok(cek)
}
