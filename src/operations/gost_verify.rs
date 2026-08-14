/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the GOST Verify operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};
use crate::operations::gost_sign::GostSign;
use subtle::ConstantTimeEq;

/// GOST Verify operation
pub struct GOSTVerifyOp;

impl Operation for GOSTVerifyOp {
    fn name(&self) -> &'static str {
        "GOST Verify"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "Verify the signature of a plaintext message using one of the GOST block ciphers. Enter the signature in the MAC field."
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
                name: "MAC",
                description: "The signature/MAC to verify.",
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
        let mac_value = args.get(2).and_then(ArgValue::as_str).unwrap_or("");
        let mac = decode_mac(mac_value)?;
        if mac.is_empty() {
            return Err(OperationError::InvalidArgument {
                name: "MAC".into(),
                reason: "MAC must not be empty".into(),
            });
        }
        let algorithm = args
            .get(4)
            .and_then(ArgValue::as_str)
            .unwrap_or("GOST R 34.12 (Magma, 2015)");
        let calculated = GostSign.run(
            input,
            &[
                args.first()
                    .cloned()
                    .unwrap_or_else(|| ArgValue::Str(String::new())),
                args.get(1)
                    .cloned()
                    .unwrap_or_else(|| ArgValue::Str(String::new())),
                args.get(3)
                    .cloned()
                    .unwrap_or_else(|| ArgValue::Str("Raw".into())),
                ArgValue::Str("Raw".into()),
                ArgValue::Str(algorithm.into()),
                args.get(5)
                    .cloned()
                    .unwrap_or_else(|| ArgValue::Str("E-TEST".into())),
                ArgValue::Str((mac.len() * 8).to_string()),
            ],
        )?;
        let verified = bool::from(calculated.as_slice().ct_eq(mac.as_slice()));
        Ok(verified.to_string().into_bytes())
    }
}

fn decode_mac(value: &str) -> Result<Vec<u8>, OperationError> {
    let hex_value = value.strip_prefix("0x").unwrap_or(value);
    if hex_value.len() % 2 == 0 && hex_value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        hex::decode(hex_value).map_err(|error| OperationError::InvalidArgument {
            name: "MAC".into(),
            reason: error.to_string(),
        })
    } else {
        Ok(value.as_bytes().to_vec())
    }
}
