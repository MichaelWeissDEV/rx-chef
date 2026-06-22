/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the HASSH Client Fingerprint operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use byteorder::{BigEndian, ReadBytesExt};
use md5::{Digest, Md5};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// HASSH Client Fingerprint operation
pub struct HASSHClientFingerprint;

impl Operation for HASSHClientFingerprint {
    fn name(&self) -> &'static str {
        "HASSH Client Fingerprint"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates a HASSH fingerprint to help identify SSH clients based on hashing together values from the Client Key Exchange Init message.<br><br>Input: A hex stream of the SSH_MSG_KEXINIT packet application layer from Client to Server."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Input format",
                description: "Input format",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output format",
                description: "Output format",
                default_value: "Hash digest",
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
        let input_format = args.first().and_then(|v| v.as_str()).unwrap_or("Hex");
        let output_format = args
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("Hash digest");

        let data = match input_format {
            "Hex" => {
                let input_str = String::from_utf8(input)
                    .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
                let clean_input = input_str.replace(|c: char| !c.is_ascii_hexdigit(), "");
                hex::decode(clean_input)
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid hex: {}", e)))?
            }
            "Base64" => {
                let input_str = String::from_utf8(input)
                    .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;
                general_purpose::STANDARD
                    .decode(input_str.trim())
                    .map_err(|e| OperationError::InvalidInput(format!("Invalid base64: {}", e)))?
            }
            _ => input,
        };

        if data.is_empty() {
            return Ok(Vec::new());
        }

        let mut cursor = Cursor::new(&data);

        // Length (4 bytes)
        let packet_length = cursor.read_u32::<BigEndian>().map_err(|_| {
            OperationError::InvalidInput("Could not read packet length.".to_string())
        })?;
        if data.len() < (packet_length + 4) as usize {
            return Err(OperationError::InvalidInput(
                "Incorrect packet length.".to_string(),
            ));
        }

        // Padding length (1 byte)
        let _padding_length = cursor.read_u8().map_err(|_| {
            OperationError::InvalidInput("Could not read padding length.".to_string())
        })?;

        // Message code (1 byte)
        let message_code = cursor.read_u8().map_err(|_| {
            OperationError::InvalidInput("Could not read message code.".to_string())
        })?;
        if message_code != 20 {
            return Err(OperationError::InvalidInput(
                "Not a Key Exchange Init.".to_string(),
            ));
        }

        // Cookie (16 bytes)
        if cursor.position() + 16 > data.len() as u64 {
            return Err(OperationError::InvalidInput(
                "Cookie exceeds data size.".to_string(),
            ));
        }
        cursor.set_position(cursor.position() + 16);

        // KEX Algorithms
        let kex_algos = read_ssh_string(&mut cursor)?;
        // Server Host Key Algorithms
        let _server_host_key_algos = read_ssh_string(&mut cursor)?;
        // Encryption Algorithms Client to Server
        let enc_algos_c2s = read_ssh_string(&mut cursor)?;
        // Encryption Algorithms Server to Client
        let _enc_algos_s2c = read_ssh_string(&mut cursor)?;
        // MAC Algorithms Client to Server
        let mac_algos_c2s = read_ssh_string(&mut cursor)?;
        // MAC Algorithms Server to Client
        let _mac_algos_s2c = read_ssh_string(&mut cursor)?;
        // Compression Algorithms Client to Server
        let comp_algos_c2s = read_ssh_string(&mut cursor)?;
        // Compression Algorithms Server to Client
        let _comp_algos_s2c = read_ssh_string(&mut cursor)?;

        let hassh_str = format!(
            "{};{};{};{}",
            kex_algos, enc_algos_c2s, mac_algos_c2s, comp_algos_c2s
        );
        let mut hasher = Md5::new();
        hasher.update(hassh_str.as_bytes());
        let hassh_hash = hex::encode(hasher.finalize());

        match output_format {
            "HASSH algorithms string" => Ok(hassh_str.into_bytes()),
            "Full details" => {
                let details = format!(
                    "Hash digest:\n{}\n\nFull HASSH algorithms string:\n{}\n\nKey Exchange Algorithms:\n{}\nEncryption Algorithms Client to Server:\n{}\nMAC Algorithms Client to Server:\n{}\nCompression Algorithms Client to Server:\n{}",
                    hassh_hash, hassh_str, kex_algos, enc_algos_c2s, mac_algos_c2s, comp_algos_c2s
                );
                Ok(details.into_bytes())
            }
            _ => Ok(hassh_hash.into_bytes()),
        }
    }
}

fn read_ssh_string(cursor: &mut Cursor<&Vec<u8>>) -> Result<String, OperationError> {
    let length = cursor
        .read_u32::<BigEndian>()
        .map_err(|_| OperationError::InvalidInput("Could not read string length.".to_string()))?
        as usize;
    let pos = cursor.position() as usize;
    let data = cursor.get_ref();
    if pos + length > data.len() {
        return Err(OperationError::InvalidInput(
            "String length exceeds data size.".to_string(),
        ));
    }
    let s = String::from_utf8_lossy(&data[pos..pos + length]).into_owned();
    cursor.set_position((pos + length) as u64);
    Ok(s)
}
