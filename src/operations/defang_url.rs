/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Defang URL operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Defang URL operation
///
/// Takes a Universal Resource Locator (URL) and 'Defangs' it; meaning the
/// URL becomes invalid, neutralising the risk of accidentally clicking on
/// a malicious link.
pub struct DefangURL;

impl Operation for DefangURL {
    fn name(&self) -> &'static str {
        "Defang URL"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Takes a Universal Resource Locator (URL) and 'Defangs' it; meaning the URL becomes invalid, neutralising the risk of accidentally clicking on a malicious link."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Escape dots",
                description: "Escape dots in the URL",
                default_value: "true",
            },
            ArgSchema {
                name: "Escape http",
                description: "Escape http/https in the URL",
                default_value: "true",
            },
            ArgSchema {
                name: "Escape ://",
                description: "Escape :// in the URL",
                default_value: "true",
            },
            ArgSchema {
                name: "Process",
                description: "Process option",
                default_value: "Valid domains and full URLs",
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
        let dots = match args.first() {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() != "false",
            _ => true,
        };
        let http = match args.get(1) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() != "false",
            _ => true,
        };
        let slashes = match args.get(2) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() != "false",
            _ => true,
        };
        let process = args
            .get(3)
            .and_then(|a| a.as_str())
            .unwrap_or("Valid domains and full URLs");

        let input_str = String::from_utf8_lossy(&input);
        let mut result = input_str.to_string();

        match process {
            "Valid domains and full URLs" => {
                // URL regex pattern
                let url_regex = regex::Regex::new(r"(?i)\b((?:https?|ftp)://[^\s/$.?#].[^\s]*)")
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid regex: {}", e))
                    })?;
                result = url_regex
                    .replace_all(&result, |caps: &regex::Captures| {
                        defang_url(&caps[0], dots, http, slashes)
                    })
                    .to_string();

                // Domain regex pattern
                let domain_regex = regex::Regex::new(r"(?i)\b((?:[a-z0-9-]+\.)+[a-z]{2,})\b")
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid regex: {}", e))
                    })?;
                result = domain_regex
                    .replace_all(&result, |caps: &regex::Captures| {
                        defang_url(&caps[0], dots, http, slashes)
                    })
                    .to_string();
            }
            "Only full URLs" => {
                let url_regex = regex::Regex::new(r"(?i)\b((?:https?|ftp)://[^\s/$.?#].[^\s]*)")
                    .map_err(|e| {
                        OperationError::ProcessingError(format!("Invalid regex: {}", e))
                    })?;
                result = url_regex
                    .replace_all(&result, |caps: &regex::Captures| {
                        defang_url(&caps[0], dots, http, slashes)
                    })
                    .to_string();
            }
            "Everything" => {
                result = defang_url(&result, dots, http, slashes);
            }
            _ => {}
        }

        Ok(result.into_bytes())
    }
}

fn defang_url(url: &str, dots: bool, http: bool, slashes: bool) -> String {
    let mut result = url.to_string();
    if dots {
        result = result.replace('.', "[.]");
    }
    if http {
        result = result.replace("http", "hxxp").replace("https", "hxxps");
    }
    if slashes {
        result = result.replace("://", "[://]");
    }
    result
}
