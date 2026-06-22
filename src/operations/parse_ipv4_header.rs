/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse IPv4 header operation.
 * -----------------------------------------------------------------------------
 */

use std::net::Ipv4Addr;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse IPv4 Header operation
///
/// Parses an IPv4 packet header and displays each field in a readable format.
pub struct ParseIPv4Header;

fn protocol_name(proto: u8) -> &'static str {
    match proto {
        1 => "ICMP",
        2 => "IGMP",
        6 => "TCP",
        17 => "UDP",
        41 => "IPv6",
        47 => "GRE",
        50 => "ESP",
        51 => "AH",
        58 => "ICMPv6",
        89 => "OSPF",
        132 => "SCTP",
        _ => "Unknown",
    }
}

/// Compute the one's complement checksum over the given bytes
fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        let word = ((data[i] as u32) << 8) | (data[i + 1] as u32);
        sum += word;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !sum as u16
}

impl Operation for ParseIPv4Header {
    fn name(&self) -> &'static str {
        "Parse IPv4 header"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses an IPv4 packet header and displays each field in a readable format including version, IHL, DSCP, ECN, total length, identification, flags, fragment offset, TTL, protocol, checksum, source and destination IP addresses, and options if present."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input format",
                description: "Hex or Raw",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output format",
                description: "Table, Data (hex), or Data (raw)",
                default_value: "Table",
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
        let input_format = args.first().and_then(|a| a.as_str()).unwrap_or("Hex");

        let output_format = args.get(1).and_then(|a| a.as_str()).unwrap_or("Table");

        let bytes: Vec<u8> = match input_format {
            "Hex" => {
                let s = String::from_utf8_lossy(&input);
                let hex_clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
                hex::decode(&hex_clean)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
            }
            _ => input, // Raw
        };

        if bytes.len() < 20 {
            return Err(OperationError::InvalidInput(
                "IPv4 header must be at least 20 bytes".to_string(),
            ));
        }

        let version = (bytes[0] >> 4) & 0x0f;
        let ihl = (bytes[0] & 0x0f) as usize;
        let dscp = (bytes[1] >> 2) & 0x3f;
        let ecn = bytes[1] & 0x03;
        let total_length = ((bytes[2] as u16) << 8) | (bytes[3] as u16);
        let identification = ((bytes[4] as u16) << 8) | (bytes[5] as u16);
        let flags = (bytes[6] >> 5) & 0x07;
        let frag_offset = (((bytes[6] & 0x1f) as u16) << 8) | (bytes[7] as u16);
        let ttl = bytes[8];
        let protocol = bytes[9];
        let checksum = ((bytes[10] as u16) << 8) | (bytes[11] as u16);
        let src_ip = Ipv4Addr::new(bytes[12], bytes[13], bytes[14], bytes[15]);
        let dst_ip = Ipv4Addr::new(bytes[16], bytes[17], bytes[18], bytes[19]);

        // Checksum verification: zero out checksum field and recompute
        let mut header_for_checksum = bytes[..20].to_vec();
        header_for_checksum[10] = 0;
        header_for_checksum[11] = 0;
        let computed_checksum = internet_checksum(&header_for_checksum);
        let checksum_ok = computed_checksum == checksum;

        let ihl_bytes = ihl * 4;
        let data_start = ihl_bytes.min(bytes.len());
        let payload = &bytes[data_start..];

        match output_format {
            "Data (hex)" => Ok(hex::encode(payload).into_bytes()),
            "Data (raw)" => Ok(payload.to_vec()),
            _ => {
                // Table output
                let version_note = if version != 4 {
                    format!("{} (Error: should be 4)", version)
                } else {
                    version.to_string()
                };

                let ihl_note = if ihl < 5 {
                    format!("{} (Error: should be >= 5)", ihl)
                } else {
                    format!("{} ({} bytes)", ihl, ihl_bytes)
                };

                let checksum_note = if checksum_ok {
                    format!("0x{:04x} (correct)", checksum)
                } else {
                    format!(
                        "0x{:04x} (incorrect, should be 0x{:04x})",
                        checksum, computed_checksum
                    )
                };

                let options_hex = if ihl > 5 && bytes.len() >= ihl_bytes {
                    let opts = &bytes[20..ihl_bytes];
                    format!("\nOptions: {}", hex::encode(opts))
                } else {
                    String::new()
                };

                let out = format!(
                    "Field                                          Value\n\
                     -----------------------------------------------\n\
                     Version                                        {}\n\
                     Internet Header Length (IHL)                   {}\n\
                     Differentiated Services Code Point (DSCP)      {}\n\
                     Explicit Congestion Notification (ECN)         {}\n\
                     Total length                                   {} bytes (IP header: {} bytes, Data: {} bytes)\n\
                     Identification                                 0x{:04x} ({})\n\
                     Flags                                          0x{:02x}\n\
                       Reserved bit:                                {}\n\
                       Don't fragment:                              {}\n\
                       More fragments:                              {}\n\
                     Fragment offset                                {}\n\
                     Time-To-Live                                   {}\n\
                     Protocol                                       {} ({})\n\
                     Header checksum                                {}\n\
                     Source IP address                              {}\n\
                     Destination IP address                         {}\n\
                     Data (hex)                                     {}{}",
                    version_note,
                    ihl_note,
                    dscp,
                    ecn,
                    total_length,
                    ihl_bytes,
                    total_length.saturating_sub(ihl_bytes as u16),
                    identification,
                    identification,
                    flags,
                    (flags >> 2) & 1,
                    (flags >> 1) & 1,
                    flags & 1,
                    frag_offset,
                    ttl,
                    protocol,
                    protocol_name(protocol),
                    checksum_note,
                    src_ip,
                    dst_ip,
                    hex::encode(payload),
                    options_hex,
                );
                Ok(out.into_bytes())
            }
        }
    }
}
