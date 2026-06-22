/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Sort operation.
 * -----------------------------------------------------------------------------
 */

use std::{cmp::Ordering, net::Ipv4Addr};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Sort operation
///
/// Alphabetically (or numerically / by IP) sorts strings separated by
/// the specified delimiter.
pub struct Sort;

/// Expand common delimiter names used in CyberChef.
fn char_rep(s: &str) -> String {
    match s {
        "Line feed" => "\n".to_string(),
        "CRLF" => "\r\n".to_string(),
        "Space" => " ".to_string(),
        "Comma" => ",".to_string(),
        "Semi-colon" => ";".to_string(),
        "Colon" => ":".to_string(),
        "Tab" => "\t".to_string(),
        "Forward slash" => "/".to_string(),
        "Backslash" => "\\".to_string(),
        "0x" => "0x".to_string(),
        "None" => "".to_string(),
        other => other
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t"),
    }
}

/// Parse a dotted-decimal IPv4 string into a u32 for numeric comparison.
fn ipv4_to_u32(s: &str) -> Option<u32> {
    s.trim().parse::<Ipv4Addr>().ok().map(|ip| {
        let oct = ip.octets();
        ((oct[0] as u32) << 24) | ((oct[1] as u32) << 16) | ((oct[2] as u32) << 8) | (oct[3] as u32)
    })
}

/// Compare two strings as IPv4 addresses; fall back to lexicographic if parsing fails.
fn ip_sort(a: &str, b: &str) -> Ordering {
    match (ipv4_to_u32(a), ipv4_to_u32(b)) {
        (Some(na), Some(nb)) => na.cmp(&nb),
        _ => a.cmp(b),
    }
}

/// Compare two strings as numbers (f64); fall back to lexicographic.
fn numeric_sort(a: &str, b: &str) -> Ordering {
    let na = a.trim().parse::<f64>();
    let nb = b.trim().parse::<f64>();
    match (na, nb) {
        (Ok(fa), Ok(fb)) => fa.partial_cmp(&fb).unwrap_or(Ordering::Equal),
        _ => a.cmp(b),
    }
}

/// Compare two strings as hexadecimal numbers; fall back to lexicographic.
fn hex_sort(a: &str, b: &str) -> Ordering {
    let parse_hex = |s: &str| -> Option<u64> {
        let s = s.trim().trim_start_matches("0x").trim_start_matches("0X");
        u64::from_str_radix(s, 16).ok()
    };
    match (parse_hex(a), parse_hex(b)) {
        (Some(na), Some(nb)) => na.cmp(&nb),
        _ => a.cmp(b),
    }
}

/// Compare two strings by length, then lexicographically for equal-length.
fn length_sort(a: &str, b: &str) -> Ordering {
    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}

impl Operation for Sort {
    fn name(&self) -> &'static str {
        "Sort"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Alphabetically sorts strings separated by the specified delimiter. The IP address option supports IPv4 only."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "Line feed, CRLF, Space, Comma, Semi-colon, Colon, Tab, None",
                default_value: "Line feed",
            },
            ArgSchema {
                name: "Reverse",
                description: "Sort in reverse order",
                default_value: "false",
            },
            ArgSchema {
                name: "Order",
                description: "Alphabetical (case sensitive), Alphabetical (case insensitive), IP address, Numeric, Numeric (hexadecimal), Length",
                default_value: "Alphabetical (case sensitive)",
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
        let text = String::from_utf8_lossy(&input).into_owned();

        let delim_name = args.first().and_then(|a| a.as_str()).unwrap_or("Line feed");
        let reverse = args.get(1).and_then(|a| a.as_bool()).unwrap_or(false);
        let order = args
            .get(2)
            .and_then(|a| a.as_str())
            .unwrap_or("Alphabetical (case sensitive)");

        let delim = char_rep(delim_name);

        let mut parts: Vec<String> = if delim.is_empty() {
            text.chars().map(|c| c.to_string()).collect()
        } else {
            text.split(delim.as_str()).map(|s| s.to_string()).collect()
        };

        match order {
            "Alphabetical (case sensitive)" => {
                parts.sort();
            }
            "Alphabetical (case insensitive)" => {
                parts.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            }
            "IP address" => {
                parts.sort_by(|a, b| ip_sort(a, b));
            }
            "Numeric" => {
                parts.sort_by(|a, b| numeric_sort(a, b));
            }
            "Numeric (hexadecimal)" => {
                parts.sort_by(|a, b| hex_sort(a, b));
            }
            "Length" => {
                parts.sort_by(|a, b| length_sort(a, b));
            }
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Order".to_string(),
                    reason: format!("Unknown sort order: {}", other),
                });
            }
        }

        if reverse {
            parts.reverse();
        }

        let output = parts.join(delim.as_str());
        Ok(output.into_bytes())
    }
}
