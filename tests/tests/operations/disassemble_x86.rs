// Tests for the disassemble_x86 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations disassemble_x86::

use rxchef::operations::disassemble_x86::DisassembleX86;
use rxchef::Operation;

#[test]
fn test_disassemble_x86_empty_input() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert_eq!(result, "".as_bytes());
}

#[test]
fn test_disassemble_x86_simple_64bit() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Simple MOV instruction: 48 89 C8 (mov rax, rcx)
    let result = op.run("4889C8".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("mov"));
    assert!(result_str.contains("rax"));
    assert!(result_str.contains("rcx"));
}

#[test]
fn test_disassemble_x86_32bit_mode() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("32".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // 32-bit MOV instruction: 89 C8 (mov eax, ecx)
    let result = op.run("89C8".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("mov"));
    assert!(result_str.contains("eax"));
    assert!(result_str.contains("ecx"));
}

#[test]
fn test_disassemble_x86_16bit_mode() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("16".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // 16-bit MOV instruction: 89 C8 (mov ax, cx)
    let result = op.run("89C8".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("mov"));
    assert!(result_str.contains("ax"));
    assert!(result_str.contains("cx"));
}

#[test]
fn test_disassemble_x86_with_whitespace() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Input with spaces and newlines
    let result = op.run("48 89 C8\n48 89 D1".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("mov"));
}

#[test]
fn test_disassemble_x86_no_hex_display() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(false), // No hex display
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run("4889C8".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should not contain the hex bytes in the output
    assert!(!result_str.contains("4889c8"));
    assert!(result_str.contains("mov"));
}

#[test]
fn test_disassemble_x86_no_position_display() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(false), // No position display
    ];
    let result = op.run("4889C8".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    // Should not contain address positions
    assert!(!result_str.contains("00000000"));
    assert!(result_str.contains("mov"));
}

#[test]
fn test_disassemble_x86_with_offset() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(4096.0), // Offset (0x1000)
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run("4889C8".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("mov"));
    assert!(result_str.contains("1000"));
}

#[test]
fn test_disassemble_x86_invalid_hex() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run("ZZZ123".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_disassemble_x86_invalid_bit_mode() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("128".to_string()), // Invalid bit mode
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    let result = op.run("4889C8".as_bytes().to_vec(), &args);
    assert!(result.is_err());
}

#[test]
fn test_disassemble_x86_multiple_instructions() {
    let op = DisassembleX86;
    let args = [
        rxchef::operation::ArgValue::Str("64".to_string()),
        rxchef::operation::ArgValue::Str("Full x86 architecture".to_string()),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Num(0.0),
        rxchef::operation::ArgValue::Bool(true),
        rxchef::operation::ArgValue::Bool(true),
    ];
    // Multiple instructions: MOV, ADD, RET
    let result = op.run("4889C84883C001C3".as_bytes().to_vec(), &args).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("mov"));
    assert!(result_str.contains("add"));
    assert!(result_str.contains("ret"));
}
