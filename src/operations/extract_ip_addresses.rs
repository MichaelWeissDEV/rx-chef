/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Extract IP addresses operation.
 * -----------------------------------------------------------------------------
 */

use std::net::IpAddr;

use itertools::Itertools;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Extract IP Addresses operation
pub struct ExtractIPAddresses;

impl Operation for ExtractIPAddresses {
    fn name(&self) -> &'static str {
        "Extract IP addresses"
    }

    fn module(&self) -> &'static str {
        "Regex"
    }

    fn description(&self) -> &'static str {
        "Extracts all IPv4 and IPv6 addresses.\n\nWarning: Given a string 1.2.3.4.5.6.7.8, this will match 1.2.3.4 and 5.6.7.8 so always check the original input!"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "IPv4",
                description: "Include IPv4 addresses",
                default_value: "true",
            },
            ArgSchema {
                name: "IPv6",
                description: "Include IPv6 addresses",
                default_value: "false",
            },
            ArgSchema {
                name: "Remove local IPv4 addresses",
                description: "Exclude local/private IPv4 addresses",
                default_value: "false",
            },
            ArgSchema {
                name: "Display total",
                description: "Display the total number of addresses found",
                default_value: "false",
            },
            ArgSchema {
                name: "Sort",
                description: "Sort the results",
                default_value: "false",
            },
            ArgSchema {
                name: "Unique",
                description: "Remove duplicate results",
                default_value: "false",
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
        let include_ipv4 = match args.first() {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() != "false",
            _ => true,
        };
        let include_ipv6 = match args.get(1) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() == "true",
            _ => false,
        };
        let remove_local = match args.get(2) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() == "true",
            _ => false,
        };
        let display_total = match args.get(3) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() == "true",
            _ => false,
        };
        let sort = match args.get(4) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() == "true",
            _ => false,
        };
        let unique = match args.get(5) {
            Some(ArgValue::Bool(b)) => *b,
            Some(ArgValue::Str(s)) => s.to_lowercase() == "true",
            _ => false,
        };

        let input_str = String::from_utf8_lossy(&input);
        let mut results = Vec::new();

        if include_ipv4 {
            let ipv4_regex = Regex::new(r"(?:\d{1,3}\.){3}\d{1,3}").unwrap();
            for m in ipv4_regex.find_iter(&input_str) {
                if m.as_str().parse::<std::net::Ipv4Addr>().is_ok() {
                    results.push(m.as_str().to_string());
                }
            }
        }

        if include_ipv6 {
            let ipv6_regex = Regex::new(r"(?i)[a-f\d:]+:[a-f\d:]+").unwrap();
            for m in ipv6_regex.find_iter(&input_str) {
                let s = m.as_str();
                if s.parse::<std::net::Ipv6Addr>().is_ok() {
                    results.push(s.to_string());
                }
            }
        }

        if results.is_empty() {
            return Ok(Vec::new());
        }

        if remove_local {
            let local_patterns = vec![
                r"^10\..+",
                r"^192\.168\..+",
                r"^172\.(?:1[6-9]|2\d|3[01])\..+",
                r"^127\..+",
            ];
            let local_regex = Regex::new(&format!("(?:{})", local_patterns.join("|"))).unwrap();
            results.retain(|ip| !local_regex.is_match(ip));
        }

        if unique {
            results = results.into_iter().unique().collect();
        }

        if sort {
            results.sort_by(|a, b| {
                let addr_a = a.parse::<IpAddr>();
                let addr_b = b.parse::<IpAddr>();
                match (addr_a, addr_b) {
                    (Ok(aa), Ok(ab)) => aa.cmp(&ab),
                    _ => a.cmp(b),
                }
            });
        }

        let mut output = String::new();
        if display_total {
            output.push_str(&format!("Total found: {}\n\n", results.len()));
        }
        output.push_str(&results.join("\n"));

        Ok(output.into_bytes())
    }
}
