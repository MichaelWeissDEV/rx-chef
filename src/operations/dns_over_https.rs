/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the DNS over HTTPS operation.
 * -----------------------------------------------------------------------------
 */

use std::{io::Read, time::Duration};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

const MAX_RESPONSE_BYTES: u64 = 8 * 1024 * 1024;

fn resolver_url(
    resolver: &str,
    domain: &str,
    request_type: &str,
    disable_dnssec: bool,
) -> Result<url::Url, OperationError> {
    let mut url = url::Url::parse(resolver).map_err(|e| OperationError::InvalidArgument {
        name: "Resolver".to_string(),
        reason: format!("Invalid Resolver URL: {}", e),
    })?;
    url.query_pairs_mut()
        .append_pair("name", domain.trim())
        .append_pair("type", request_type)
        .append_pair("cd", if disable_dnssec { "true" } else { "false" });
    Ok(url)
}

fn format_response(body: &[u8], just_answer: bool) -> Result<Vec<u8>, OperationError> {
    let data: serde_json::Value = serde_json::from_slice(body).map_err(|e| {
        OperationError::ProcessingError(format!("Error parsing JSON response: {}", e))
    })?;

    if just_answer {
        let extracted = data
            .get("Answer")
            .and_then(|answers| answers.as_array())
            .map(|answers| {
                answers
                    .iter()
                    .filter_map(|answer| answer.get("data").cloned())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        return serde_json::to_vec_pretty(&serde_json::Value::Array(extracted)).map_err(|e| {
            OperationError::ProcessingError(format!("Error serializing response: {}", e))
        });
    }

    serde_json::to_vec_pretty(&data)
        .map_err(|e| OperationError::ProcessingError(format!("Error serializing response: {}", e)))
}

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
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Request Type",
                description: "The type of DNS request (A, AAAA, TXT, etc.).",
                default_value: "A",
                kind: crate::operation::ArgKind::String,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Answer Data Only",
                description: "If true, only returns the Answer section data values.",
                default_value: "false",
                kind: crate::operation::ArgKind::Boolean,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
            ArgSchema {
                name: "Disable DNSSEC validation",
                description: "Disable DNSSEC validation (Checking Disabled).",
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

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    /// Queries a remote DNS-over-HTTPS resolver.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Network]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
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
            return Ok(b"{}".to_vec());
        }

        let url = resolver_url(resolver, &domain, request_type, disable_dnssec)?;

        // reqwest is expected to be available for making HTTP requests
        let client = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .map_err(|error| OperationError::ProcessingError(error.to_string()))?;
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

        if response
            .content_length()
            .is_some_and(|length| length > MAX_RESPONSE_BYTES)
        {
            return Err(OperationError::ProcessingError(
                "DNS response exceeds the 8 MiB limit".to_string(),
            ));
        }
        let mut body = Vec::new();
        response
            .take(MAX_RESPONSE_BYTES + 1)
            .read_to_end(&mut body)
            .map_err(|e| OperationError::ProcessingError(format!("Error reading response: {e}")))?;
        if body.len() as u64 > MAX_RESPONSE_BYTES {
            return Err(OperationError::ProcessingError(
                "DNS response exceeds the 8 MiB limit".to_string(),
            ));
        }
        format_response(&body, just_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::{format_response, resolver_url};

    const RESPONSE: &[u8] =
        br#"{"Status":0,"Answer":[{"name":"example.com.","type":1,"TTL":60,"data":"192.0.2.1"}]}"#;

    #[test]
    fn resolver_url_encodes_query_and_dnssec_flag() {
        let url = resolver_url(
            "https://resolver.example/dns-query",
            "example.com",
            "AAAA",
            true,
        )
        .unwrap();
        assert_eq!(
            url.as_str(),
            "https://resolver.example/dns-query?name=example.com&type=AAAA&cd=true"
        );
        let pairs = url.query_pairs().collect::<Vec<_>>();
        assert!(pairs
            .iter()
            .any(|pair| pair.0 == "name" && pair.1 == "example.com"));
        assert!(pairs
            .iter()
            .any(|pair| pair.0 == "type" && pair.1 == "AAAA"));
        assert!(pairs.iter().any(|pair| pair.0 == "cd" && pair.1 == "true"));
    }

    #[test]
    fn response_formatter_preserves_document_or_extracts_answers() {
        let complete: serde_json::Value =
            serde_json::from_slice(&format_response(RESPONSE, false).unwrap()).unwrap();
        assert_eq!(complete["Status"], 0);
        assert_eq!(complete["Answer"][0]["name"], "example.com.");

        let answers: serde_json::Value =
            serde_json::from_slice(&format_response(RESPONSE, true).unwrap()).unwrap();
        assert_eq!(answers, serde_json::json!(["192.0.2.1"]));
        assert_eq!(
            format_response(RESPONSE, true).unwrap(),
            b"[\n  \"192.0.2.1\"\n]"
        );
    }

    #[test]
    fn response_formatter_rejects_invalid_json() {
        assert!(format_response(b"not json", false).is_err());
    }
}
