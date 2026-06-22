/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the IPv6 Transition Addresses operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// IPv6 Transition Addresses operation
pub struct IPv6TransitionAddresses;

impl Operation for IPv6TransitionAddresses {
    fn name(&self) -> &'static str {
        "IPv6 Transition Addresses"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Converts IPv4 addresses to their IPv6 Transition addresses. IPv6 Transition addresses can also be converted back into their original IPv4 address. MAC addresses can also be converted into the EUI-64 format, this can them be appended to your IPv6 /64 range to obtain a full /128 address.<br><br>Transition technologies enable translation between IPv4 and IPv6 addresses or tunneling to allow traffic to pass through the incompatible network, allowing the two standards to coexist.<br><br>Only /24 ranges and currently handled. Remove headers to easily copy out results."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Ignore ranges",
                description: "If checked, input ranges will be ignored.",
                default_value: "true",
            },
            ArgSchema {
                name: "Remove headers",
                description: "Remove headers to easily copy out results.",
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
        let input_str = String::from_utf8_lossy(&input);
        let ignore_ranges = args.first().and_then(|v| v.as_bool()).unwrap_or(true);
        let remove_headers = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);

        let mut output = String::new();
        let inputs: Vec<&str> = input_str.lines().filter(|s| !s.is_empty()).collect();

        let re_ipv4 = Regex::new(r"^[0-9]{1,3}(?:\.[0-9]{1,3}){3}$").unwrap();
        let re_range24 = Regex::new(r"/24$").unwrap();
        let re_mac = Regex::new(r"^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$").unwrap();
        let re_ipv6 = Regex::new(r"^((?:[0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,7}:|(?:[0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,5}(?::[0-9a-fA-F]{1,4}){1,2}|(?:[0-9a-fA-F]{1,4}:){1,4}(?::[0-9a-fA-F]{1,4}){1,3}|(?:[0-9a-fA-F]{1,4}:){1,3}(?::[0-9a-fA-F]{1,4}){1,4}|(?:[0-9a-fA-F]{1,4}:){1,2}(?::[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:(?:(?::[0-9a-fA-F]{1,4}){1,6})|:(?:(?::[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(?::[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(?:ffff(?::0{1,4}){0,1}:){0,1}(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])|(?:[0-9a-fA-F]{1,4}:){1,4}:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$").unwrap();

        for item in inputs {
            let is_range = item.contains('/');
            if (ignore_ranges && !is_range) || (!ignore_ranges) {
                if re_ipv4.is_match(item) {
                    output.push_str(&ip_transition(item, false, remove_headers));
                } else if re_range24.is_match(item) {
                    output.push_str(&ip_transition(item, true, remove_headers));
                } else if re_mac.is_match(item) {
                    output.push_str(&mac_transition(item, remove_headers));
                } else if re_ipv6.is_match(item) {
                    output.push_str(&un_transition(item, remove_headers));
                } else {
                    return Ok(
                        "Enter compressed or expanded IPv6 address, IPv4 address or MAC Address."
                            .as_bytes()
                            .to_vec(),
                    );
                }
            }
        }

        Ok(output.into_bytes())
    }
}

fn hexify(octet: &str) -> String {
    if let Ok(num) = octet.parse::<u8>() {
        format!("{:02x}", num)
    } else {
        "00".to_string()
    }
}

fn intify(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).unwrap_or(0)
}

fn ip_transition(input: &str, range: bool, remove_headers: bool) -> String {
    let mut output = String::new();
    let parts: Vec<&str> = if range {
        input.trim_end_matches("/24").split('.').collect()
    } else {
        input.split('.').collect()
    };

    if parts.len() < 4 {
        return "".to_string();
    }

    let h0 = hexify(parts[0]);
    let h1 = hexify(parts[1]);
    let h2 = hexify(parts[2]);
    let h3 = hexify(parts[3]);

    // 6to4
    if !remove_headers {
        output.push_str("6to4: ");
    }
    output.push_str(&format!("2002:{}{}:{}", h0, h1, h2));
    if range {
        output.push_str("00::/40\n");
    } else {
        output.push_str(&format!("{}::/48\n", h3));
    }

    // Mapped
    if !remove_headers {
        output.push_str("IPv4 Mapped: ");
    }
    output.push_str(&format!("::ffff:{}{}:{}", h0, h1, h2));
    if range {
        output.push_str("00/120\n");
    } else {
        output.push_str(&format!("{}\n", h3));
    }

    // Translated
    if !remove_headers {
        output.push_str("IPv4 Translated: ");
    }
    output.push_str(&format!("::ffff:0:{}{}:{}", h0, h1, h2));
    if range {
        output.push_str("00/120\n");
    } else {
        output.push_str(&format!("{}\n", h3));
    }

    // Nat64
    if !remove_headers {
        output.push_str("Nat 64: ");
    }
    output.push_str(&format!("64:ff9b::{}{}:{}", h0, h1, h2));
    if range {
        output.push_str("00/120\n");
    } else {
        output.push_str(&format!("{}\n", h3));
    }

    output
}

fn mac_transition(input: &str, remove_headers: bool) -> String {
    let mut output = String::new();
    let mac_parts: Vec<&str> = input.split(':').collect();
    if mac_parts.len() != 6 {
        return "".to_string();
    }

    if !remove_headers {
        output.push_str("EUI-64 Interface ID: ");
    }

    let _mac_str = format!(
        "{}{}:{}{}ff:fe{}:{}{}",
        mac_parts[0],
        mac_parts[1],
        mac_parts[2],
        "", // dummy
        mac_parts[3],
        mac_parts[4],
        mac_parts[5]
    );
    // Wait, CyberChef does: const MAC = MACPARTS[0] + MACPARTS[1] + ":" + MACPARTS[2] + "ff:fe" + MACPARTS[3] + ":" + MACPARTS[4] + MACPARTS[5];
    // MACPARTS[0] is first byte.
    // So MAC starts with 4 hex chars, then ':', then 4 hex chars, then ':', then 4 hex chars.
    // e.g. aabb:ccff:fedd:eeff
    let mac_val = format!(
        "{}{}:{}{}ff:fe{}:{}{}",
        mac_parts[0], mac_parts[1], mac_parts[2], "", mac_parts[3], mac_parts[4], mac_parts[5]
    );

    // XOR table
    fn xor_char(c: char) -> char {
        match c {
            '0' => '2',
            '1' => '3',
            '2' => '0',
            '3' => '1',
            '4' => '6',
            '5' => '7',
            '6' => '4',
            '7' => '5',
            '8' => 'a',
            '9' => 'b',
            'a' => '8',
            'b' => '9',
            'c' => 'e',
            'd' => 'f',
            'e' => 'c',
            'f' => 'd',
            'A' => '8',
            'B' => '9',
            'C' => 'e',
            'D' => 'f',
            'E' => 'c',
            'F' => 'd', // handle uppercase
            _ => c,
        }
    }

    if mac_val.len() > 1 {
        let mut chars: Vec<char> = mac_val.chars().collect();
        chars[1] = xor_char(chars[1]);
        output.push_str(&chars.into_iter().collect::<String>());
    }

    output
}

fn un_transition(input: &str, remove_headers: bool) -> String {
    let mut output = String::new();
    let input_lower = input.to_lowercase();

    if input_lower.starts_with("2002:") {
        if !remove_headers {
            output.push_str("IPv4: ");
        }
        if input.len() >= 14 {
            let i1 = intify(&input[5..7]);
            let i2 = intify(&input[7..9]);
            let i3 = intify(&input[10..12]);
            let i4 = intify(&input[12..14]);
            output.push_str(&format!("{}.{}.{}.{}\n", i1, i2, i3, i4));
        }
    } else if input_lower.starts_with("::ffff:")
        || input_lower.starts_with("0000:0000:0000:0000:0000:ffff:")
        || input_lower.starts_with("::ffff:0000:")
        || input_lower.starts_with("0000:0000:0000:0000:ffff:0000:")
        || input_lower.starts_with("64:ff9b::")
        || input_lower.starts_with("0064:ff9b:0000:0000:0000:0000:")
    {
        let re1 = Regex::new(r":([0-9a-z]{1,4}):[0-9a-z]{1,4}$").unwrap();
        let re2 = Regex::new(r":([0-9a-z]{1,4})$").unwrap();

        if let (Some(m1), Some(m2)) = (re1.captures(&input_lower), re2.captures(&input_lower)) {
            let h1 = format!("{:0>4}", m1.get(1).unwrap().as_str());
            let h2 = format!("{:0>4}", m2.get(1).unwrap().as_str());
            let hextets = format!("{}{}", h1, h2);

            if hextets.len() >= 8 {
                if !remove_headers {
                    output.push_str("IPv4: ");
                }
                let i1 = intify(&hextets[0..2]);
                let i2 = intify(&hextets[2..4]);
                let i3 = intify(&hextets[4..6]);
                let i4 = intify(&hextets[6..8]);
                output.push_str(&format!("{}.{}.{}.{}\n", i1, i2, i3, i4));
            }
        }
    } else if input.len() >= 12
        && input[input.len() - 12..input.len() - 7].to_uppercase() == "FF:FE"
    {
        if !remove_headers {
            output.push_str("Mac Address: ");
        }
        // EUI-64
        // MAC = (input.slice(-19, -17) + ":" + input.slice(-17, -15) + ":" + input.slice(-14, -12) + ":" + input.slice(-7, -5) + ":" + input.slice(-4, -2) + ":" + input.slice(-2,)).toUpperCase();
        if input.len() >= 19 {
            let part1 = &input[input.len() - 19..input.len() - 17];
            let part2 = &input[input.len() - 17..input.len() - 15];
            let part3 = &input[input.len() - 14..input.len() - 12];
            let part4 = &input[input.len() - 7..input.len() - 5];
            let part5 = &input[input.len() - 4..input.len() - 2];
            let part6 = &input[input.len() - 2..];

            let mac = format!(
                "{}:{}:{}:{}:{}:{}",
                part1, part2, part3, part4, part5, part6
            )
            .to_uppercase();

            fn xor_char(c: char) -> char {
                match c {
                    '0' => '2',
                    '1' => '3',
                    '2' => '0',
                    '3' => '1',
                    '4' => '6',
                    '5' => '7',
                    '6' => '4',
                    '7' => '5',
                    '8' => 'A',
                    '9' => 'B',
                    'A' => '8',
                    'B' => '9',
                    'C' => 'E',
                    'D' => 'F',
                    'E' => 'C',
                    'F' => 'D',
                    _ => c,
                }
            }

            let mut chars: Vec<char> = mac.chars().collect();
            chars[1] = xor_char(chars[1]);
            output.push_str(&chars.into_iter().collect::<String>());
            output.push('\n');
        }
    }

    output
}
