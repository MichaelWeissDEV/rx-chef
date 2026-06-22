/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse ASN.1 hex string operation.
 * -----------------------------------------------------------------------------
 */

use der::Tag;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse ASN.1 hex string operation
pub struct ParseASN1HexString;

impl Operation for ParseASN1HexString {
    fn name(&self) -> &'static str {
        "Parse ASN.1 hex string"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Abstract Syntax Notation One (ASN.1) is a standard and notation that describes rules and structures for representing, encoding, transmitting, and decoding data in telecommunications and computer networking.<br><br>This operation parses arbitrary ASN.1 data (encoded as an hex string: use the 'To Hex' operation if necessary) and presents the resulting tree."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Starting index",
                description: "Starting index in the byte array",
                default_value: "0",
            },
            ArgSchema {
                name: "Truncate octet strings longer than",
                description: "Truncate octet strings longer than this value",
                default_value: "32",
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
        let hex_input = String::from_utf8_lossy(&input)
            .replace(|c: char| c.is_whitespace(), "")
            .to_lowercase();
        if hex_input.is_empty() {
            return Ok(Vec::new());
        }

        let der_bytes = hex::decode(&hex_input)
            .map_err(|_| OperationError::InvalidInput("Invalid hex string".to_string()))?;

        let start_index = if let Some(arg) = args.first() {
            arg.as_usize().unwrap_or(0)
        } else {
            0
        };

        if start_index >= der_bytes.len() {
            return Err(OperationError::InvalidInput(
                "Starting index out of bounds".to_string(),
            ));
        }

        let truncate_len = if let Some(arg) = args.get(1) {
            arg.as_usize().unwrap_or(32)
        } else {
            32
        };

        let mut output = String::new();
        dump_asn1(&der_bytes[start_index..], &mut output, 0, truncate_len)?;

        Ok(output.into_bytes())
    }
}

fn dump_asn1(
    bytes: &[u8],
    output: &mut String,
    depth: usize,
    truncate_len: usize,
) -> Result<(), OperationError> {
    use der::{Decode, Header, Reader};
    let mut reader = der::SliceReader::new(bytes)
        .map_err(|e| OperationError::ProcessingError(format!("ASN.1 parse error: {}", e)))?;

    while !reader.is_finished() {
        let header = Header::decode(&mut reader)
            .map_err(|e| OperationError::ProcessingError(format!("ASN.1 parse error: {}", e)))?;
        let tag = header.tag();
        let length = header.length();
        let value_bytes = reader
            .read_slice(length)
            .map_err(|e| OperationError::ProcessingError(format!("ASN.1 parse error: {}", e)))?;

        let indent = "  ".repeat(depth);
        output.push_str(&format!("{}{}: ", indent, tag));

        if tag.is_constructed() {
            output.push('\n');
            dump_asn1(value_bytes, output, depth + 1, truncate_len)?;
        } else {
            match tag {
                Tag::Integer => {
                    output.push_str(&format!("{}\n", hex::encode(value_bytes)));
                }
                Tag::BitString => {
                    output.push_str(&format!("BIT STRING ({} bytes)\n", value_bytes.len()));
                }
                Tag::OctetString => {
                    if value_bytes.len() > truncate_len {
                        output.push_str(&format!(
                            "OCTET STRING ({} bytes, truncated): {}\n",
                            value_bytes.len(),
                            hex::encode(&value_bytes[..truncate_len])
                        ));
                    } else {
                        output.push_str(&format!(
                            "OCTET STRING ({} bytes): {}\n",
                            value_bytes.len(),
                            hex::encode(value_bytes)
                        ));
                    }
                }
                Tag::Null => {
                    output.push_str("NULL\n");
                }
                Tag::ObjectIdentifier => {
                    output.push_str(&format!("OID ({})\n", hex::encode(value_bytes)));
                }
                Tag::PrintableString | Tag::Utf8String | Tag::Ia5String => {
                    output.push_str(&format!("\"{}\"\n", String::from_utf8_lossy(value_bytes)));
                }
                _ => {
                    output.push_str(&format!(
                        "(raw {} bytes): {}\n",
                        value_bytes.len(),
                        hex::encode(value_bytes)
                    ));
                }
            }
        }
    }
    Ok(())
}
