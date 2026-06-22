/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Group IP addresses operation.
 * -----------------------------------------------------------------------------
 */

use std::{
    collections::BTreeMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Group IP addresses operation
///
/// Groups a list of IP addresses into subnets. Supports both IPv4 and IPv6 addresses.
pub struct GroupIPAddresses;

impl Operation for GroupIPAddresses {
    fn name(&self) -> &'static str {
        "Group IP addresses"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Groups a list of IP addresses into subnets. Supports both IPv4 and IPv6 addresses."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Delimiter",
                description: "The delimiter between IP addresses",
                default_value: "Line feed",
            },
            ArgSchema {
                name: "Subnet (CIDR)",
                description: "The CIDR subnet mask to group by",
                default_value: "24",
            },
            ArgSchema {
                name: "Only show the subnets",
                description: "Only show the resulting subnets, not the individual IP addresses",
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
        let delim_str = args.first().and_then(|v| v.as_str()).unwrap_or("Line feed");
        let cidr = args.get(1).and_then(|v| v.as_i64()).unwrap_or(24);
        let only_subnets = args.get(2).and_then(|v| v.as_bool()).unwrap_or(false);

        if cidr < 0 || cidr > 128 {
            return Err(OperationError::InvalidArgument {
                name: "Subnet (CIDR)".to_string(),
                reason: "CIDR must be between 0 and 128".to_string(),
            });
        }

        let delims = match delim_str {
            "Line feed" => vec!["\n"],
            "CRLF" => vec!["\r\n"],
            "Space" => vec![" "],
            "Comma" => vec![","],
            "Semi-colon" => vec![";"],
            _ => vec!["\n"],
        };

        let input_str = String::from_utf8_lossy(&input);

        // Split by the first delimiter found that actually exists in the input,
        // or just split by the specified delimiter.
        // CyberChef's split logic is usually simple.
        let ips: Vec<&str> = if delims.len() == 1 {
            input_str.split(delims[0]).collect()
        } else {
            // This part is for more complex delimiter handling if needed
            input_str.split('\n').collect()
        };

        let mut ipv4_networks: BTreeMap<Ipv4Addr, Vec<Ipv4Addr>> = BTreeMap::new();
        let mut ipv6_networks: BTreeMap<Ipv6Addr, Vec<Ipv6Addr>> = BTreeMap::new();

        for ip_str in ips {
            let ip_str = ip_str.trim();
            if ip_str.is_empty() {
                continue;
            }

            // Remove any trailing/leading characters that might be part of the input but not the IP
            let clean_ip_str =
                ip_str.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != ':');

            if let Ok(ip) = clean_ip_str.parse::<IpAddr>() {
                match ip {
                    IpAddr::V4(ipv4) => {
                        if cidr <= 32 {
                            let mask = if cidr == 0 { 0 } else { !0u32 << (32 - cidr) };
                            let network_addr = Ipv4Addr::from(u32::from(ipv4) & mask);
                            ipv4_networks.entry(network_addr).or_default().push(ipv4);
                        }
                    }
                    IpAddr::V6(ipv6) => {
                        let mask = ipv6_mask(cidr as u8);
                        let network_addr = Ipv6Addr::from(u128::from(ipv6) & mask);
                        ipv6_networks.entry(network_addr).or_default().push(ipv6);
                    }
                }
            }
        }

        let mut output = String::new();

        // Print IPv4 Networks
        for (network, mut member_ips) in ipv4_networks {
            member_ips.sort();
            output.push_str(&format!("{}/{}\n", network, cidr));
            if !only_subnets {
                for member in member_ips {
                    output.push_str(&format!("  {}\n", member));
                }
                output.push('\n');
            }
        }

        // Print IPv6 Networks
        for (network, mut member_ips) in ipv6_networks {
            member_ips.sort();
            output.push_str(&format!("{}/{}\n", network, cidr));
            if !only_subnets {
                for member in member_ips {
                    output.push_str(&format!("  {}\n", member));
                }
                output.push('\n');
            }
        }

        Ok(output.trim_end().to_string().into_bytes())
    }
}

fn ipv6_mask(cidr: u8) -> u128 {
    if cidr == 0 {
        0
    } else if cidr >= 128 {
        u128::MAX
    } else {
        u128::MAX << (128 - cidr)
    }
}
