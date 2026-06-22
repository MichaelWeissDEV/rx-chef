/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Rison Decode operation.
 * -----------------------------------------------------------------------------
 */

use serde_json::{Map, Number, Value};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Rison Decode operation
pub struct RisonDecode;

impl Operation for RisonDecode {
    fn name(&self) -> &'static str {
        "Rison Decode"
    }

    fn module(&self) -> &'static str {
        "Encodings"
    }

    fn description(&self) -> &'static str {
        "Rison, a data serialization format optimized for compactness in URIs. Rison is a slight variation of JSON that looks vastly superior after URI encoding. Rison still expresses exactly the same set of data structures as JSON, so data can be translated back and forth without loss or guesswork."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Decode Option",
            description: "Decode, Decode Object, or Decode Array",
            default_value: "Decode",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let decode_option = args.first().and_then(|a| a.as_str()).unwrap_or("Decode");

        let mut parser = RisonParser::new(&input_str);
        let value = match decode_option {
            "Decode" => parser.parse_value()?,
            "Decode Object" => parser.parse_object()?,
            "Decode Array" => parser.parse_array()?,
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Decode Option".to_string(),
                    reason: format!("Invalid decode option: {}", decode_option),
                })
            }
        };

        serde_json::to_vec_pretty(&value).map_err(|e| {
            OperationError::ProcessingError(format!("Failed to serialize to JSON: {}", e))
        })
    }
}

struct RisonParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> RisonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos..).and_then(|s| s.chars().next())
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn parse_value(&mut self) -> Result<Value, OperationError> {
        match self.peek() {
            Some('(') => self.parse_object(),
            Some('!') => {
                self.next();
                match self.peek() {
                    Some('t') => {
                        self.next();
                        Ok(Value::Bool(true))
                    }
                    Some('f') => {
                        self.next();
                        Ok(Value::Bool(false))
                    }
                    Some('n') => {
                        self.next();
                        Ok(Value::Null)
                    }
                    Some('(') => self.parse_array_after_bang(),
                    _ => Err(OperationError::InvalidInput(
                        "Invalid literal after '!'".to_string(),
                    )),
                }
            }
            Some('\'') => self.parse_quoted_string(),
            Some(c) if c.is_ascii_digit() || c == '-' || c == '.' => self.parse_number(),
            Some(_) => self.parse_simple_string(),
            None => Err(OperationError::InvalidInput(
                "Unexpected end of input".to_string(),
            )),
        }
    }

    fn parse_object(&mut self) -> Result<Value, OperationError> {
        if self.next() != Some('(') {
            return Err(OperationError::InvalidInput(
                "Expected '(' at start of object".to_string(),
            ));
        }

        let mut map = Map::new();
        loop {
            match self.peek() {
                Some(')') => {
                    self.next();
                    break;
                }
                None => {
                    return Err(OperationError::InvalidInput(
                        "Unexpected end of input in object".to_string(),
                    ))
                }
                _ => {
                    let key = self.parse_key()?;
                    if self.next() != Some(':') {
                        return Err(OperationError::InvalidInput(
                            "Expected ':' after key".to_string(),
                        ));
                    }
                    let val = self.parse_value()?;
                    map.insert(key, val);

                    match self.peek() {
                        Some(',') => {
                            self.next();
                        }
                        Some(')') => {}
                        _ => {
                            return Err(OperationError::InvalidInput(
                                "Expected ',' or ')' in object".to_string(),
                            ))
                        }
                    }
                }
            }
        }
        Ok(Value::Object(map))
    }

    fn parse_array_after_bang(&mut self) -> Result<Value, OperationError> {
        if self.next() != Some('(') {
            return Err(OperationError::InvalidInput(
                "Expected '(' after '!' for array".to_string(),
            ));
        }
        self.parse_array_contents()
    }

    fn parse_array(&mut self) -> Result<Value, OperationError> {
        // Handle both !() and just () if called from "Decode Array"
        if self.peek() == Some('!') {
            self.next();
        }
        if self.next() != Some('(') {
            return Err(OperationError::InvalidInput(
                "Expected '(' for array".to_string(),
            ));
        }
        self.parse_array_contents()
    }

    fn parse_array_contents(&mut self) -> Result<Value, OperationError> {
        let mut arr = Vec::new();
        loop {
            match self.peek() {
                Some(')') => {
                    self.next();
                    break;
                }
                None => {
                    return Err(OperationError::InvalidInput(
                        "Unexpected end of input in array".to_string(),
                    ))
                }
                _ => {
                    arr.push(self.parse_value()?);
                    match self.peek() {
                        Some(',') => {
                            self.next();
                        }
                        Some(')') => {}
                        _ => {
                            return Err(OperationError::InvalidInput(
                                "Expected ',' or ')' in array".to_string(),
                            ))
                        }
                    }
                }
            }
        }
        Ok(Value::Array(arr))
    }

    fn parse_key(&mut self) -> Result<String, OperationError> {
        match self.peek() {
            Some('\'') => {
                if let Value::String(s) = self.parse_quoted_string()? {
                    Ok(s)
                } else {
                    unreachable!()
                }
            }
            _ => self.parse_simple_string_raw(),
        }
    }

    fn parse_quoted_string(&mut self) -> Result<Value, OperationError> {
        self.next(); // skip '
        let mut s = String::new();
        while let Some(c) = self.next() {
            if c == '\'' {
                return Ok(Value::String(s));
            } else if c == '!' {
                match self.next() {
                    Some('!') => s.push('!'),
                    Some('\'') => s.push('\''),
                    Some(ec) => s.push(ec),
                    None => {
                        return Err(OperationError::InvalidInput(
                            "Unexpected end of input after '!'".to_string(),
                        ))
                    }
                }
            } else {
                s.push(c);
            }
        }
        Err(OperationError::InvalidInput(
            "Unterminated quoted string".to_string(),
        ))
    }

    fn parse_simple_string(&mut self) -> Result<Value, OperationError> {
        self.parse_simple_string_raw().map(Value::String)
    }

    fn parse_simple_string_raw(&mut self) -> Result<String, OperationError> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '/' || c == '-' || c == '.' {
                s.push(self.next().unwrap());
            } else {
                break;
            }
        }
        if s.is_empty() {
            Err(OperationError::InvalidInput(
                "Expected simple string".to_string(),
            ))
        } else {
            Ok(s)
        }
    }

    fn parse_number(&mut self) -> Result<Value, OperationError> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '-' || c == '.' || c == 'e' || c == 'E' || c == '+' {
                s.push(self.next().unwrap());
            } else {
                break;
            }
        }
        let n: f64 = s
            .parse()
            .map_err(|_| OperationError::InvalidInput(format!("Invalid number: {}", s)))?;
        if let Some(num) = Number::from_f64(n) {
            Ok(Value::Number(num))
        } else {
            Err(OperationError::InvalidInput(format!(
                "Invalid float: {}",
                n
            )))
        }
    }
}
