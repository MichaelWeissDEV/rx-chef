/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the DNS over HTTPS operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// DNS over HTTPS operation
pub struct DnsOverHttps;

impl Operation for DnsOverHttps {
    fn name(&self) -> &'static str {
        "DNS over HTTPS"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Takes a single domain name and performs a DNS lookup using DNS over HTTPS.\n\n\
        By default, Cloudflare and Google DNS over HTTPS services are supported.\n\n\
        Can be used with any service that supports the GET parameters `name` and `type`."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Resolver",
                description: "The DNS over HTTPS resolver URL (e.g., Google or Cloudflare).",
                default_value: "https://cloudflare-dns.com/dns-query",
            },
            ArgSchema {
                name: "Request Type",
                description: "The type of DNS request (A, AAAA, TXT, etc.).",
                default_value: "A",
            },
            ArgSchema {
                name: "Answer Data Only",
                description: "If true, only returns the Answer section data values.",
                default_value: "false",
            },
            ArgSchema {
                name: "Disable DNSSEC validation",
                description: "Disable DNSSEC validation (Checking Disabled).",
                default_value: "false",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let resolver = args
            .get(0)
            .and_then(|a| a.as_str())
            .unwrap_or("https://cloudflare-dns.com/dns-query");
        let request_type = args.get(1).and_then(|a| a.as_str()).unwrap_or("A");
        let just_answer = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);
        let disable_dnssec = args.get(3).and_then(|a| a.as_bool()).unwrap_or(false);

        let domain = String::from_utf8_lossy(&input);
        if domain.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut url = url::Url::parse(resolver).map_err(|e| OperationError::InvalidArgument {
            name: "Resolver".to_string(),
            reason: format!("Invalid Resolver URL: {}", e),
        })?;

        url.query_pairs_mut()
            .append_pair("name", domain.trim())
            .append_pair("type", request_type)
            .append_pair("cd", if disable_dnssec { "true" } else { "false" });

        // reqwest is expected to be available for making HTTP requests
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url.clone())
            .header("Accept", "application/dns-json")
            .send()
            .map_err(|e| {
                OperationError::ProcessingError(format!("Error making request to {}: {}", url, e))
            })?;

        let status = response.status();
        if !status.is_success() {
            return Err(OperationError::ProcessingError(format!(
                "HTTP request failed with status: {}",
                status
            )));
        }

        let data: serde_json::Value = response.json().map_err(|e| {
            OperationError::ProcessingError(format!("Error parsing JSON response: {}", e))
        })?;

        if just_answer {
            if let Some(answers) = data.get("Answer").and_then(|a| a.as_array()) {
                let mut extracted = Vec::new();
                for answer in answers {
                    if let Some(d) = answer.get("data") {
                        extracted.push(d.clone());
                    }
                }
                let extracted_json = serde_json::Value::Array(extracted);
                return serde_json::to_vec_pretty(&extracted_json).map_err(|e| {
                    OperationError::ProcessingError(format!("Error serializing response: {}", e))
                });
            } else {
                return Ok(b"[]".to_vec());
            }
        }

        serde_json::to_vec_pretty(&data).map_err(|e| {
            OperationError::ProcessingError(format!("Error serializing response: {}", e))
        })
    }
}
