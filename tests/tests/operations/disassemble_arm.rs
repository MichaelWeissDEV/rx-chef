// Tests for the disassemble_arm operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations disassemble_arm::

use rxchef::operation::ArgValue;
use rxchef::operations::disassemble_arm::DisassembleArm;
use rxchef::Operation;

fn run_test(input: &str, args: &[ArgValue]) -> String {
    let op = DisassembleArm;
    let result = op.run(input.as_bytes().to_vec(), args).unwrap();
    String::from_utf8(result).unwrap()
}
#[test]
fn test_arm32_nop() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 a0 e1", &args);
    assert!(output.contains("mov") && output.contains("r0") && output.contains("r0"));
}
#[test]
fn test_arm32_bx_lr() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("1e ff 2f e1", &args);
    assert!(output.contains("bx") && output.contains("lr"));
}
#[test]
fn test_arm32_push() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 48 2d e9", &args);
    assert!(output.contains("push") && output.contains("fp") && output.contains("lr"));
}
#[test]
fn test_arm32_add() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("04 b0 8d e2", &args);
    assert!(output.contains("add") && output.contains("fp") && output.contains("sp"));
}
#[test]
fn test_arm32_ldr() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 91 e5", &args);
    assert!(output.contains("ldr") && output.contains("r0") && output.contains("[r1]"));
}
#[test]
fn test_arm32_str() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 81 e5", &args);
    assert!(output.contains("str") && output.contains("r0") && output.contains("[r1]"));
}
#[test]
fn test_arm32_bl() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 00 eb", &args);
    assert!(output.contains("bl"));
}
#[test]
fn test_arm32_mul() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("91 02 00 e0", &args);
    assert!(
        output.contains("mul")
            && output.contains("r0")
            && output.contains("r1")
            && output.contains("r2")
    );
}
#[test]
fn test_thumb_mov() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("Thumb".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 46", &args);
    assert!(output.contains("mov") && output.contains("r0") && output.contains("r0"));
}
#[test]
fn test_thumb_bx() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("Thumb".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("70 47", &args);
    assert!(output.contains("bx") && output.contains("lr"));
}
#[test]
fn test_thumb_push() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("Thumb".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("10 b5", &args);
    assert!(output.contains("push") && output.contains("r4") && output.contains("lr"));
}
#[test]
fn test_thumb_pop() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("Thumb".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("10 bd", &args);
    assert!(output.contains("pop") && output.contains("r4") && output.contains("pc"));
}
#[test]
fn test_arm64_ret() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("c0 03 5f d6", &args);
    assert!(output.contains("ret"));
}
#[test]
fn test_arm64_mov() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 80 d2", &args);
    assert!(output.contains("mov") && output.contains("x0") && output.contains("#0"));
}
#[test]
fn test_arm64_stp() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("fd 7b bf a9", &args);
    assert!(
        output.contains("stp")
            && output.contains("x29")
            && output.contains("x30")
            && output.contains("sp")
    );
}
#[test]
fn test_arm64_ldp() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("fd 7b c1 a8", &args);
    assert!(
        output.contains("ldp")
            && output.contains("x29")
            && output.contains("x30")
            && output.contains("sp")
    );
}
#[test]
fn test_arm64_add() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("20 00 02 8b", &args);
    assert!(
        output.contains("add")
            && output.contains("x0")
            && output.contains("x1")
            && output.contains("x2")
    );
}
#[test]
fn test_arm64_sub() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("20 00 02 cb", &args);
    assert!(
        output.contains("sub")
            && output.contains("x0")
            && output.contains("x1")
            && output.contains("x2")
    );
}
#[test]
fn test_arm64_mul() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("20 7c 02 9b", &args);
    assert!(
        output.contains("mul")
            && output.contains("x0")
            && output.contains("x1")
            && output.contains("x2")
    );
}
#[test]
fn test_arm64_ldr() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("20 00 40 f9", &args);
    assert!(output.contains("ldr") && output.contains("x0") && output.contains("[x1]"));
}
#[test]
fn test_arm64_str() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("20 00 00 f9", &args);
    assert!(output.contains("str") && output.contains("x0") && output.contains("[x1]"));
}
#[test]
fn test_arm64_bl() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 00 94", &args);
    assert!(output.contains("bl"));
}
#[test]
fn test_arm64_cbz() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 00 b4", &args);
    assert!(output.contains("cbz") && output.contains("x0"));
}
#[test]
fn test_arm64_cbnz() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 00 b5", &args);
    assert!(output.contains("cbnz") && output.contains("x0"));
}
#[test]
fn test_arm64_sub_sp() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("ff 83 00 d1", &args);
    assert!(output.contains("sub") && output.contains("sp") && output.contains("sp"));
}
#[test]
fn test_arm64_add_sp() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("ff 83 00 91", &args);
    assert!(output.contains("add") && output.contains("sp") && output.contains("sp"));
}
#[test]
fn test_arm32_multiple() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 48 2d e9 04 b0 8d e2 00 00 a0 e1 00 88 bd e8", &args);
    assert!(
        output.contains("push")
            && output.contains("add")
            && output.contains("mov")
            && output.contains("pop")
    );
}
#[test]
fn test_arm64_prologue_epilogue() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test(
        "fd 7b bf a9 fd 03 00 91 00 00 80 52 fd 7b c1 a8 c0 03 5f d6",
        &args,
    );
    assert!(
        output.contains("stp")
            && output.contains("mov")
            && output.contains("ldp")
            && output.contains("ret")
    );
}
#[test]
fn test_arm64_start_address() {
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(4096.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("c0 03 5f d6", &args);
    assert!(output.contains("0x00001000"));
}
#[test]
fn test_arm32_start_address() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(32768.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("00 00 a0 e1", &args);
    assert!(output.contains("0x00008000"));
}
#[test]
fn test_arm32_big_endian() {
    let args = [
        ArgValue::Str("ARM (32-bit)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Big Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let output = run_test("e1 a0 00 00", &args);
    assert!(output.contains("mov") && output.contains("r0") && output.contains("r0"));
}
#[test]
fn test_empty_input() {
    let op = DisassembleArm;
    let args = [
        ArgValue::Str("ARM64 (AArch64)".to_string()),
        ArgValue::Str("ARM".to_string()),
        ArgValue::Str("Little Endian".to_string()),
        ArgValue::Num(0.0),
        ArgValue::Bool(true),
        ArgValue::Bool(true),
    ];
    let result = op.run(vec![], &args).unwrap();
    assert!(result.is_empty());
}
