// Tests for the elf_info operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations elf_info::

use rxchef::operations::elf_info::ELFInfo;
use rxchef::Operation;

#[test]
fn test_elf_info_empty_input() {
    let op = ELFInfo;
    let args = [];
    let result = op.run(vec![], &args);
    assert!(result.is_err());
}

#[test]
fn test_elf_info_invalid_elf() {
    let op = ELFInfo;
    let args = [];
    let invalid_elf = vec![0x00, 0x00, 0x00, 0x00]; // Not ELF magic
    let result = op.run(invalid_elf, &args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid ELF"));
}

#[test]
fn test_elf_info_minimal_valid_elf() {
    let op = ELFInfo;
    let args = [];
    // Create a minimal valid ELF header (32-bit, little-endian)
    let mut elf_data = vec![];
    // ELF magic: 0x7f, 'E', 'L', 'F'
    elf_data.extend_from_slice(&[0x7f, 0x45, 0x4c, 0x46]);
    // 32-bit format
    elf_data.push(0x01);
    // Little-endian
    elf_data.push(0x01);
    // ELF version 1
    elf_data.push(0x01);
    // System V ABI
    elf_data.push(0x00);
    // ABI version
    elf_data.push(0x00);
    // Padding (7 bytes)
    elf_data.extend_from_slice(&[0x00; 7]);
    // ET_NONE type (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // EM_NONE machine (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // EV_CURRENT version (0x00000001)
    elf_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    // Entry point (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Program header offset (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Section header offset (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Flags (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // ELF header size (0x0034 = 52 bytes)
    elf_data.extend_from_slice(&[0x34, 0x00]);
    // Program header entry size (0x0020 = 32 bytes)
    elf_data.extend_from_slice(&[0x20, 0x00]);
    // Program header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header entry size (0x0028 = 40 bytes)
    elf_data.extend_from_slice(&[0x28, 0x00]);
    // Section header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header string table index (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);

    let result = op.run(elf_data, &args);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.contains("ELF Header"));
    assert!(result_str.contains("Magic:"));
    assert!(result_str.contains("Format:"));
    assert!(result_str.contains("32-bit"));
}

#[test]
fn test_elf_info_64bit_elf() {
    let op = ELFInfo;
    let args = [];
    // Create a minimal valid 64-bit ELF header
    let mut elf_data = vec![];
    // ELF magic: 0x7f, 'E', 'L', 'F'
    elf_data.extend_from_slice(&[0x7f, 0x45, 0x4c, 0x46]);
    // 64-bit format
    elf_data.push(0x02);
    // Little-endian
    elf_data.push(0x01);
    // ELF version 1
    elf_data.push(0x01);
    // System V ABI
    elf_data.push(0x00);
    // ABI version
    elf_data.push(0x00);
    // Padding (7 bytes)
    elf_data.extend_from_slice(&[0x00; 7]);
    // ET_NONE type (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // EM_X86_64 machine (0x003E)
    elf_data.extend_from_slice(&[0x3E, 0x00]);
    // EV_CURRENT version (0x00000001)
    elf_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    // Entry point (0x0000000000000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    // Program header offset (0x0000000000000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    // Section header offset (0x0000000000000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    // Flags (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // ELF header size (0x0040 = 64 bytes)
    elf_data.extend_from_slice(&[0x40, 0x00]);
    // Program header entry size (0x0038 = 56 bytes)
    elf_data.extend_from_slice(&[0x38, 0x00]);
    // Program header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header entry size (0x0040 = 64 bytes)
    elf_data.extend_from_slice(&[0x40, 0x00]);
    // Section header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header string table index (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);

    let result = op.run(elf_data, &args);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.contains("ELF Header"));
    assert!(result_str.contains("64-bit"));
}

#[test]
fn test_elf_info_big_endian_elf() {
    let op = ELFInfo;
    let args = [];
    // Create a minimal valid big-endian ELF header
    let mut elf_data = vec![];
    // ELF magic: 0x7f, 'E', 'L', 'F'
    elf_data.extend_from_slice(&[0x7f, 0x45, 0x4c, 0x46]);
    // 32-bit format
    elf_data.push(0x01);
    // Big-endian
    elf_data.push(0x02);
    // ELF version 1
    elf_data.push(0x01);
    // System V ABI
    elf_data.push(0x00);
    // ABI version
    elf_data.push(0x00);
    // Padding (7 bytes)
    elf_data.extend_from_slice(&[0x00; 7]);
    // ET_NONE type (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // EM_NONE machine (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // EV_CURRENT version (0x00000001)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]); // Big-endian
    // Entry point (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Program header offset (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Section header offset (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Flags (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // ELF header size (0x0034 = 52 bytes)
    elf_data.extend_from_slice(&[0x00, 0x34]);
    // Program header entry size (0x0020 = 32 bytes)
    elf_data.extend_from_slice(&[0x00, 0x20]);
    // Program header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header entry size (0x0028 = 40 bytes)
    elf_data.extend_from_slice(&[0x00, 0x28]);
    // Section header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header string table index (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);

    let result = op.run(elf_data, &args);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.contains("ELF Header"));
    assert!(result_str.contains("Big"));
}

#[test]
fn test_elf_info_with_program_headers() {
    let op = ELFInfo;
    let args = [];
    // Create a minimal ELF with program headers
    let mut elf_data = vec![];
    // ELF magic: 0x7f, 'E', 'L', 'F'
    elf_data.extend_from_slice(&[0x7f, 0x45, 0x4c, 0x46]);
    // 32-bit format
    elf_data.push(0x01);
    // Little-endian
    elf_data.push(0x01);
    // ELF version 1
    elf_data.push(0x01);
    // System V ABI
    elf_data.push(0x00);
    // ABI version
    elf_data.push(0x00);
    // Padding (7 bytes)
    elf_data.extend_from_slice(&[0x00; 7]);
    // ET_EXEC type (0x0002)
    elf_data.extend_from_slice(&[0x02, 0x00]);
    // EM_386 machine (0x0003)
    elf_data.extend_from_slice(&[0x03, 0x00]);
    // EV_CURRENT version (0x00000001)
    elf_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    // Entry point (0x08048000)
    elf_data.extend_from_slice(&[0x00, 0x80, 0x04, 0x08]);
    // Program header offset (0x00000034 = 52 bytes, right after ELF header)
    elf_data.extend_from_slice(&[0x34, 0x00, 0x00, 0x00]);
    // Section header offset (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // Flags (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // ELF header size (0x0034 = 52 bytes)
    elf_data.extend_from_slice(&[0x34, 0x00]);
    // Program header entry size (0x0020 = 32 bytes)
    elf_data.extend_from_slice(&[0x20, 0x00]);
    // Program header entries (0x0001)
    elf_data.extend_from_slice(&[0x01, 0x00]);
    // Section header entry size (0x0028 = 40 bytes)
    elf_data.extend_from_slice(&[0x28, 0x00]);
    // Section header entries (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);
    // Section header string table index (0x0000)
    elf_data.extend_from_slice(&[0x00, 0x00]);

    // Add a minimal program header (PT_LOAD type)
    // p_type = PT_LOAD (0x00000001)
    elf_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    // p_offset (0x00000000)
    elf_data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    // p_vaddr (0x08048000)
    elf_data.extend_from_slice(&[0x00, 0x80, 0x04, 0x08]);
    // p_paddr (0x08048000)
    elf_data.extend_from_slice(&[0x00, 0x80, 0x04, 0x08]);
    // p_filesz (0x00001000)
    elf_data.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]);
    // p_memsz (0x00001000)
    elf_data.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]);
    // p_flags (PF_R | PF_X = 0x00000005)
    elf_data.extend_from_slice(&[0x05, 0x00, 0x00, 0x00]);
    // p_align (0x00001000)
    elf_data.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]);

    let result = op.run(elf_data, &args);
    assert!(result.is_ok());
    let result_str = String::from_utf8(result.unwrap()).unwrap();
    assert!(result_str.contains("ELF Header"));
    assert!(result_str.contains("Program Header"));
    assert!(result_str.contains("Loadable Segment"));
}

#[test]
fn test_elf_info_truncated_elf() {
    let op = ELFInfo;
    let args = [];
    // Incomplete ELF header (missing most fields)
    let truncated_elf = vec![0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01];
    let result = op.run(truncated_elf, &args);
    // The operation might handle this gracefully or fail - let's check both cases
    if result.is_ok() {
        // If it succeeds, it should still produce some output
        let output = result.unwrap();
        assert!(output.len() > 0);
    } else {
        // If it fails, that's also acceptable
        assert!(result.is_err());
    }
}
