/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Format MAC addresses operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Format MAC addresses operation
pub struct FormatMACAddresses;

impl Operation for FormatMACAddresses {
    fn name(&self) -> &'static str {
        "Format MAC addresses"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Displays given MAC addresses in multiple different formats. Expects addresses separated by newlines, spaces, or commas."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Output case",
                description: "Both, Upper only, or Lower only",
                default_value: "Both",
            },
            ArgSchema {
                name: "No delimiter",
                description: "Output format with no delimiter (e.g. AABBCCDDEEFF)",
                default_value: "true",
            },
            ArgSchema {
                name: "Dash delimiter",
                description: "Output format with dashes (e.g. AA-BB-CC-DD-EE-FF)",
                default_value: "true",
            },
            ArgSchema {
                name: "Colon delimiter",
                description: "Output format with colons (e.g. AA:BB:CC:DD:EE:FF)",
                default_value: "true",
            },
            ArgSchema {
                name: "Cisco style",
                description: "Output Cisco dot notation (e.g. AABB.CCDD.EEFF)",
                default_value: "false",
            },
            ArgSchema {
                name: "IPv6 interface ID",
                description: "Output as IPv6 interface identifier",
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
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8 input".to_string()))?;

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let output_case = args.first().and_then(|v| v.as_str()).unwrap_or("Both");
        let no_delim = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let dash_delim = args.get(2).and_then(|v| v.as_bool()).unwrap_or(true);
        let colon_delim = args.get(3).and_then(|v| v.as_bool()).unwrap_or(true);
        let cisco_style = args.get(4).and_then(|v| v.as_bool()).unwrap_or(false);
        let ipv6_int_id = args.get(5).and_then(|v| v.as_bool()).unwrap_or(false);

        let sep_re = Regex::new(r"[,\s\r\n]+")
            .map_err(|e| OperationError::ProcessingError(format!("Regex error: {}", e)))?;

        let macs: Vec<&str> = sep_re.split(input_str.trim()).collect();
        let mut output_list: Vec<String> = Vec::new();

        for mac in &macs {
            if mac.is_empty() {
                continue;
            }
            let lower = mac.to_lowercase();
            // Remove all existing delimiters
            let clean: String = lower.chars().filter(|c| c.is_ascii_hexdigit()).collect();

            if clean.len() != 12 {
                continue;
            }

            // Insert separator every 2 chars for hyphen/colon
            let mac_hyphen = insert_sep(&clean, '-', 2);
            let mac_colon = insert_sep(&clean, ':', 2);
            // Cisco style: every 4 chars separated by dot
            let mac_cisco = insert_sep(&clean, '.', 4);

            // IPv6 interface ID: insert fffe in middle then XOR bit 6
            let mac_ipv6 =
                build_ipv6_iid(&clean).map_err(|e| OperationError::ProcessingError(e))?;

            match output_case {
                "Lower only" => {
                    if no_delim {
                        output_list.push(clean.clone());
                    }
                    if dash_delim {
                        output_list.push(mac_hyphen);
                    }
                    if colon_delim {
                        output_list.push(mac_colon);
                    }
                    if cisco_style {
                        output_list.push(mac_cisco);
                    }
                    if ipv6_int_id {
                        output_list.push(mac_ipv6);
                    }
                }
                "Upper only" => {
                    if no_delim {
                        output_list.push(clean.to_uppercase());
                    }
                    if dash_delim {
                        output_list.push(mac_hyphen.to_uppercase());
                    }
                    if colon_delim {
                        output_list.push(mac_colon.to_uppercase());
                    }
                    if cisco_style {
                        output_list.push(mac_cisco.to_uppercase());
                    }
                    if ipv6_int_id {
                        output_list.push(mac_ipv6.to_uppercase());
                    }
                }
                _ => {
                    // Both
                    if no_delim {
                        output_list.push(clean.clone());
                        output_list.push(clean.to_uppercase());
                    }
                    if dash_delim {
                        output_list.push(mac_hyphen.clone());
                        output_list.push(mac_hyphen.to_uppercase());
                    }
                    if colon_delim {
                        output_list.push(mac_colon.clone());
                        output_list.push(mac_colon.to_uppercase());
                    }
                    if cisco_style {
                        output_list.push(mac_cisco.clone());
                        output_list.push(mac_cisco.to_uppercase());
                    }
                    if ipv6_int_id {
                        output_list.push(mac_ipv6.clone());
                        output_list.push(mac_ipv6.to_uppercase());
                    }
                }
            }

            // Empty line between groups
            output_list.push(String::new());
        }

        Ok(output_list.join("\n").into_bytes())
    }
}

/// Insert a separator character every `every` hex digits.
fn insert_sep(clean: &str, sep: char, every: usize) -> String {
    let chars: Vec<char> = clean.chars().collect();
    let mut result = String::new();
    let mut count = 0;
    for (i, c) in chars.iter().enumerate() {
        result.push(*c);
        count += 1;
        if count == every && i + 1 < chars.len() {
            result.push(sep);
            count = 0;
        }
    }
    result
}

/// Build an IPv6 interface identifier from a 12-char hex MAC address.
fn build_ipv6_iid(clean: &str) -> Result<String, String> {
    // Insert fffe in the middle: first 6 + fffe + last 6
    let expanded = format!("{}fffe{}", &clean[..6], &clean[6..]);
    // XOR bit 6 (universal/local bit) of the first byte
    let first_byte =
        u8::from_str_radix(&expanded[..2], 16).map_err(|e| format!("Hex parse error: {}", e))?;
    let flipped = first_byte ^ 0x02;
    let iid_str = format!("{:02x}{}", flipped, &expanded[2..]);
    // Format as groups of 4 hex digits separated by colons
    Ok(insert_sep(&iid_str, ':', 4))
}
