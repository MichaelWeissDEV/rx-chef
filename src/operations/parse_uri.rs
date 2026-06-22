/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse URI operation.
 * -----------------------------------------------------------------------------
 */

use url::Url;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse URI operation
pub struct ParseURI;

impl Operation for ParseURI {
    fn name(&self) -> &'static str {
        "Parse URI"
    }

    fn module(&self) -> &'static str {
        "URL"
    }

    fn description(&self) -> &'static str {
        "Pretty prints complicated Uniform Resource Identifier (URI) strings for ease of reading. Particularly useful for Uniform Resource Locators (URLs) with a lot of arguments."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let input_str = input_str.trim();

        let u = Url::parse(input_str)
            .map_err(|e| OperationError::InvalidInput(format!("Failed to parse URI: {}", e)))?;

        let mut output = String::new();

        if !u.scheme().is_empty() {
            output += &format!("Protocol:\t{}\n", u.scheme());
        }

        if let Some(password) = u.password() {
            output += &format!("Auth:\t\t{}:{}\n", u.username(), password);
        } else if !u.username().is_empty() {
            output += &format!("Auth:\t\t{}\n", u.username());
        }

        if let Some(host) = u.host_str() {
            output += &format!("Hostname:\t{}\n", host);
        }

        if let Some(port) = u.port() {
            output += &format!("Port:\t\t{}\n", port);
        }

        if !u.path().is_empty() {
            output += &format!("Path name:\t{}\n", u.path());
        }

        let query_pairs: Vec<_> = u.query_pairs().collect();
        if !query_pairs.is_empty() {
            let mut padding = 0;
            for (k, _) in &query_pairs {
                if k.len() > padding {
                    padding = k.len();
                }
            }

            output += "Arguments:\n";
            for (k, v) in &query_pairs {
                output += &format!("\t{:width$}", k, width = padding);
                if !v.is_empty() {
                    output += &format!(" = {}\n", v);
                } else {
                    output += "\n";
                }
            }
        }

        if let Some(fragment) = u.fragment() {
            output += &format!("Hash:\t\t#{}\n", fragment);
        }

        Ok(output.into_bytes())
    }
}
