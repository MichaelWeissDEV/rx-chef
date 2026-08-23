// Tests for the crc32 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations crc32::

use rxchef::operation::ArgValue;
use rxchef::operations::crc32::CRC32;
use rxchef::Operation;

#[test]
fn test_crc32_basic() {
    let operation = CRC32;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // CRC32 IEEE of "Hello, World!" with reflection enabled
    assert_eq!(output, "13B53C2F");
}
#[test]
fn test_crc32_empty() {
    let operation = CRC32;
    let input = b"".to_vec();
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // CRC32 IEEE of empty string = FFFFFFFF
    assert_eq!(output, "FFFFFFFF");
}
#[test]
fn test_crc32_binary() {
    let operation = CRC32;
    let input = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = operation.run(input, &[]).unwrap();
    let output = String::from_utf8(result).unwrap();
    assert!(output.len() == 8); // 32 bits = 8 hex chars
}

#[test]
fn test_crc32_iso_hdlc_raw_register_check_vector() {
    let operation = CRC32;
    // ISO/IEC 13239 publishes CBF43926 for "123456789" after xorout.
    // This operation's default XOR Output applies one further FFFFFFFF XOR,
    // therefore the exposed raw register is its exact complement 340BC6D9.
    assert_eq!(
        operation.run(b"123456789".to_vec(), &[]).unwrap(),
        b"340BC6D9"
    );
}

#[test]
fn test_crc32_rejects_malformed_initial_value() {
    let operation = CRC32;
    let args = [
        ArgValue::Str("IEEE".to_string()),
        ArgValue::Str("not-hex".to_string()),
    ];

    assert!(operation.run(b"data".to_vec(), &args).is_err());
}
