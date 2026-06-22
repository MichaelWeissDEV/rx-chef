/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse TCP operation.
 * -----------------------------------------------------------------------------
 */

use hex;
use serde_json::{json, Value};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse TCP operation
pub struct ParseTcp;

impl Operation for ParseTcp {
    fn name(&self) -> &'static str {
        "Parse TCP"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Parses a TCP header and payload (if present)."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Input format",
            description: "The format of the input data",
            default_value: "Hex",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let format = args.first().and_then(|a| a.as_str()).unwrap_or("Hex");

        let data = if format == "Hex" {
            let input_str = String::from_utf8_lossy(&input).trim().to_string();
            let cleaned = input_str
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .collect::<String>();
            hex::decode(cleaned).map_err(|e| OperationError::InvalidArgument {
                name: "Input format".to_string(),
                reason: format!("Invalid hex: {}", e),
            })?
        } else {
            input
        };

        if data.len() < 20 {
            return Err(OperationError::ProcessingError(
                "Need at least 20 bytes for a TCP Header".to_string(),
            ));
        }

        let mut offset = 0;
        let src_port = read_u16(&data, &mut offset);
        let dst_port = read_u16(&data, &mut offset);
        let seq_num = read_u32(&data, &mut offset);
        let ack_num = read_u32(&data, &mut offset);

        let b12 = data[offset];
        offset += 1;
        let data_offset = (b12 >> 4) as u8;
        let res = (b12 & 0x0F) >> 1; // 3 bits reserved
        let ns = (b12 & 0x01) != 0;

        let b13 = data[offset];
        offset += 1;
        let cwr = (b13 & 0x80) != 0;
        let ece = (b13 & 0x40) != 0;
        let urg = (b13 & 0x20) != 0;
        let ack = (b13 & 0x10) != 0;
        let psh = (b13 & 0x08) != 0;
        let rst = (b13 & 0x04) != 0;
        let syn = (b13 & 0x02) != 0;
        let fin = (b13 & 0x01) != 0;

        let window_size = read_u16(&data, &mut offset);
        let checksum = read_u16(&data, &mut offset);
        let urgent_pointer = read_u16(&data, &mut offset);

        let mut tcp_packet = json!({
            "Source port": src_port,
            "Destination port": dst_port,
            "Sequence number": seq_num,
            "Acknowledgement number": ack_num,
            "Data offset": format!("{} ({} bytes)", data_offset, data_offset * 4),
            "Flags": {
                "Reserved": format!("{:03b}", res),
                "NS": ns as u8,
                "CWR": cwr as u8,
                "ECE": ece as u8,
                "URG": urg as u8,
                "ACK": ack as u8,
                "PSH": psh as u8,
                "RST": rst as u8,
                "SYN": syn as u8,
                "FIN": fin as u8,
            },
            "Window size": window_size,
            "Checksum": format!("0x{:04x}", checksum),
            "Urgent pointer": format!("0x{:04x}", urgent_pointer)
        });

        let mut window_scale_shift = 0;
        if data_offset > 5 {
            let mut remaining_length = (data_offset as usize * 4) - 20;
            let mut options = serde_json::Map::new();

            while remaining_length > 0 && offset < data.len() {
                let kind = data[offset];
                offset += 1;
                remaining_length -= 1;

                if kind == 0 {
                    // End of options
                    options.insert("End of Option List".to_string(), json!({"Kind": 0}));
                    break;
                }
                if kind == 1 {
                    // No-op
                    options.insert("No-Operation".to_string(), json!({"Kind": 1}));
                    continue;
                }

                if remaining_length == 0 {
                    break;
                }
                let length = data[offset];
                offset += 1;
                remaining_length -= 1;

                let val_len = if length >= 2 {
                    (length - 2) as usize
                } else {
                    0
                };
                let mut val = Vec::new();
                for _ in 0..val_len {
                    if offset < data.len() {
                        val.push(data[offset]);
                        offset += 1;
                        if remaining_length > 0 {
                            remaining_length -= 1;
                        }
                    }
                }

                let (name, value) = match kind {
                    2 => {
                        if val.len() >= 2 {
                            (
                                "Maximum Segment Size",
                                json!(u16::from_be_bytes([val[0], val[1]])),
                            )
                        } else {
                            ("Maximum Segment Size", json!(null))
                        }
                    }
                    3 => {
                        if !val.is_empty() {
                            window_scale_shift = val[0];
                            (
                                "Window Scale",
                                json!({
                                    "Shift count": val[0],
                                    "Multiplier": 1 << val[0]
                                }),
                            )
                        } else {
                            ("Window Scale", json!(null))
                        }
                    }
                    4 => ("SACK Permitted", json!(null)),
                    8 => {
                        if val.len() == 8 {
                            let tsval = u32::from_be_bytes([val[0], val[1], val[2], val[3]]);
                            let tsecr = u32::from_be_bytes([val[4], val[5], val[6], val[7]]);
                            (
                                "Timestamps",
                                json!({
                                    "Current Timestamp": tsval,
                                    "Echo Reply": tsecr
                                }),
                            )
                        } else {
                            ("Timestamps", json!(format!("0x{}", hex::encode(&val))))
                        }
                    }
                    _ => ("Reserved", json!(format!("0x{}", hex::encode(&val)))),
                };

                options.insert(
                    name.to_string(),
                    json!({
                        "Kind": kind,
                        "Length": length,
                        "Value": value
                    }),
                );
            }
            tcp_packet
                .as_object_mut()
                .unwrap()
                .insert("Options".to_string(), Value::Object(options));
        }

        if offset < data.len() {
            tcp_packet.as_object_mut().unwrap().insert(
                "Data".to_string(),
                json!(format!("0x{}", hex::encode(&data[offset..]))),
            );
        }

        let true_window_size = (window_size as u64) << window_scale_shift;
        tcp_packet.as_object_mut().unwrap().insert(
            "Window size".to_string(),
            json!(format!("{} (Scaled: {})", window_size, true_window_size)),
        );

        Ok(serde_json::to_string_pretty(&tcp_packet)
            .unwrap()
            .into_bytes())
    }
}

fn read_u16(data: &[u8], offset: &mut usize) -> u16 {
    let val = u16::from_be_bytes([data[*offset], data[*offset + 1]]);
    *offset += 2;
    val
}

fn read_u32(data: &[u8], offset: &mut usize) -> u32 {
    let val = u32::from_be_bytes([
        data[*offset],
        data[*offset + 1],
        data[*offset + 2],
        data[*offset + 3],
    ]);
    *offset += 4;
    val
}
