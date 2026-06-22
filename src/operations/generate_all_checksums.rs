/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate all checksums operation.
 * -----------------------------------------------------------------------------
 */

// // use crate::operations::crc_checksum::CRCChecksum;
use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::{
        adler32_checksum::Adler32Checksum, fletcher16_checksum::Fletcher16Checksum,
        fletcher32_checksum::Fletcher32Checksum, fletcher64_checksum::Fletcher64Checksum,
        fletcher8_checksum::Fletcher8Checksum,
    },
};

/// Generate all checksums operation
pub struct GenerateAllChecksums;

impl Operation for GenerateAllChecksums {
    fn name(&self) -> &'static str {
        "Generate all checksums"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates all available checksums for the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Length (bits)",
                description: "Length of the checksum in bits",
                default_value: "All",
            },
            ArgSchema {
                name: "Include names",
                description: "Include the name of the checksum algorithm in the output",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let length_arg = args.first().and_then(|v| v.as_str()).unwrap_or("All");
        let include_names = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let mut output = String::new();

        let checksums = get_checksums();

        for (name, algo, param) in checksums {
            let checksum_length = extract_length_from_name(name);
            if length_arg == "All" || length_arg == checksum_length {
                let algo_args = if param.is_empty() {
                    vec![]
                } else {
                    vec![ArgValue::Str(param.to_string())]
                };

                let val_bytes = algo.run(input.clone(), &algo_args)?;
                let val = String::from_utf8_lossy(&val_bytes);

                if include_names {
                    let padding = 25_usize.saturating_sub(name.len());
                    output.push_str(&format!(
                        "{}: {}{}\n",
                        name,
                        " ".repeat(padding.saturating_sub(1)),
                        val
                    ));
                } else {
                    output.push_str(&format!("{}\n", val));
                }
            }
        }

        Ok(output.into_bytes())
    }
}

fn extract_length_from_name(name: &str) -> String {
    // Basic extraction, e.g. "CRC-32/BZIP2" -> "32"
    if let Some(dash_idx) = name.find('-') {
        let rest = &name[dash_idx + 1..];
        if let Some(slash_idx) = rest.find('/') {
            rest[..slash_idx].to_string()
        } else {
            rest.to_string()
        }
    } else {
        // e.g. Fletcher-8 -> "8"
        if name.starts_with("Fletcher-") || name.starts_with("Adler-") {
            let parts: Vec<&str> = name.split('-').collect();
            if parts.len() == 2 {
                return parts[1].to_string();
            }
        }
        "".to_string()
    }
}

fn get_checksums() -> Vec<(&'static str, Box<dyn Operation>, &'static str)> {
    vec![
        //         ("CRC-3/GSM", Box::new(CRCChecksum), "CRC-3/GSM"),
        //         ("CRC-3/ROHC", Box::new(CRCChecksum), "CRC-3/ROHC"),
        //         ("CRC-4/G-704", Box::new(CRCChecksum), "CRC-4/G-704"),
        //         ("CRC-4/INTERLAKEN", Box::new(CRCChecksum), "CRC-4/INTERLAKEN"),
        //         ("CRC-4/ITU", Box::new(CRCChecksum), "CRC-4/ITU"),
        //         ("CRC-5/EPC", Box::new(CRCChecksum), "CRC-5/EPC"),
        //         ("CRC-5/EPC-C1G2", Box::new(CRCChecksum), "CRC-5/EPC-C1G2"),
        //         ("CRC-5/G-704", Box::new(CRCChecksum), "CRC-5/G-704"),
        //         ("CRC-5/ITU", Box::new(CRCChecksum), "CRC-5/ITU"),
        //         ("CRC-5/USB", Box::new(CRCChecksum), "CRC-5/USB"),
        //         ("CRC-6/CDMA2000-A", Box::new(CRCChecksum), "CRC-6/CDMA2000-A"),
        //         ("CRC-6/CDMA2000-B", Box::new(CRCChecksum), "CRC-6/CDMA2000-B"),
        //         ("CRC-6/DARC", Box::new(CRCChecksum), "CRC-6/DARC"),
        //         ("CRC-6/G-704", Box::new(CRCChecksum), "CRC-6/G-704"),
        //         ("CRC-6/GSM", Box::new(CRCChecksum), "CRC-6/GSM"),
        //         ("CRC-6/ITU", Box::new(CRCChecksum), "CRC-6/ITU"),
        //         ("CRC-7/MMC", Box::new(CRCChecksum), "CRC-7/MMC"),
        //         ("CRC-7/ROHC", Box::new(CRCChecksum), "CRC-7/ROHC"),
        //         ("CRC-7/UMTS", Box::new(CRCChecksum), "CRC-7/UMTS"),
        //         ("CRC-8", Box::new(CRCChecksum), "CRC-8"),
        //         ("CRC-8/8H2F", Box::new(CRCChecksum), "CRC-8/8H2F"),
        //         ("CRC-8/AES", Box::new(CRCChecksum), "CRC-8/AES"),
        //         ("CRC-8/AUTOSAR", Box::new(CRCChecksum), "CRC-8/AUTOSAR"),
        //         ("CRC-8/BLUETOOTH", Box::new(CRCChecksum), "CRC-8/BLUETOOTH"),
        //         ("CRC-8/CDMA2000", Box::new(CRCChecksum), "CRC-8/CDMA2000"),
        //         ("CRC-8/DARC", Box::new(CRCChecksum), "CRC-8/DARC"),
        //         ("CRC-8/DVB-S2", Box::new(CRCChecksum), "CRC-8/DVB-S2"),
        //         ("CRC-8/EBU", Box::new(CRCChecksum), "CRC-8/EBU"),
        //         ("CRC-8/GSM-A", Box::new(CRCChecksum), "CRC-8/GSM-A"),
        //         ("CRC-8/GSM-B", Box::new(CRCChecksum), "CRC-8/GSM-B"),
        //         ("CRC-8/HITAG", Box::new(CRCChecksum), "CRC-8/HITAG"),
        //         ("CRC-8/I-432-1", Box::new(CRCChecksum), "CRC-8/I-432-1"),
        //         ("CRC-8/I-CODE", Box::new(CRCChecksum), "CRC-8/I-CODE"),
        //         ("CRC-8/ITU", Box::new(CRCChecksum), "CRC-8/ITU"),
        //         ("CRC-8/LTE", Box::new(CRCChecksum), "CRC-8/LTE"),
        //         ("CRC-8/MAXIM", Box::new(CRCChecksum), "CRC-8/MAXIM"),
        //         ("CRC-8/MAXIM-DOW", Box::new(CRCChecksum), "CRC-8/MAXIM-DOW"),
        //         ("CRC-8/MIFARE-MAD", Box::new(CRCChecksum), "CRC-8/MIFARE-MAD"),
        //         ("CRC-8/NRSC-5", Box::new(CRCChecksum), "CRC-8/NRSC-5"),
        //         ("CRC-8/OPENSAFETY", Box::new(CRCChecksum), "CRC-8/OPENSAFETY"),
        //         ("CRC-8/ROHC", Box::new(CRCChecksum), "CRC-8/ROHC"),
        //         ("CRC-8/SAE-J1850", Box::new(CRCChecksum), "CRC-8/SAE-J1850"),
        //         ("CRC-8/SAE-J1850-ZERO", Box::new(CRCChecksum), "CRC-8/SAE-J1850-ZERO"),
        //         ("CRC-8/SMBUS", Box::new(CRCChecksum), "CRC-8/SMBUS"),
        //         ("CRC-8/TECH-3250", Box::new(CRCChecksum), "CRC-8/TECH-3250"),
        //         ("CRC-8/WCDMA", Box::new(CRCChecksum), "CRC-8/WCDMA"),
        ("Fletcher-8", Box::new(Fletcher8Checksum), ""),
        //         ("CRC-10/ATM", Box::new(CRCChecksum), "CRC-10/ATM"),
        //         ("CRC-10/CDMA2000", Box::new(CRCChecksum), "CRC-10/CDMA2000"),
        //         ("CRC-10/GSM", Box::new(CRCChecksum), "CRC-10/GSM"),
        //         ("CRC-10/I-610", Box::new(CRCChecksum), "CRC-10/I-610"),
        //         ("CRC-11/FLEXRAY", Box::new(CRCChecksum), "CRC-11/FLEXRAY"),
        //         ("CRC-11/UMTS", Box::new(CRCChecksum), "CRC-11/UMTS"),
        //         ("CRC-12/3GPP", Box::new(CRCChecksum), "CRC-12/3GPP"),
        //         ("CRC-12/CDMA2000", Box::new(CRCChecksum), "CRC-12/CDMA2000"),
        //         ("CRC-12/DECT", Box::new(CRCChecksum), "CRC-12/DECT"),
        //         ("CRC-12/GSM", Box::new(CRCChecksum), "CRC-12/GSM"),
        //         ("CRC-12/UMTS", Box::new(CRCChecksum), "CRC-12/UMTS"),
        //         ("CRC-13/BBC", Box::new(CRCChecksum), "CRC-13/BBC"),
        //         ("CRC-14/DARC", Box::new(CRCChecksum), "CRC-14/DARC"),
        //         ("CRC-14/GSM", Box::new(CRCChecksum), "CRC-14/GSM"),
        //         ("CRC-15/CAN", Box::new(CRCChecksum), "CRC-15/CAN"),
        //         ("CRC-15/MPT1327", Box::new(CRCChecksum), "CRC-15/MPT1327"),
        //         ("CRC-16", Box::new(CRCChecksum), "CRC-16"),
        //         ("CRC-16/A", Box::new(CRCChecksum), "CRC-16/A"),
        //         ("CRC-16/ACORN", Box::new(CRCChecksum), "CRC-16/ACORN"),
        //         ("CRC-16/ARC", Box::new(CRCChecksum), "CRC-16/ARC"),
        //         ("CRC-16/AUG-CCITT", Box::new(CRCChecksum), "CRC-16/AUG-CCITT"),
        //         ("CRC-16/AUTOSAR", Box::new(CRCChecksum), "CRC-16/AUTOSAR"),
        //         ("CRC-16/B", Box::new(CRCChecksum), "CRC-16/B"),
        //         ("CRC-16/BLUETOOTH", Box::new(CRCChecksum), "CRC-16/BLUETOOTH"),
        //         ("CRC-16/BUYPASS", Box::new(CRCChecksum), "CRC-16/BUYPASS"),
        //         ("CRC-16/CCITT", Box::new(CRCChecksum), "CRC-16/CCITT"),
        //         ("CRC-16/CCITT-FALSE", Box::new(CRCChecksum), "CRC-16/CCITT-FALSE"),
        //         ("CRC-16/CCITT-TRUE", Box::new(CRCChecksum), "CRC-16/CCITT-TRUE"),
        //         ("CRC-16/CCITT-ZERO", Box::new(CRCChecksum), "CRC-16/CCITT-ZERO"),
        //         ("CRC-16/CDMA2000", Box::new(CRCChecksum), "CRC-16/CDMA2000"),
        //         ("CRC-16/CMS", Box::new(CRCChecksum), "CRC-16/CMS"),
        //         ("CRC-16/DARC", Box::new(CRCChecksum), "CRC-16/DARC"),
        //         ("CRC-16/DDS-110", Box::new(CRCChecksum), "CRC-16/DDS-110"),
        //         ("CRC-16/DECT-R", Box::new(CRCChecksum), "CRC-16/DECT-R"),
        //         ("CRC-16/DECT-X", Box::new(CRCChecksum), "CRC-16/DECT-X"),
        //         ("CRC-16/DNP", Box::new(CRCChecksum), "CRC-16/DNP"),
        //         ("CRC-16/EN-13757", Box::new(CRCChecksum), "CRC-16/EN-13757"),
        //         ("CRC-16/EPC", Box::new(CRCChecksum), "CRC-16/EPC"),
        //         ("CRC-16/EPC-C1G2", Box::new(CRCChecksum), "CRC-16/EPC-C1G2"),
        //         ("CRC-16/GENIBUS", Box::new(CRCChecksum), "CRC-16/GENIBUS"),
        //         ("CRC-16/GSM", Box::new(CRCChecksum), "CRC-16/GSM"),
        //         ("CRC-16/I-CODE", Box::new(CRCChecksum), "CRC-16/I-CODE"),
        //         ("CRC-16/IBM", Box::new(CRCChecksum), "CRC-16/IBM"),
        //         ("CRC-16/IBM-3740", Box::new(CRCChecksum), "CRC-16/IBM-3740"),
        //         ("CRC-16/IBM-SDLC", Box::new(CRCChecksum), "CRC-16/IBM-SDLC"),
        //         ("CRC-16/IEC-61158-2", Box::new(CRCChecksum), "CRC-16/IEC-61158-2"),
        //         ("CRC-16/ISO-HDLC", Box::new(CRCChecksum), "CRC-16/ISO-HDLC"),
        //         ("CRC-16/ISO-IEC-14443-3-A", Box::new(CRCChecksum), "CRC-16/ISO-IEC-14443-3-A"),
        //         ("CRC-16/ISO-IEC-14443-3-B", Box::new(CRCChecksum), "CRC-16/ISO-IEC-14443-3-B"),
        //         ("CRC-16/KERMIT", Box::new(CRCChecksum), "CRC-16/KERMIT"),
        //         ("CRC-16/LHA", Box::new(CRCChecksum), "CRC-16/LHA"),
        //         ("CRC-16/LJ1200", Box::new(CRCChecksum), "CRC-16/LJ1200"),
        //         ("CRC-16/LTE", Box::new(CRCChecksum), "CRC-16/LTE"),
        //         ("CRC-16/M17", Box::new(CRCChecksum), "CRC-16/M17"),
        //         ("CRC-16/MAXIM", Box::new(CRCChecksum), "CRC-16/MAXIM"),
        //         ("CRC-16/MAXIM-DOW", Box::new(CRCChecksum), "CRC-16/MAXIM-DOW"),
        //         ("CRC-16/MCRF4XX", Box::new(CRCChecksum), "CRC-16/MCRF4XX"),
        //         ("CRC-16/MODBUS", Box::new(CRCChecksum), "CRC-16/MODBUS"),
        //         ("CRC-16/NRSC-5", Box::new(CRCChecksum), "CRC-16/NRSC-5"),
        //         ("CRC-16/OPENSAFETY-A", Box::new(CRCChecksum), "CRC-16/OPENSAFETY-A"),
        //         ("CRC-16/OPENSAFETY-B", Box::new(CRCChecksum), "CRC-16/OPENSAFETY-B"),
        //         ("CRC-16/PROFIBUS", Box::new(CRCChecksum), "CRC-16/PROFIBUS"),
        //         ("CRC-16/RIELLO", Box::new(CRCChecksum), "CRC-16/RIELLO"),
        //         ("CRC-16/SPI-FUJITSU", Box::new(CRCChecksum), "CRC-16/SPI-FUJITSU"),
        //         ("CRC-16/T10-DIF", Box::new(CRCChecksum), "CRC-16/T10-DIF"),
        //         ("CRC-16/TELEDISK", Box::new(CRCChecksum), "CRC-16/TELEDISK"),
        //         ("CRC-16/TMS37157", Box::new(CRCChecksum), "CRC-16/TMS37157"),
        //         ("CRC-16/UMTS", Box::new(CRCChecksum), "CRC-16/UMTS"),
        //         ("CRC-16/USB", Box::new(CRCChecksum), "CRC-16/USB"),
        //         ("CRC-16/V-41-LSB", Box::new(CRCChecksum), "CRC-16/V-41-LSB"),
        //         ("CRC-16/V-41-MSB", Box::new(CRCChecksum), "CRC-16/V-41-MSB"),
        //         ("CRC-16/VERIFONE", Box::new(CRCChecksum), "CRC-16/VERIFONE"),
        //         ("CRC-16/X-25", Box::new(CRCChecksum), "CRC-16/X-25"),
        //         ("CRC-16/XMODEM", Box::new(CRCChecksum), "CRC-16/XMODEM"),
        //         ("CRC-16/ZMODEM", Box::new(CRCChecksum), "CRC-16/ZMODEM"),
        ("Fletcher-16", Box::new(Fletcher16Checksum), ""),
        //         ("CRC-17/CAN-FD", Box::new(CRCChecksum), "CRC-17/CAN-FD"),
        //         ("CRC-21/CAN-FD", Box::new(CRCChecksum), "CRC-21/CAN-FD"),
        //         ("CRC-24/BLE", Box::new(CRCChecksum), "CRC-24/BLE"),
        //         ("CRC-24/FLEXRAY-A", Box::new(CRCChecksum), "CRC-24/FLEXRAY-A"),
        //         ("CRC-24/FLEXRAY-B", Box::new(CRCChecksum), "CRC-24/FLEXRAY-B"),
        //         ("CRC-24/INTERLAKEN", Box::new(CRCChecksum), "CRC-24/INTERLAKEN"),
        //         ("CRC-24/LTE-A", Box::new(CRCChecksum), "CRC-24/LTE-A"),
        //         ("CRC-24/LTE-B", Box::new(CRCChecksum), "CRC-24/LTE-B"),
        //         ("CRC-24/OPENPGP", Box::new(CRCChecksum), "CRC-24/OPENPGP"),
        //         ("CRC-24/OS-9", Box::new(CRCChecksum), "CRC-24/OS-9"),
        //         ("CRC-30/CDMA", Box::new(CRCChecksum), "CRC-30/CDMA"),
        //         ("CRC-31/PHILIPS", Box::new(CRCChecksum), "CRC-31/PHILIPS"),
        ("Adler-32", Box::new(Adler32Checksum), ""),
        //         ("CRC-32", Box::new(CRCChecksum), "CRC-32"),
        //         ("CRC-32/AAL5", Box::new(CRCChecksum), "CRC-32/AAL5"),
        //         ("CRC-32/ADCCP", Box::new(CRCChecksum), "CRC-32/ADCCP"),
        //         ("CRC-32/AIXM", Box::new(CRCChecksum), "CRC-32/AIXM"),
        //         ("CRC-32/AUTOSAR", Box::new(CRCChecksum), "CRC-32/AUTOSAR"),
        //         ("CRC-32/BASE91-C", Box::new(CRCChecksum), "CRC-32/BASE91-C"),
        //         ("CRC-32/BASE91-D", Box::new(CRCChecksum), "CRC-32/BASE91-D"),
        //         ("CRC-32/BZIP2", Box::new(CRCChecksum), "CRC-32/BZIP2"),
        //         ("CRC-32/C", Box::new(CRCChecksum), "CRC-32/C"),
        //         ("CRC-32/CASTAGNOLI", Box::new(CRCChecksum), "CRC-32/CASTAGNOLI"),
        //         ("CRC-32/CD-ROM-EDC", Box::new(CRCChecksum), "CRC-32/CD-ROM-EDC"),
        //         ("CRC-32/CKSUM", Box::new(CRCChecksum), "CRC-32/CKSUM"),
        //         ("CRC-32/D", Box::new(CRCChecksum), "CRC-32/D"),
        //         ("CRC-32/DECT-B", Box::new(CRCChecksum), "CRC-32/DECT-B"),
        //         ("CRC-32/INTERLAKEN", Box::new(CRCChecksum), "CRC-32/INTERLAKEN"),
        //         ("CRC-32/ISCSI", Box::new(CRCChecksum), "CRC-32/ISCSI"),
        //         ("CRC-32/ISO-HDLC", Box::new(CRCChecksum), "CRC-32/ISO-HDLC"),
        //         ("CRC-32/JAMCRC", Box::new(CRCChecksum), "CRC-32/JAMCRC"),
        //         ("CRC-32/MEF", Box::new(CRCChecksum), "CRC-32/MEF"),
        //         ("CRC-32/MPEG-2", Box::new(CRCChecksum), "CRC-32/MPEG-2"),
        //         ("CRC-32/NVME", Box::new(CRCChecksum), "CRC-32/NVME"),
        //         ("CRC-32/PKZIP", Box::new(CRCChecksum), "CRC-32/PKZIP"),
        //         ("CRC-32/POSIX", Box::new(CRCChecksum), "CRC-32/POSIX"),
        //         ("CRC-32/Q", Box::new(CRCChecksum), "CRC-32/Q"),
        //         ("CRC-32/SATA", Box::new(CRCChecksum), "CRC-32/SATA"),
        //         ("CRC-32/V-42", Box::new(CRCChecksum), "CRC-32/V-42"),
        //         ("CRC-32/XFER", Box::new(CRCChecksum), "CRC-32/XFER"),
        //         ("CRC-32/XZ", Box::new(CRCChecksum), "CRC-32/XZ"),
        ("Fletcher-32", Box::new(Fletcher32Checksum), ""),
        //         ("CRC-40/GSM", Box::new(CRCChecksum), "CRC-40/GSM"),
        //         ("CRC-64/ECMA-182", Box::new(CRCChecksum), "CRC-64/ECMA-182"),
        //         ("CRC-64/GO-ECMA", Box::new(CRCChecksum), "CRC-64/GO-ECMA"),
        //         ("CRC-64/GO-ISO", Box::new(CRCChecksum), "CRC-64/GO-ISO"),
        //         ("CRC-64/MS", Box::new(CRCChecksum), "CRC-64/MS"),
        //         ("CRC-64/NVME", Box::new(CRCChecksum), "CRC-64/NVME"),
        //         ("CRC-64/REDIS", Box::new(CRCChecksum), "CRC-64/REDIS"),
        //         ("CRC-64/WE", Box::new(CRCChecksum), "CRC-64/WE"),
        //         ("CRC-64/XZ", Box::new(CRCChecksum), "CRC-64/XZ"),
        ("Fletcher-64", Box::new(Fletcher64Checksum), ""),
        //         ("CRC-82/DARC", Box::new(CRCChecksum), "CRC-82/DARC"),
    ]
}
