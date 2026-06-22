/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse Ethernet frame operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse Ethernet Frame operation
///
/// Parses an Ethernet frame and displays source/destination MAC addresses,
/// optional VLAN tags, and the inner payload.
pub struct ParseEthernetFrame;

impl Operation for ParseEthernetFrame {
    fn name(&self) -> &'static str {
        "Parse Ethernet frame"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses an Ethernet frame and shows the deduced values: Source and Destination MAC, optional VLAN IDs, and the inner packet payload."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input type",
                description: "Raw bytes or Hex string",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Return type",
                description: "Text output, Packet data, or Packet data (hex)",
                default_value: "Text output",
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
        let input_type = args.first().and_then(|a| a.as_str()).unwrap_or("Hex");

        let return_type = args
            .get(1)
            .and_then(|a| a.as_str())
            .unwrap_or("Text output");

        let bytes: Vec<u8> = match input_type {
            "Hex" => {
                let s = String::from_utf8_lossy(&input);
                let hex_clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
                hex::decode(&hex_clean)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
            }
            _ => input, // Raw
        };

        if bytes.len() < 14 {
            return Err(OperationError::InvalidInput(
                "Ethernet frame must be at least 14 bytes".to_string(),
            ));
        }

        let dst_mac = &bytes[0..6];
        let src_mac = &bytes[6..12];

        let mut offset = 12usize;
        let mut vlans: Vec<u16> = Vec::new();

        // Parse VLAN tags (0x8100 = 802.1Q, 0x88A8 = 802.1ad QinQ)
        loop {
            if offset + 2 > bytes.len() {
                break;
            }
            let eth_type = ((bytes[offset] as u16) << 8) | (bytes[offset + 1] as u16);
            offset += 2;
            if eth_type == 0x8100 || eth_type == 0x88A8 {
                if offset + 2 > bytes.len() {
                    break;
                }
                let vlan_id = (((bytes[offset] & 0x0f) as u16) << 8) | (bytes[offset + 1] as u16);
                vlans.push(vlan_id);
                offset += 2;
            } else {
                break;
            }
        }

        let packet_data = &bytes[offset..];

        match return_type {
            "Packet data" => Ok(packet_data.to_vec()),
            "Packet data (hex)" => Ok(hex::encode(packet_data).into_bytes()),
            _ => {
                // Text output
                let src_str = mac_to_str(src_mac);
                let dst_str = mac_to_str(dst_mac);
                let mut out = format!("Source MAC: {}\nDestination MAC: {}\n", src_str, dst_str);
                if !vlans.is_empty() {
                    let vlan_strs: Vec<String> = vlans.iter().map(|v| v.to_string()).collect();
                    out.push_str(&format!("VLAN: {}\n", vlan_strs.join(", ")));
                }
                out.push_str(&format!("Data:\n{}", hex::encode(packet_data)));
                Ok(out.into_bytes())
            }
        }
    }
}

fn mac_to_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(":")
}
