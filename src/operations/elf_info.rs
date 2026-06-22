/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the ELF Info operation.
 * -----------------------------------------------------------------------------
 */

use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// ELF Info operation
pub struct ELFInfo;

impl Operation for ELFInfo {
    fn name(&self) -> &'static str {
        "ELF Info"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Implements readelf-like functionality. This operation will extract the ELF Header, Program Headers, Section Headers and Symbol Table for an ELF file."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let mut stream = ElfStream::new(&input);
        let mut result = Vec::new();
        let align = 30;

        result.push("=".repeat(align) + " ELF Header " + &"=".repeat(align));

        let header_info = match elf_header(&mut stream, align) {
            Ok(info) => info,
            Err(e) => return Err(OperationError::ProcessingError(e)),
        };
        result.push(header_info.output.clone() + "\n");

        let names_offset = get_names_offset(&mut stream, &header_info);

        result.push("=".repeat(align) + " Program Header " + &"=".repeat(align));
        stream.move_to(header_info.phoff as u64);
        for _ in 0..header_info.ph_entries {
            result.push(program_header(&mut stream, &header_info, align) + "\n");
        }

        result.push("=".repeat(align) + " Section Header " + &"=".repeat(align));
        stream.move_to(header_info.shoff as u64);
        let mut symtab_info = None;
        let mut strtab_offset = 0;

        for _ in 0..header_info.sh_entries {
            let (sh_output, _sh_type, name, sh_offset, sh_size, sh_entsize) =
                section_header(&mut stream, &header_info, names_offset, align);
            result.push(sh_output + "\n");

            if name == ".strtab" {
                strtab_offset = sh_offset;
            } else if name == ".symtab" {
                symtab_info = Some((sh_offset, sh_size, sh_entsize));
            }
        }

        result.push("=".repeat(align) + " Symbol Table " + &"=".repeat(align));
        if let Some((sym_offset, sym_size, sym_entsize)) = symtab_info {
            stream.move_to(sym_offset as u64);
            let count = if sym_entsize > 0 {
                sym_size / sym_entsize
            } else {
                0
            };
            for _ in 0..count {
                if let Some(symbol_name) = get_symbol(&mut stream, &header_info, strtab_offset) {
                    if !symbol_name.is_empty() {
                        result.push(format!(
                            "{: <align$}{}",
                            "Symbol Name:",
                            symbol_name,
                            align = align
                        ));
                    }
                }
            }
        }

        Ok(result.join("\n").into_bytes())
    }
}

#[allow(dead_code)]
struct ElfStream<'a> {
    cursor: Cursor<&'a [u8]>,
    data: &'a [u8],
}

impl<'a> ElfStream<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
            data,
        }
    }

    fn move_to(&mut self, pos: u64) {
        self.cursor.set_position(pos);
    }

    fn position(&self) -> u64 {
        self.cursor.position()
    }

    fn move_forwards(&mut self, offset: i64) {
        let new_pos = self.cursor.position() as i64 + offset;
        if new_pos >= 0 {
            self.cursor.set_position(new_pos as u64);
        }
    }

    fn read_u8(&mut self) -> u8 {
        self.cursor.read_u8().unwrap_or(0)
    }

    fn read_u16(&mut self, le: bool) -> u16 {
        if le {
            self.cursor.read_u16::<LittleEndian>().unwrap_or(0)
        } else {
            self.cursor.read_u16::<BigEndian>().unwrap_or(0)
        }
    }

    fn read_u32(&mut self, le: bool) -> u32 {
        if le {
            self.cursor.read_u32::<LittleEndian>().unwrap_or(0)
        } else {
            self.cursor.read_u32::<BigEndian>().unwrap_or(0)
        }
    }

    fn read_u64(&mut self, le: bool) -> u64 {
        if le {
            self.cursor.read_u64::<LittleEndian>().unwrap_or(0)
        } else {
            self.cursor.read_u64::<BigEndian>().unwrap_or(0)
        }
    }

    fn read_int(&mut self, size: usize, le: bool) -> u64 {
        match size {
            1 => self.read_u8() as u64,
            2 => self.read_u16(le) as u64,
            4 => self.read_u32(le) as u64,
            8 => self.read_u64(le) as u64,
            _ => 0,
        }
    }

    fn get_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut buf = vec![0u8; size];
        use std::io::Read;
        let _ = self.cursor.read_exact(&mut buf);
        buf
    }

    fn read_string(&mut self) -> String {
        let mut s = Vec::new();
        while let Ok(b) = self.cursor.read_u8() {
            if b == 0 {
                break;
            }
            s.push(b);
        }
        String::from_utf8_lossy(&s).to_string()
    }
}

struct HeaderInfo {
    output: String,
    format: u8,
    endianness_le: bool,
    phoff: u64,
    ph_entries: u16,
    shoff: u64,
    sh_entries: u16,
    shent_size: u16,
    shstrtab: u16,
}

fn elf_header(stream: &mut ElfStream, align: usize) -> Result<HeaderInfo, String> {
    let mut eh_result = Vec::new();
    let magic = stream.get_bytes(4);
    if magic != [0x7f, 0x45, 0x4c, 0x46] {
        return Err("Invalid ELF".to_string());
    }

    eh_result.push(format!(
        "{: <align$}{}",
        "Magic:",
        String::from_utf8_lossy(&magic),
        align = align
    ));

    let format = stream.read_u8();
    eh_result.push(format!(
        "{: <align$}{}",
        "Format:",
        if format == 1 { "32-bit" } else { "64-bit" },
        align = align
    ));

    let endianness_le = stream.read_u8() == 1;
    eh_result.push(format!(
        "{: <align$}{}",
        "Endianness:",
        if endianness_le { "Little" } else { "Big" },
        align = align
    ));

    eh_result.push(format!(
        "{: <align$}{}",
        "Version:",
        stream.read_u8().to_string(),
        align = align
    ));

    let abi_val = stream.read_u8();
    let abi = match abi_val {
        0x00 => "System V",
        0x01 => "HP-UX",
        0x02 => "NetBSD",
        0x03 => "Linux",
        0x04 => "GNU Hurd",
        0x06 => "Solaris",
        0x07 => "AIX",
        0x08 => "IRIX",
        0x09 => "FreeBSD",
        0x0A => "Tru64",
        0x0B => "Novell Modesto",
        0x0C => "OpenBSD",
        0x0D => "OpenVMS",
        0x0E => "NonStop Kernel",
        0x0F => "AROS",
        0x10 => "Fenix OS",
        0x11 => "CloudABI",
        0x12 => "Stratus Technologies OpenVOS",
        _ => "",
    };
    eh_result.push(format!("{: <align$}{}", "ABI:", abi, align = align));

    let abi_version = stream.read_u8().to_string();
    if abi != "Linux" {
        eh_result.push(format!(
            "{: <align$}{}",
            "ABI Version:",
            abi_version,
            align = align
        ));
    }

    stream.move_forwards(7);

    let e_type_val = stream.read_u16(endianness_le);
    let e_type = match e_type_val {
        0x0000 => "Unknown",
        0x0001 => "Relocatable File",
        0x0002 => "Executable File",
        0x0003 => "Shared Object",
        0x0004 => "Core File",
        0xFE00 => "LOOS",
        0xFEFF => "HIOS",
        0xFF00 => "LOPROC",
        0xFFFF => "HIPROC",
        _ => "",
    };
    eh_result.push(format!("{: <align$}{}", "Type:", e_type, align = align));

    let isa_val = stream.read_u16(endianness_le);
    let isa = match isa_val {
        0x0000 => "No specific instruction set",
        0x0001 => "AT&T WE 32100",
        0x0002 => "SPARC",
        0x0003 => "x86",
        0x0004 => "Motorola 68000 (M68k)",
        0x0005 => "Motorola 88000 (M88k)",
        0x0006 => "Intel MCU",
        0x0007 => "Intel 80860",
        0x0008 => "MIPS",
        0x0009 => "IBM System/370",
        0x000A => "MIPS RS3000 Little-endian",
        0x000B..=0x000E | 0x0018..=0x0023 => "Reserved for future use",
        0x000F => "Hewlett-Packard PA-RISC",
        0x0011 => "Fujitsu VPP500",
        0x0012 => "Enhanced instruction set SPARC",
        0x0013 => "Intel 80960",
        0x0014 => "PowerPC",
        0x0015 => "PowerPC (64-bit)",
        0x0016 => "S390, including S390",
        0x0017 => "IBM SPU/SPC",
        0x0024 => "NEC V800",
        0x0025 => "Fujitsu FR20",
        0x0026 => "TRW RH-32",
        0x0027 => "Motorola RCE",
        0x0028 => "ARM (up to ARMv7/Aarch32)",
        0x0029 => "Digital Alpha",
        0x002A => "SuperH",
        0x002B => "SPARC Version 9",
        0x002C => "Siemens TriCore embedded processor",
        0x002D => "Argonaut RISC Core",
        0x002E => "Hitachi H8/300",
        0x002F => "Hitachi H8/300H",
        0x0030 => "Hitachi H8S",
        0x0031 => "Hitachi H8/500",
        0x0032 => "IA-64",
        0x0033 => "Standford MIPS-X",
        0x0034 => "Motorola ColdFire",
        0x0035 => "Motorola M68HC12",
        0x0036 => "Fujitsu MMA Multimedia Accelerator",
        0x0037 => "Siemens PCP",
        0x0038 => "Sony nCPU embedded RISC processor",
        0x0039 => "Denso NDR1 microprocessor",
        0x003A => "Motorola Star*Core processor",
        0x003B => "Toyota ME16 processor",
        0x003C => "STMicroelectronics ST100 processor",
        0x003D => "Advanced Logic Corp. TinyJ embedded processor family",
        0x003E => "AMD x86-64",
        0x003F => "Sony DSP Processor",
        0x0040 => "Digital Equipment Corp. PDP-10",
        0x0041 => "Digital Equipment Corp. PDP-11",
        0x0042 => "Siemens FX66 microcontroller",
        0x0043 => "STMicroelectronics ST9+ 8/16 bit microcontroller",
        0x0044 => "STMicroelectronics ST7 8-bit microcontroller",
        0x0045 => "Motorola MC68HC16 Microcontroller",
        0x0046 => "Motorola MC68HC11 Microcontroller",
        0x0047 => "Motorola MC68HC08 Microcontroller",
        0x0048 => "Motorola MC68HC05 Microcontroller",
        0x0049 => "Silicon Graphics SVx",
        0x004A => "STMicroelectronics ST19 8-bit microcontroller",
        0x004B => "Digital VAX",
        0x004C => "Axis Communications 32-bit embedded processor",
        0x004D => "Infineon Technologies 32-bit embedded processor",
        0x004E => "Element 14 64-bit DSP Processor",
        0x004F => "LSI Logic 16-bit DSP Processor",
        0x0050 => "Donald Knuth's educational 64-bit processor",
        0x0051 => "Harvard University machine-independent object files",
        0x0052 => "SiTera Prism",
        0x0053 => "Atmel AVR 8-bit microcontroller",
        0x0054 => "Fujitsu FR30",
        0x0055 => "Mitsubishi D10V",
        0x0056 => "Mitsubishi D30V",
        0x0057 => "NEC v850",
        0x0058 => "Mitsubishi M32R",
        0x0059 => "Matsushita MN10300",
        0x005A => "Matsushita MN10200",
        0x005B => "picoJava",
        0x005C => "OpenRISC 32-bit embedded processor",
        0x005D => "ARC Cores Tangent-A5",
        0x005E => "Tensilica Xtensa Architecture",
        0x005F => "Alphamosaic VideoCore processor",
        0x0060 => "Thompson Multimedia General Purpose Processor",
        0x0061 => "National Semiconductor 32000 series",
        0x0062 => "Tenor Network TPC processor",
        0x0063 => "Trebia SNP 1000 processor",
        0x0064 => "STMicroelectronics (www.st.com) ST200 microcontroller",
        0x008C => "TMS320C6000 Family",
        0x00AF => "MCST Elbrus e2k",
        0x00B7 => "ARM 64-bits (ARMv8/Aarch64)",
        0x00F3 => "RISC-V",
        0x00F7 => "Berkeley Packet Filter",
        0x0101 => "WDC 65C816",
        _ => "Unimplemented",
    };
    eh_result.push(format!(
        "{: <align$}{}",
        "Instruction Set Architecture:",
        isa,
        align = align
    ));

    eh_result.push(format!(
        "{: <align$}{}",
        "ELF Version:",
        stream.read_u32(endianness_le).to_string(),
        align = align
    ));

    let read_size = if format == 1 { 4 } else { 8 };
    let entry = stream.read_int(read_size, endianness_le);
    let phoff = stream.read_int(read_size, endianness_le);
    let shoff = stream.read_int(read_size, endianness_le);
    eh_result.push(format!(
        "{: <align$}0x{:x}",
        "Entry Point:",
        entry,
        align = align
    ));
    eh_result.push(format!(
        "{: <align$}0x{:x}",
        "Entry PHOFF:",
        phoff,
        align = align
    ));
    eh_result.push(format!(
        "{: <align$}0x{:x}",
        "Entry SHOFF:",
        shoff,
        align = align
    ));

    let flags = stream.read_u32(endianness_le);
    eh_result.push(format!("{: <align$}{:08x}", "Flags:", flags, align = align));

    eh_result.push(format!(
        "{: <align$}{} bytes",
        "ELF Header Size:",
        stream.read_u16(endianness_le),
        align = align
    ));
    eh_result.push(format!(
        "{: <align$}{} bytes",
        "Program Header Size:",
        stream.read_u16(endianness_le),
        align = align
    ));
    let ph_entries = stream.read_u16(endianness_le);
    eh_result.push(format!(
        "{: <align$}{}",
        "Program Header Entries:",
        ph_entries,
        align = align
    ));
    let shent_size = stream.read_u16(endianness_le);
    eh_result.push(format!(
        "{: <align$}{} bytes",
        "Section Header Size:",
        shent_size,
        align = align
    ));
    let sh_entries = stream.read_u16(endianness_le);
    eh_result.push(format!(
        "{: <align$}{}",
        "Section Header Entries:",
        sh_entries,
        align = align
    ));
    let shstrtab = stream.read_u16(endianness_le);
    eh_result.push(format!(
        "{: <align$}{}",
        "Section Header Names:",
        shstrtab,
        align = align
    ));

    Ok(HeaderInfo {
        output: eh_result.join("\n"),
        format,
        endianness_le,
        phoff,
        ph_entries,
        shoff,
        sh_entries,
        shent_size,
        shstrtab,
    })
}

fn program_header(stream: &mut ElfStream, info: &HeaderInfo, align: usize) -> String {
    let mut ph_result = Vec::new();
    let p_type_val = stream.read_u32(info.endianness_le);
    let p_type = match p_type_val {
        0x00000000 => "Unused",
        0x00000001 => "Loadable Segment",
        0x00000002 => "Dynamic linking information",
        0x00000003 => "Interpreter Information",
        0x00000004 => "Auxiliary Information",
        0x00000005 => "Reserved",
        0x00000006 => "Program Header Table",
        0x00000007 => "Thread-Local Storage Template",
        0x60000000..=0x6FFFFFFF => "Reserved Inclusive Range. OS Specific",
        0x70000000..=0x7FFFFFFF => "Reserved Inclusive Range. Processor Specific",
        _ => "",
    };
    ph_result.push(format!(
        "{: <align$}{}",
        "Program Header Type:",
        p_type,
        align = align
    ));

    if info.format == 2 {
        ph_result.push(format!(
            "{: <align$}{}",
            "Flags:",
            read_ph_flags(stream.read_u32(info.endianness_le)),
            align = align
        ));
    }

    let read_size = if info.format == 1 { 4 } else { 8 };
    ph_result.push(format!(
        "{: <align$}{}",
        "Offset Of Segment:",
        stream.read_int(read_size, info.endianness_le),
        align = align
    ));
    ph_result.push(format!(
        "{: <align$}{}",
        "Virtual Address of Segment:",
        stream.read_int(read_size, info.endianness_le),
        align = align
    ));
    ph_result.push(format!(
        "{: <align$}{}",
        "Physical Address of Segment:",
        stream.read_int(read_size, info.endianness_le),
        align = align
    ));
    ph_result.push(format!(
        "{: <align$}{} bytes",
        "Size of Segment:",
        stream.read_int(read_size, info.endianness_le),
        align = align
    ));
    ph_result.push(format!(
        "{: <align$}{} bytes",
        "Size of Segment in Memory:",
        stream.read_int(read_size, info.endianness_le),
        align = align
    ));

    if info.format == 1 {
        ph_result.push(format!(
            "{: <align$}{}",
            "Flags:",
            read_ph_flags(stream.read_u32(info.endianness_le)),
            align = align
        ));
    }

    stream.move_forwards(read_size as i64);

    ph_result.join("\n")
}

fn read_ph_flags(flags: u32) -> String {
    let mut result = Vec::new();
    if flags & 0x1 != 0 {
        result.push("Execute");
    }
    if flags & 0x2 != 0 {
        result.push("Write");
    }
    if flags & 0x4 != 0 {
        result.push("Read");
    }
    if flags & 0xf0000000 != 0 {
        result.push("Unspecified");
    }
    result.join(",")
}

fn section_header(
    stream: &mut ElfStream,
    info: &HeaderInfo,
    names_offset: u64,
    align: usize,
) -> (String, u32, String, u64, u64, u64) {
    let mut sh_result = Vec::new();
    let name_offset = stream.read_u32(info.endianness_le);
    let sh_type = stream.read_u32(info.endianness_le);
    let type_str = match sh_type {
        0x00000001 => "Program Data",
        0x00000002 => "Symbol Table",
        0x00000003 => "String Table",
        0x00000004 => "Relocation Entries with Addens",
        0x00000005 => "Symbol Hash Table",
        0x00000006 => "Dynamic Linking Information",
        0x00000007 => "Notes",
        0x00000008 => "Program Space with No Data",
        0x00000009 => "Relocation Entries with no Addens",
        0x0000000A => "Reserved",
        0x0000000B => "Dynamic Linker Symbol Table",
        0x0000000E => "Array of Constructors",
        0x0000000F => "Array of Destructors",
        0x00000010 => "Array of pre-constructors",
        0x00000011 => "Section group",
        0x00000012 => "Extended section indices",
        0x00000013 => "Number of defined types",
        0x60000000..=0x6fffffff => "OS-specific",
        0x70000000..=0x7fffffff => "Processor-specific",
        0x80000000..=0x8fffffff => "Application-specific",
        _ => "Unused",
    };

    sh_result.push(format!("{: <align$}{}", "Type:", type_str, align = align));

    let mut section_name = String::new();
    if type_str != "Unused" {
        section_name = read_string_at(stream, names_offset + name_offset as u64);
        sh_result.push(format!(
            "{: <align$}{}",
            "Section Name: ",
            section_name,
            align = align
        ));
    }

    let read_size = if info.format == 1 { 4 } else { 8 };
    let flags = stream.read_int(read_size, info.endianness_le);
    let mut sh_flags = Vec::new();
    let bit_masks = [
        (0x00000001, "Writable"),
        (0x00000002, "Alloc"),
        (0x00000004, "Executable"),
        (0x00000010, "Merge"),
        (0x00000020, "Strings"),
        (0x00000040, "SHT Info Link"),
        (0x00000080, "Link Order"),
        (0x00000100, "OS Specific Handling"),
        (0x00000200, "Group"),
        (0x00000400, "Thread Local Data"),
        (0x0FF00000, "OS-Specific"),
        (0xF0000000, "Processor Specific"),
        (0x04000000, "Special Ordering (Solaris)"),
        (0x08000000, "Excluded (Solaris)"),
    ];
    for (mask, name) in bit_masks {
        if flags & mask != 0 {
            sh_flags.push(name);
        }
    }
    sh_result.push(format!(
        "{: <align$}{}",
        "Flags:",
        sh_flags.join(","),
        align = align
    ));

    sh_result.push(format!(
        "{: <align$}{}",
        "Section Vaddr in memory:",
        stream.read_int(read_size, info.endianness_le),
        align = align
    ));
    let sh_offset = stream.read_int(read_size, info.endianness_le);
    sh_result.push(format!(
        "{: <align$}{}",
        "Offset of the section:",
        sh_offset,
        align = align
    ));
    let sh_size = stream.read_int(read_size, info.endianness_le);
    sh_result.push(format!(
        "{: <align$}{}",
        "Section Size:",
        sh_size,
        align = align
    ));

    sh_result.push(format!(
        "{: <align$}{}",
        "Associated Section:",
        stream.read_u32(info.endianness_le),
        align = align
    ));
    sh_result.push(format!(
        "{: <align$}{}",
        "Section Extra Information:",
        stream.read_u32(info.endianness_le),
        align = align
    ));

    stream.move_forwards(read_size as i64);
    let ent_size = stream.read_int(read_size, info.endianness_le);

    (
        sh_result.join("\n"),
        sh_type,
        section_name,
        sh_offset,
        sh_size,
        ent_size,
    )
}

fn read_string_at(stream: &mut ElfStream, pos: u64) -> String {
    let old_pos = stream.position();
    stream.move_to(pos);
    let s = stream.read_string();
    stream.move_to(old_pos);
    s
}

fn get_names_offset(stream: &mut ElfStream, info: &HeaderInfo) -> u64 {
    let old_pos = stream.position();
    stream.move_to(info.shoff + (info.shent_size as u64 * info.shstrtab as u64));
    let offset = if info.format == 1 {
        stream.move_forwards(0x10);
        stream.read_u32(info.endianness_le) as u64
    } else {
        stream.move_forwards(0x18);
        stream.read_u64(info.endianness_le)
    };
    stream.move_to(old_pos);
    offset
}

fn get_symbol(stream: &mut ElfStream, info: &HeaderInfo, strtab_offset: u64) -> Option<String> {
    let name_offset = stream.read_u32(info.endianness_le);
    if info.format == 2 {
        stream.move_forwards(20);
    } else {
        stream.move_forwards(12);
    }
    Some(read_string_at(stream, strtab_offset + name_offset as u64))
}
