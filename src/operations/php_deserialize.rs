/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the PHP Deserialize operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// PHP Deserialize operation
pub struct PHPDeserialize;

impl Operation for PHPDeserialize {
    fn name(&self) -> &'static str {
        "PHP Deserialize"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Deserializes PHP serialized data, outputting keyed arrays as JSON.<br><br>This function does not support <code>object</code> tags.<br><br>Example:<br><code>a:2:{s:1:&quot;a&quot;;i:10;i:0;a:1:{s:2:&quot;ab&quot;;b:1;}}</code><br>becomes<br><code>{&quot;a&quot;: 10,0: {&quot;ab&quot;: true}}</code><br><br><u>Output valid JSON:</u> JSON doesn't support integers as keys, whereas PHP serialization does. Enabling this will cast these integers to strings. This will also escape backslashes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Output valid JSON",
            description: "JSON doesn't support integers as keys, whereas PHP serialization does. Enabling this will cast these integers to strings. This will also escape backslashes.",
            default_value: "true",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let valid_json = if let Some(arg) = args.first() {
            arg.as_bool().unwrap_or(true)
        } else {
            true
        };

        let mut parser = PHPParser::new(&input_str, valid_json);
        let result = parser.parse()?;
        Ok(result.into_bytes())
    }
}

struct PHPParser<'a> {
    input: &'a str,
    offset: usize,
    valid_json: bool,
}

impl<'a> PHPParser<'a> {
    fn new(input: &'a str, valid_json: bool) -> Self {
        Self {
            input,
            offset: 0,
            valid_json,
        }
    }

    fn read(&mut self, length: usize) -> Result<&'a str, OperationError> {
        if self.offset + length > self.input.len() {
            return Err(OperationError::ProcessingError(
                "End of input reached".to_string(),
            ));
        }
        let start = self.offset;
        self.offset += length;
        Ok(&self.input[start..self.offset])
    }

    #[allow(dead_code)]
    fn peek(&self) -> Option<char> {
        self.input[self.offset..].chars().next()
    }

    fn read_until(&mut self, until: char) -> Result<String, OperationError> {
        let mut result = String::new();
        for c in self.input[self.offset..].chars() {
            self.offset += c.len_utf8();
            if c == until {
                return Ok(result);
            }
            result.push(c);
        }
        Err(OperationError::ProcessingError(format!(
            "Expected '{}' not found",
            until
        )))
    }

    fn expect(&mut self, expected: &str) -> Result<(), OperationError> {
        let actual = self.read(expected.len())?;
        if actual != expected {
            return Err(OperationError::ProcessingError(format!(
                "Expected '{}', found '{}'",
                expected, actual
            )));
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<String, OperationError> {
        let kind = self.read(1)?.to_lowercase();
        match kind.as_str() {
            "n" => {
                self.expect(";")?;
                Ok("null".to_string())
            }
            "i" | "d" | "b" => {
                self.expect(":")?;
                let data = self.read_until(';')?;
                if kind == "b" {
                    Ok(if data == "0" {
                        "false".to_string()
                    } else {
                        "true".to_string()
                    })
                } else {
                    Ok(data)
                }
            }
            "a" => {
                self.expect(":")?;
                let items_count = self.read_until(':')?.parse::<usize>().map_err(|_| {
                    OperationError::ProcessingError("Invalid array length".to_string())
                })?;
                self.expect("{")?;
                let mut elements = Vec::new();
                for _ in 0..items_count {
                    let key = self.parse()?;
                    let value = self.parse()?;

                    if self.valid_json {
                        // In valid JSON, keys must be strings.
                        // If the key is already a string (e.g. "a"), it will be returned as "\"a\"" by parse().
                        // If it's an integer (e.g. 10), it will be returned as "10".
                        if !key.starts_with('"') {
                            elements.push(format!("\"{}\": {}", key, value));
                        } else {
                            elements.push(format!("{}: {}", key, value));
                        }
                    } else {
                        elements.push(format!("{}: {}", key, value));
                    }
                }
                self.expect("}")?;
                Ok(format!("{{{}}}", elements.join(",")))
            }
            "s" => {
                self.expect(":")?;
                let length = self.read_until(':')?.parse::<usize>().map_err(|_| {
                    OperationError::ProcessingError("Invalid string length".to_string())
                })?;
                self.expect("\"")?;
                let value = self.read(length)?;
                self.expect("\";")?;
                if self.valid_json {
                    Ok(format!(
                        "\"{}\"",
                        value.replace('\\', "\\\\").replace('"', "\\\"")
                    ))
                } else {
                    Ok(format!("\"{}\"", value))
                }
            }
            _ => Err(OperationError::ProcessingError(format!(
                "Unknown type: {}",
                kind
            ))),
        }
    }
}
