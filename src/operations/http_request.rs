/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the HTTP request operation.
 * -----------------------------------------------------------------------------
 */

use std::str::FromStr;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// HTTP request operation
pub struct HTTPRequest;

impl Operation for HTTPRequest {
    fn name(&self) -> &'static str {
        "HTTP request"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Makes an HTTP request and returns the response."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Method",
                description: "HTTP method",
                default_value: "GET",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "URL",
                description: "The URL to request",
                default_value: "",
                kind: crate::operation::ArgKind::Url,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Headers",
                description: "Request headers (Key: Value)",
                default_value: "",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Mode",
                description: "CORS mode (ignored in Rust)",
                default_value: "Cross-Origin Resource Sharing",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Show response metadata",
                description: "Include status and headers in output",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
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

    fn input_requirement(&self) -> crate::operation::InputRequirement {
        crate::operation::InputRequirement::Optional
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    /// Issues a live HTTP request.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Network]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let method_str = args.first().and_then(|v| v.as_str()).unwrap_or("GET");
        let url = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let headers_text = args.get(2).and_then(|v| v.as_str()).unwrap_or("");
        let show_metadata = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);

        if url.is_empty() {
            return Ok(Vec::new());
        }

        let client = Client::new();
        let method =
            reqwest::Method::from_str(method_str).map_err(|_| OperationError::InvalidArgument {
                name: "Method".to_string(),
                reason: format!("Invalid method: {}", method_str),
            })?;

        let mut headers = HeaderMap::new();
        for line in headers_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = HeaderName::from_str(parts[0].trim()).map_err(|e| {
                    OperationError::InvalidArgument {
                        name: "Headers".to_string(),
                        reason: format!("Invalid header name: {}", e),
                    }
                })?;
                let value = HeaderValue::from_str(parts[1].trim()).map_err(|e| {
                    OperationError::InvalidArgument {
                        name: "Headers".to_string(),
                        reason: format!("Invalid header value: {}", e),
                    }
                })?;
                headers.insert(key, value);
            }
        }

        let mut request = client.request(method, url).headers(headers);

        if method_str != "GET" && method_str != "HEAD" {
            request = request.body(input);
        }

        let response = request
            .send()
            .map_err(|e| OperationError::ProcessingError(format!("Request failed: {}", e)))?;

        let status = response.status();
        let resp_headers = response.headers().clone();
        let body = response
            .bytes()
            .map_err(|e| OperationError::ProcessingError(format!("Failed to read body: {}", e)))?;

        if show_metadata {
            let mut meta = format!("####\n  Status: {}\n  Exposed headers:\n", status);
            for (key, value) in resp_headers.iter() {
                meta.push_str(&format!(
                    "    {}: {}\n",
                    key,
                    value.to_str().unwrap_or("[invalid]")
                ));
            }
            meta.push_str("####\n\n");
            let mut result = meta.into_bytes();
            result.extend_from_slice(&body);
            Ok(result)
        } else {
            Ok(body.to_vec())
        }
    }
}
