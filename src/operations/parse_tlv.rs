/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse TLV operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::json;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse TLV operation
pub struct ParseTLV;

struct TLVParser<'a> {
    input: &'a [u8],
    location: usize,
    bytes_in_length: usize,
    basic_encoding_rules: bool,
}

impl<'a> TLVParser<'a> {
    fn new(input: &'a [u8], bytes_in_length: usize, basic_encoding_rules: bool) -> Self {
        Self {
            input,
            location: 0,
            bytes_in_length,
            basic_encoding_rules,
        }
    }

    fn get_length(&mut self) -> usize {
        if self.basic_encoding_rules {
            if self.location >= self.input.len() {
                return 0;
            }
            let bit = self.input[self.location];
            if bit & 0x80 != 0 {
                self.bytes_in_length = (bit & !0x80) as usize;
                self.location += 1; // Assuming CyberChef should have had this or handles it in the loop
            } else {
                self.location += 1;
                return (bit & !0x80) as usize;
            }
        }

        let mut length: usize = 0;
        for i in 0..self.bytes_in_length {
            if self.location >= self.input.len() {
                break;
            }
            length += (self.input[self.location] as usize) * (1 << (8 * i));
            self.location += 1;
        }
        length
    }

    fn get_value(&mut self, length: usize) -> Vec<u8> {
        let mut value = Vec::new();
        for _ in 0..length {
            if self.location >= self.input.len() {
                break;
            }
            value.push(self.input[self.location]);
            self.location += 1;
        }
        value
    }

    fn at_end(&self) -> bool {
        self.location >= self.input.len()
    }
}

impl Operation for ParseTLV {
    fn name(&self) -> &'static str {
        "Parse TLV"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts a Type-Length-Value (TLV) encoded string into a JSON object. Can optionally include a Key / Type entry."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Type/Key size",
                description: "Size of the type/key field in bytes",
                default_value: "1",
            },
            ArgSchema {
                name: "Length size",
                description: "Size of the length field in bytes",
                default_value: "1",
            },
            ArgSchema {
                name: "Use BER",
                description: "Use Basic Encoding Rules for length field",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let bytes_in_key = if args.len() > 0 {
            args[0].as_usize().unwrap_or(1)
        } else {
            1
        };
        let bytes_in_length = if args.len() > 1 {
            args[1].as_usize().unwrap_or(1)
        } else {
            1
        };
        let basic_encoding_rules = if args.len() > 2 {
            args[2].as_bool().unwrap_or(false)
        } else {
            false
        };

        if bytes_in_key == 0 && bytes_in_length == 0 {
            return Err(OperationError::InvalidInput(
                "Type or Length size must be greater than 0".to_string(),
            ));
        }

        let mut tlv = TLVParser::new(&input, bytes_in_length, basic_encoding_rules);
        let mut data = Vec::new();

        while !tlv.at_end() {
            let key = if bytes_in_key > 0 {
                Some(tlv.get_value(bytes_in_key))
            } else {
                None
            };
            let length = tlv.get_length();
            let value = tlv.get_value(length);

            let mut entry = json!({
                "length": length,
                "value": value,
            });

            if let Some(k) = key {
                entry
                    .as_object_mut()
                    .unwrap()
                    .insert("key".to_string(), json!(k));
            }

            data.push(entry);
        }

        let output = serde_json::to_string_pretty(&data)
            .map_err(|e| OperationError::ProcessingError(e.to_string()))?;
        Ok(output.into_bytes())
    }
}
