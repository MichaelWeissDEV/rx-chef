/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CRC Checksum operation.
 * -----------------------------------------------------------------------------
 */

use crc::Crc;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// CRC Checksum operation
///
/// A Cyclic Redundancy Check (CRC) is an error-detecting code commonly used in
/// digital networks and storage devices to detect accidental changes to raw data.
/// Supports a selection of widely-used CRC algorithms across 8, 16, and 32 bit widths.
pub struct CrcChecksum;

impl Operation for CrcChecksum {
    fn name(&self) -> &'static str {
        "CRC Checksum"
    }

    fn module(&self) -> &'static str {
        "Hashing"
    }

    fn description(&self) -> &'static str {
        "A Cyclic Redundancy Check (CRC) is an error-detecting code commonly used in \
        digital networks and storage devices to detect accidental changes to raw data. \
        Supports CRC-8, CRC-16, and CRC-32 variants."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[ArgSchema {
            name: "Algorithm",
            description: "CRC algorithm to use",
            default_value: "CRC-32",
        }];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let algorithm = args
            .first()
            .and_then(|a| a.as_str())
            .unwrap_or("CRC-32")
            .to_uppercase();

        let result = match algorithm.as_str() {
            // ---- 8-bit algorithms ----
            "CRC-8" | "CRC-8/SMBUS" => {
                let c = Crc::<u8>::new(&crc::CRC_8_SMBUS);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/AUTOSAR" | "CRC-8/8H2F" => {
                let c = Crc::<u8>::new(&crc::CRC_8_AUTOSAR);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/BLUETOOTH" => {
                let c = Crc::<u8>::new(&crc::CRC_8_BLUETOOTH);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/CDMA2000" => {
                let c = Crc::<u8>::new(&crc::CRC_8_CDMA2000);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/DARC" => {
                let c = Crc::<u8>::new(&crc::CRC_8_DARC);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/DVB-S2" => {
                let c = Crc::<u8>::new(&crc::CRC_8_DVB_S2);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/GSM-A" => {
                let c = Crc::<u8>::new(&crc::CRC_8_GSM_A);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/GSM-B" => {
                let c = Crc::<u8>::new(&crc::CRC_8_GSM_B);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/HITAG" => {
                let c = Crc::<u8>::new(&crc::CRC_8_HITAG);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/I-432-1" | "CRC-8/ITU" => {
                let c = Crc::<u8>::new(&crc::CRC_8_I_432_1);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/I-CODE" => {
                let c = Crc::<u8>::new(&crc::CRC_8_I_CODE);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/LTE" => {
                let c = Crc::<u8>::new(&crc::CRC_8_LTE);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/MAXIM" | "CRC-8/MAXIM-DOW" => {
                let c = Crc::<u8>::new(&crc::CRC_8_MAXIM_DOW);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/MIFARE-MAD" => {
                let c = Crc::<u8>::new(&crc::CRC_8_MIFARE_MAD);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/NRSC-5" => {
                let c = Crc::<u8>::new(&crc::CRC_8_NRSC_5);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/OPENSAFETY" => {
                let c = Crc::<u8>::new(&crc::CRC_8_OPENSAFETY);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/ROHC" => {
                let c = Crc::<u8>::new(&crc::CRC_8_ROHC);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/SAE-J1850" => {
                let c = Crc::<u8>::new(&crc::CRC_8_SAE_J1850);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/TECH-3250" | "CRC-8/AES" | "CRC-8/EBU" => {
                let c = Crc::<u8>::new(&crc::CRC_8_TECH_3250);
                format!("{:02x}", c.checksum(&input))
            }
            "CRC-8/WCDMA" => {
                let c = Crc::<u8>::new(&crc::CRC_8_WCDMA);
                format!("{:02x}", c.checksum(&input))
            }
            // ---- 16-bit algorithms ----
            "CRC-16" | "CRC-16/ARC" | "CRC-16/IBM" | "CRC-16/LHA" => {
                let c = Crc::<u16>::new(&crc::CRC_16_ARC);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/AUTOSAR" | "CRC-16/CCITT-FALSE" | "CRC-16/IBM-3740" => {
                let c = Crc::<u16>::new(&crc::CRC_16_IBM_3740);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/BLUETOOTH" | "CRC-16/CCITT" | "CRC-16/CCITT-TRUE" | "CRC-16/KERMIT"
            | "CRC-16/V-41-LSB" => {
                let c = Crc::<u16>::new(&crc::CRC_16_KERMIT);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/BUYPASS" | "CRC-16/UMTS" | "CRC-16/VERIFONE" => {
                let c = Crc::<u16>::new(&crc::CRC_16_UMTS);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/CDMA2000" => {
                let c = Crc::<u16>::new(&crc::CRC_16_CDMA2000);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/CMS" => {
                let c = Crc::<u16>::new(&crc::CRC_16_CMS);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/DDS-110" => {
                let c = Crc::<u16>::new(&crc::CRC_16_DDS_110);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/DECT-R" => {
                let c = Crc::<u16>::new(&crc::CRC_16_DECT_R);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/DECT-X" => {
                let c = Crc::<u16>::new(&crc::CRC_16_DECT_X);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/DNP" => {
                let c = Crc::<u16>::new(&crc::CRC_16_DNP);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/EN-13757" => {
                let c = Crc::<u16>::new(&crc::CRC_16_EN_13757);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/GENIBUS" | "CRC-16/DARC" | "CRC-16/EPC" | "CRC-16/EPC-C1G2"
            | "CRC-16/I-CODE" => {
                let c = Crc::<u16>::new(&crc::CRC_16_GENIBUS);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/GSM" => {
                let c = Crc::<u16>::new(&crc::CRC_16_GSM);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/IBM-SDLC"
            | "CRC-16/B"
            | "CRC-16/ISO-HDLC"
            | "CRC-16/ISO-IEC-14443-3-B"
            | "CRC-16/X-25" => {
                let c = Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/ISO-IEC-14443-3-A" | "CRC-16/A" => {
                let c = Crc::<u16>::new(&crc::CRC_16_ISO_IEC_14443_3_A);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/LJ1200" => {
                let c = Crc::<u16>::new(&crc::CRC_16_LJ1200);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/M17" => {
                let c = Crc::<u16>::new(&crc::CRC_16_M17);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/MAXIM" | "CRC-16/MAXIM-DOW" => {
                let c = Crc::<u16>::new(&crc::CRC_16_MAXIM_DOW);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/MCRF4XX" => {
                let c = Crc::<u16>::new(&crc::CRC_16_MCRF4XX);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/MODBUS" => {
                let c = Crc::<u16>::new(&crc::CRC_16_MODBUS);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/NRSC-5" => {
                let c = Crc::<u16>::new(&crc::CRC_16_NRSC_5);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/OPENSAFETY-A" => {
                let c = Crc::<u16>::new(&crc::CRC_16_OPENSAFETY_A);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/OPENSAFETY-B" => {
                let c = Crc::<u16>::new(&crc::CRC_16_OPENSAFETY_B);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/PROFIBUS" | "CRC-16/IEC-61158-2" => {
                let c = Crc::<u16>::new(&crc::CRC_16_PROFIBUS);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/RIELLO" => {
                let c = Crc::<u16>::new(&crc::CRC_16_RIELLO);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/SPI-FUJITSU" | "CRC-16/AUG-CCITT" => {
                let c = Crc::<u16>::new(&crc::CRC_16_SPI_FUJITSU);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/T10-DIF" => {
                let c = Crc::<u16>::new(&crc::CRC_16_T10_DIF);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/TELEDISK" => {
                let c = Crc::<u16>::new(&crc::CRC_16_TELEDISK);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/TMS37157" => {
                let c = Crc::<u16>::new(&crc::CRC_16_TMS37157);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/USB" => {
                let c = Crc::<u16>::new(&crc::CRC_16_USB);
                format!("{:04x}", c.checksum(&input))
            }
            "CRC-16/XMODEM" | "CRC-16/ACORN" | "CRC-16/LTE" | "CRC-16/V-41-MSB"
            | "CRC-16/ZMODEM" | "CRC-16/CCITT-ZERO" => {
                let c = Crc::<u16>::new(&crc::CRC_16_XMODEM);
                format!("{:04x}", c.checksum(&input))
            }
            // ---- 32-bit algorithms ----
            "CRC-32" | "CRC-32/ADCCP" | "CRC-32/ISO-HDLC" | "CRC-32/PKZIP" | "CRC-32/V-42"
            | "CRC-32/XZ" => {
                let c = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/AIXM" | "CRC-32/Q" => {
                let c = Crc::<u32>::new(&crc::CRC_32_AIXM);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/AUTOSAR" => {
                let c = Crc::<u32>::new(&crc::CRC_32_AUTOSAR);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/BASE91-D" | "CRC-32/D" => {
                let c = Crc::<u32>::new(&crc::CRC_32_BASE91_D);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/BZIP2" | "CRC-32/AAL5" | "CRC-32/DECT-B" => {
                let c = Crc::<u32>::new(&crc::CRC_32_BZIP2);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/C" | "CRC-32/BASE91-C" | "CRC-32/CASTAGNOLI" | "CRC-32/INTERLAKEN"
            | "CRC-32/ISCSI" | "CRC-32/NVME" => {
                let c = Crc::<u32>::new(&crc::CRC_32_ISCSI);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/CD-ROM-EDC" => {
                let c = Crc::<u32>::new(&crc::CRC_32_CD_ROM_EDC);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/CKSUM" | "CRC-32/POSIX" => {
                let c = Crc::<u32>::new(&crc::CRC_32_CKSUM);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/JAMCRC" => {
                let c = Crc::<u32>::new(&crc::CRC_32_JAMCRC);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/MEF" => {
                let c = Crc::<u32>::new(&crc::CRC_32_MEF);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/MPEG-2" => {
                let c = Crc::<u32>::new(&crc::CRC_32_MPEG_2);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/SATA" => {
                // CRC-32/SATA: poly=0x04C11DB7, init=0x52325032, refin=false, refout=false, xorout=0
                const CRC_32_SATA_ALG: crc::Algorithm<u32> = crc::Algorithm {
                    width: 32,
                    poly: 0x04c11db7,
                    init: 0x52325032,
                    refin: false,
                    refout: false,
                    xorout: 0x00000000,
                    check: 0xcf72afe8,
                    residue: 0x00000000,
                };
                let c = Crc::<u32>::new(&CRC_32_SATA_ALG);
                format!("{:08x}", c.checksum(&input))
            }
            "CRC-32/XFER" => {
                let c = Crc::<u32>::new(&crc::CRC_32_XFER);
                format!("{:08x}", c.checksum(&input))
            }
            _ => {
                return Err(OperationError::InvalidArgument {
                    name: "Algorithm".to_string(),
                    reason: format!("Unknown CRC algorithm: {}", algorithm),
                });
            }
        };

        Ok(result.into_bytes())
    }
}
