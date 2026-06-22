/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse IP range operation.
 * -----------------------------------------------------------------------------
 */

use std::net::Ipv4Addr;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

const LARGE_QUERY_LIMIT: u64 = 65536;

/// Parse IP Range operation
///
/// Given a CIDR range (e.g. 10.0.0.0/24), a hyphenated range
/// (e.g. 10.0.0.1 - 10.0.0.50), or a single IPv4 address, enumerates
/// all IP addresses in the range and shows network information.
/// IPv6 is not enumerated (too large).
pub struct ParseIPRange;

fn ipv4_to_u32(addr: Ipv4Addr) -> u32 {
    u32::from(addr)
}

fn u32_to_ipv4(n: u32) -> Ipv4Addr {
    Ipv4Addr::from(n)
}

fn parse_ipv4(s: &str) -> Result<Ipv4Addr, OperationError> {
    s.trim()
        .parse::<Ipv4Addr>()
        .map_err(|_| OperationError::InvalidInput(format!("Invalid IPv4 address: '{}'", s)))
}

fn cidr_to_mask(prefix: u32) -> u32 {
    if prefix == 0 {
        0u32
    } else {
        !0u32 << (32 - prefix)
    }
}

fn ipv4_cidr_range(
    ip_str: &str,
    prefix: u32,
    include_info: bool,
    enumerate: bool,
    allow_large: bool,
) -> Result<String, OperationError> {
    let base_ip = parse_ipv4(ip_str)?;
    let base = ipv4_to_u32(base_ip);
    let mask = cidr_to_mask(prefix);
    let network = base & mask;
    let broadcast = network | !mask;
    let host_count = if prefix >= 32 {
        1u64
    } else {
        (broadcast - network) as u64 + 1
    };

    let mut out = String::new();

    if include_info {
        out.push_str(&format!(
            "Network: {}/{}\nCIDR Notation: {}/{}\nMask: {}\nHosts: {}\nFirst: {}\nLast: {}\n\n",
            u32_to_ipv4(network),
            prefix,
            u32_to_ipv4(network),
            prefix,
            u32_to_ipv4(mask),
            host_count,
            u32_to_ipv4(network),
            u32_to_ipv4(broadcast),
        ));
    }

    if enumerate {
        if host_count > LARGE_QUERY_LIMIT && !allow_large {
            return Err(OperationError::ProcessingError(format!(
                "The list of {} addresses is too large. Enable 'Allow large queries' to proceed.",
                host_count
            )));
        }
        for addr in network..=broadcast {
            out.push_str(&format!("{}\n", u32_to_ipv4(addr)));
        }
        // Remove trailing newline
        if out.ends_with('\n') {
            out.pop();
        }
    }

    Ok(out)
}

fn ipv4_hyphenated_range(
    start_str: &str,
    end_str: &str,
    include_info: bool,
    enumerate: bool,
    allow_large: bool,
) -> Result<String, OperationError> {
    let start_ip = parse_ipv4(start_str)?;
    let end_ip = parse_ipv4(end_str)?;
    let start = ipv4_to_u32(start_ip);
    let end = ipv4_to_u32(end_ip);

    if start > end {
        return Err(OperationError::InvalidInput(
            "Start address must not be greater than end address".to_string(),
        ));
    }

    let count = (end - start) as u64 + 1;
    let mut out = String::new();

    if include_info {
        out.push_str(&format!(
            "Start: {}\nEnd: {}\nAddresses: {}\n\n",
            start_ip, end_ip, count
        ));
    }

    if enumerate {
        if count > LARGE_QUERY_LIMIT && !allow_large {
            return Err(OperationError::ProcessingError(format!(
                "The list of {} addresses is too large. Enable 'Allow large queries' to proceed.",
                count
            )));
        }
        for addr in start..=end {
            out.push_str(&format!("{}\n", u32_to_ipv4(addr)));
        }
        if out.ends_with('\n') {
            out.pop();
        }
    }

    Ok(out)
}

impl Operation for ParseIPRange {
    fn name(&self) -> &'static str {
        "Parse IP range"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Given a CIDR range (e.g. 10.0.0.0/24), a hyphenated range (e.g. 10.0.0.0 - 10.0.1.0), or a single IP address, this operation provides network information and enumerates all IP addresses in the range. IPv6 is supported but not enumerated."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Include network info",
                description: "Show network/mask/host information",
                default_value: "true",
            },
            ArgSchema {
                name: "Enumerate IP addresses",
                description: "List every IP in the range",
                default_value: "true",
            },
            ArgSchema {
                name: "Allow large queries",
                description: "Allow ranges larger than 65536",
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
        let text = String::from_utf8_lossy(&input);
        let text = text.trim();

        let include_info = args.first().and_then(|a| a.as_bool()).unwrap_or(true);

        let enumerate = args.get(1).and_then(|a| a.as_bool()).unwrap_or(true);

        let allow_large = args.get(2).and_then(|a| a.as_bool()).unwrap_or(false);

        // CIDR notation
        if let Some(slash) = text.find('/') {
            let ip_str = text[..slash].trim();
            let prefix_str = text[slash + 1..].trim();

            // Check if IPv6
            if ip_str.contains(':') {
                // IPv6 CIDR - show info only, no enumeration
                let prefix: u32 = prefix_str.parse().map_err(|_| {
                    OperationError::InvalidInput("Invalid prefix length".to_string())
                })?;
                let addr: std::net::Ipv6Addr = ip_str
                    .parse()
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid IPv6: {}", e)))?;
                let out = if include_info {
                    format!(
                        "Network: {}/{}\nNote: IPv6 enumeration is not supported.\n",
                        addr, prefix
                    )
                } else {
                    format!("{}/{}\n", addr, prefix)
                };
                return Ok(out.into_bytes());
            }

            let prefix: u32 = prefix_str
                .parse()
                .map_err(|_| OperationError::InvalidInput("Invalid prefix length".to_string()))?;
            if prefix > 32 {
                return Err(OperationError::InvalidInput(
                    "IPv4 prefix must be 0-32".to_string(),
                ));
            }
            let result = ipv4_cidr_range(ip_str, prefix, include_info, enumerate, allow_large)?;
            return Ok(result.into_bytes());
        }

        // Hyphenated range
        if text.contains('-') {
            // Try to detect if it looks like "IP - IP"
            let parts: Vec<&str> = text.splitn(2, '-').collect();
            if parts.len() == 2 {
                let start = parts[0].trim();
                let end = parts[1].trim();
                if start.contains('.') && end.contains('.') {
                    let result =
                        ipv4_hyphenated_range(start, end, include_info, enumerate, allow_large)?;
                    return Ok(result.into_bytes());
                }
            }
        }

        // Single IP
        if text.contains(':') {
            // IPv6 single address
            let addr: std::net::Ipv6Addr = text.parse().map_err(|e| {
                OperationError::InvalidInput(format!("Invalid IPv6 address: {}", e))
            })?;
            return Ok(format!("{}\n", addr).into_bytes());
        }

        // IPv4 single address
        let addr = parse_ipv4(text)?;
        let result = ipv4_cidr_range(&addr.to_string(), 32, include_info, enumerate, allow_large)?;
        Ok(result.into_bytes())
    }
}
