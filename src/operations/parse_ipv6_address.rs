/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse IPv6 address operation.
 * -----------------------------------------------------------------------------
 */

use std::net::Ipv6Addr;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse IPv6 Address operation
///
/// Displays the longhand and shorthand versions of a valid IPv6 address and
/// identifies reserved ranges such as loopback, link-local, multicast,
/// Teredo, 6to4, and unique local addresses.
pub struct ParseIPv6Address;

fn ipv6_to_groups(addr: Ipv6Addr) -> [u16; 8] {
    let o = addr.octets();
    [
        ((o[0] as u16) << 8) | o[1] as u16,
        ((o[2] as u16) << 8) | o[3] as u16,
        ((o[4] as u16) << 8) | o[5] as u16,
        ((o[6] as u16) << 8) | o[7] as u16,
        ((o[8] as u16) << 8) | o[9] as u16,
        ((o[10] as u16) << 8) | o[11] as u16,
        ((o[12] as u16) << 8) | o[13] as u16,
        ((o[14] as u16) << 8) | o[15] as u16,
    ]
}

fn ipv6_longhand(addr: Ipv6Addr) -> String {
    let g = ipv6_to_groups(addr);
    format!(
        "{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}",
        g[0], g[1], g[2], g[3], g[4], g[5], g[6], g[7]
    )
}

fn u32_to_ipv4_str(val: u32) -> String {
    std::net::Ipv4Addr::from(val).to_string()
}

impl Operation for ParseIPv6Address {
    fn name(&self) -> &'static str {
        "Parse IPv6 address"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Displays the longhand and shorthand versions of a valid IPv6 address, identifies reserved ranges including loopback, link-local, multicast, Teredo, 6to4, and unique local addresses, and detects EUI-64 interface identifiers."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let text = String::from_utf8_lossy(&input);
        let text = text.trim();

        let addr: Ipv6Addr = text
            .parse()
            .map_err(|_| OperationError::InvalidInput("Invalid IPv6 address".to_string()))?;

        let g = ipv6_to_groups(addr);
        let longhand = ipv6_longhand(addr);
        let shorthand = addr.to_string(); // Rust std library produces canonical short form

        let mut out = format!("Longhand:  {}\nShorthand: {}\n", longhand, shorthand);

        // Reserved range detection
        if addr == Ipv6Addr::UNSPECIFIED {
            out.push_str("\nUnspecified address corresponding to 0.0.0.0/32 in IPv4.");
            out.push_str("\nUnspecified address range: ::/128");
        } else if addr == Ipv6Addr::LOCALHOST {
            out.push_str(
                "\nLoopback address to the local host corresponding to 127.0.0.1/8 in IPv4.",
            );
            out.push_str("\nLoopback addresses range: ::1/128");
        } else if g[0] == 0 && g[1] == 0 && g[2] == 0 && g[3] == 0 && g[4] == 0 && g[5] == 0xffff {
            // IPv4-mapped
            let ipv4_val = ((g[6] as u32) << 16) | g[7] as u32;
            out.push_str("\nIPv4-mapped IPv6 address detected.");
            out.push_str(&format!(
                "\nMapped IPv4 address: {}",
                u32_to_ipv4_str(ipv4_val)
            ));
            out.push_str("\nIPv4-mapped IPv6 addresses range: ::ffff:0:0/96");
        } else if g[0] == 0 && g[1] == 0 && g[2] == 0 && g[3] == 0 && g[4] == 0xffff && g[5] == 0 {
            // IPv4-translated
            let ipv4_val = ((g[6] as u32) << 16) | g[7] as u32;
            out.push_str("\nIPv4-translated address detected (SIIT). See RFCs 6145 and 6052.");
            out.push_str(&format!(
                "\nTranslated IPv4 address: {}",
                u32_to_ipv4_str(ipv4_val)
            ));
            out.push_str("\nIPv4-translated addresses range: ::ffff:0:0:0/96");
        } else if g[0] == 0x0100 {
            out.push_str("\nDiscard prefix (RFC 6666). Used for sinkhole routing.");
            out.push_str("\nDiscard range: 100::/64");
        } else if g[0] == 0x0064
            && g[1] == 0xff9b
            && g[2] == 0
            && g[3] == 0
            && g[4] == 0
            && g[5] == 0
        {
            // Well-known prefix 64:ff9b::/96
            let ipv4_val = ((g[6] as u32) << 16) | g[7] as u32;
            out.push_str("\n'Well-Known' prefix for IPv4/IPv6 translation (RFC 6052).");
            out.push_str(&format!(
                "\nTranslated IPv4 address: {}",
                u32_to_ipv4_str(ipv4_val)
            ));
            out.push_str("\n'Well-Known' prefix range: 64:ff9b::/96");
        } else if g[0] == 0x2001 && g[1] == 0x0000 {
            // Teredo
            let server_ipv4 = ((g[2] as u32) << 16) | g[3] as u32;
            let udp_port = (!g[5]) & 0xffff;
            let client_ipv4 = !(((g[6] as u32) << 16) | g[7] as u32);
            let flag_cone = (g[4] >> 15) & 1;
            let flag_r = (g[4] >> 14) & 1;
            let flag_ug = (g[4] >> 8) & 3;

            out.push_str("\nTeredo tunneling IPv6 address detected.");
            out.push_str(&format!(
                "\nServer IPv4 address: {}",
                u32_to_ipv4_str(server_ipv4)
            ));
            out.push_str(&format!(
                "\nClient IPv4 address: {}",
                u32_to_ipv4_str(client_ipv4)
            ));
            out.push_str(&format!("\nClient UDP port:     {}", udp_port));
            out.push_str(&format!(
                "\nFlags:\n\tCone: {} ({})\n\tR: {}\n\tUG: {}",
                flag_cone,
                if flag_cone == 1 {
                    "behind cone NAT"
                } else {
                    "not behind cone NAT"
                },
                flag_r,
                flag_ug,
            ));
            out.push_str("\nTeredo prefix range: 2001::/32");
        } else if g[0] == 0x2001 && g[1] == 0x0002 && g[2] == 0 {
            out.push_str("\nBMWG benchmarking range (RFC 5180): 2001:2::/48");
        } else if g[0] == 0x2001 && g[1] >= 0x0010 && g[1] <= 0x001f {
            out.push_str("\nDeprecated ORCHIDv1 range: 2001:10::/28. ORCHIDv2 uses 2001:20::/28.");
        } else if g[0] == 0x2001 && g[1] >= 0x0020 && g[1] <= 0x002f {
            out.push_str(
                "\nORCHIDv2 (Overlay Routable Cryptographic Hash Identifiers): 2001:20::/28",
            );
        } else if g[0] == 0x2001 && g[1] == 0x0db8 {
            out.push_str("\nDocumentation IPv6 address. Use in examples only.");
            out.push_str("\nDocumentation range: 2001:db8::/32");
        } else if g[0] == 0x2002 {
            // 6to4
            let v4_addr = ((g[1] as u32) << 16) | g[2] as u32;
            out.push_str("\n6to4 transition IPv6 address (RFC 3056).");
            out.push_str(&format!(
                "\nEncapsulated IPv4 address: {}",
                u32_to_ipv4_str(v4_addr)
            ));
            out.push_str("\n6to4 prefix range: 2002::/16");
        } else if g[0] >= 0xfc00 && g[0] <= 0xfdff {
            out.push_str("\nUnique local address (RFC 4193), comparable to IPv4 private ranges.");
            out.push_str("\nUnique local addresses range: fc00::/7");
        } else if g[0] >= 0xfe80 && g[0] <= 0xfebf {
            out.push_str("\nLink-local address comparable to 169.254.0.0/16 in IPv4.");
            out.push_str("\nLink-local addresses range: fe80::/10");
        } else if g[0] >= 0xff00 {
            out.push_str("\nReserved multicast address.");
            out.push_str("\nMulticast addresses range: ff00::/8");
        }

        // EUI-64 detection: FF:FE in octets 11-12 (groups 5 lower byte and 6 upper byte)
        if (g[5] & 0x00ff) == 0x00ff && (g[6] >> 8) == 0xfe {
            out.push_str(
                "\n\nThis IPv6 address contains a modified EUI-64 address (FF:FE in octets 11-12).",
            );
            let mac = format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                ((g[4] >> 8) ^ 2) as u8,
                (g[4] & 0xff) as u8,
                (g[5] >> 8) as u8,
                (g[6] & 0xff) as u8,
                (g[7] >> 8) as u8,
                (g[7] & 0xff) as u8,
            );
            out.push_str(&format!("\nMAC address: {}", mac));
        }

        Ok(out.into_bytes())
    }
}
