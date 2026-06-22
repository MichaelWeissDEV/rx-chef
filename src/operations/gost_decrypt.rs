/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use cipher::{BlockDecrypt, BlockDecryptMut, BlockSizeUser, KeyInit, KeyIvInit};
use kuznyechik::Kuznyechik;
use magma::Magma;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// GOST Decrypt operation
pub struct GOSTDecryptOp;

impl Operation for GOSTDecryptOp {
    fn name(&self) -> &'static str {
        "GOST Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "The GOST block cipher (Magma), defined in the standard GOST 28147-89 (RFC 5830), is a Soviet and Russian government standard symmetric key block cipher with a block size of 64 bits."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "The decryption key.",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "The initialization vector.",
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
                name: "Block mode",
                description: "Mode of operation",
                default_value: "ECB",
            },
            ArgSchema {
                name: "Key meshing mode",
                description: "Key meshing",
                default_value: "NO",
            },
            ArgSchema {
                name: "Padding",
                description: "Padding scheme",
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
        let key =
            Utils::convert_to_byte_array(args.first().ok_or(OperationError::InvalidArgument {
                name: "Key".to_string(),
                reason: "Missing".to_string(),
            })?)?;
        let iv = Utils::convert_to_byte_array(args.get(1).unwrap_or(&ArgValue::Bytes(Vec::new())))?;
        let input_type = args.get(2).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Raw");
        let algorithm = args
            .get(4)
            .and_then(|a| a.as_str())
            .unwrap_or("GOST R 34.12 (Magma, 2015)");
        let block_mode = args.get(6).and_then(|a| a.as_str()).unwrap_or("ECB");

        let data = if input_type == "Hex" {
            let input_str = String::from_utf8_lossy(&input).trim().to_string();
            hex::decode(input_str)
                .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let result = if algorithm.contains("Kuznyechik") {
            decrypt_gost::<Kuznyechik>(&key, &iv, &data, block_mode)?
        } else {
            // Both GOST 28147 (1989) and Magma (2015) use 64-bit blocks.
            // Magma crate implements the 2015 standard.
            decrypt_gost::<Magma>(&key, &iv, &data, block_mode)?
        };

        if output_type == "Hex" {
            Ok(hex::encode(result).into_bytes())
        } else {
            Ok(result)
        }
    }
}

fn decrypt_gost<C>(
    key: &[u8],
    iv: &[u8],
    data: &[u8],
    mode: &str,
) -> Result<Vec<u8>, OperationError>
where
    C: BlockDecrypt + KeyInit + BlockSizeUser + cipher::BlockCipher + BlockDecryptMut,
{
    let mut buffer = data.to_vec();
    match mode {
        "ECB" => {
            let cipher = C::new_from_slice(key)
                .map_err(|_| OperationError::ProcessingError("Invalid key length".to_string()))?;
            for chunk in buffer.chunks_mut(C::block_size()) {
                if chunk.len() == C::block_size() {
                    cipher.decrypt_block(generic_array::GenericArray::from_mut_slice(chunk));
                }
            }
        }
        "CBC" => {
            // Requires cbc crate
            type CbcDecryptor<C> = cbc::Decryptor<C>;
            let mut cipher = CbcDecryptor::<C>::new_from_slices(key, iv).map_err(|_| {
                OperationError::ProcessingError("Invalid key/IV length".to_string())
            })?;
            for chunk in buffer.chunks_mut(C::block_size()) {
                if chunk.len() == C::block_size() {
                    cipher.decrypt_block_mut(generic_array::GenericArray::from_mut_slice(chunk));
                }
            }
        }
        _ => {
            return Err(OperationError::ProcessingError(format!(
                "Block mode {} not yet implemented for GOST",
                mode
            )))
        }
    }
    Ok(buffer)
}
