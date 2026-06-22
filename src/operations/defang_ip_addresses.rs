/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Defang IP Addresses operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Defang IP Addresses operation
///
/// Takes an IPv4 or IPv6 address and 'Defangs' it, meaning the IP becomes
/// invalid, removing the risk of accidentally utilising it as an IP address.
pub struct DefangIPAddresses;

impl Operation for DefangIPAddresses {
    fn name(&self) -> &'static str {
        "Defang IP Addresses"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Takes an IPv4 or IPv6 address and 'Defangs' it, meaning the IP becomes invalid, removing the risk of accidentally utilising it as an IP address."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);

        // Broad regex to find potential IP addresses (v4 or v6)
        let ip_regex = regex::Regex::new(r"(?i)[a-f\d.:]+(?:/\d+)?")
            .map_err(|e| OperationError::ProcessingError(format!("Invalid regex: {}", e)))?;

        let result = ip_regex.replace_all(&input_str, |caps: &regex::Captures| {
            let matched = &caps[0];
            let (ip_part, cidr_part) = match matched.find('/') {
                Some(idx) => (&matched[..idx], &matched[idx..]),
                None => (matched, ""),
            };

            if let Ok(_ipv4) = ip_part.parse::<std::net::Ipv4Addr>() {
                return format!("{}{}", ip_part.replace('.', "[.]"), cidr_part);
            }
            if let Ok(_ipv6) = ip_part.parse::<std::net::Ipv6Addr>() {
                return format!("{}{}", ip_part.replace(':', "[:]"), cidr_part);
            }
            matched.to_string()
        });

        Ok(result.into_owned().into_bytes())
    }
}
