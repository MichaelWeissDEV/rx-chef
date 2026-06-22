// Tests for the encode_netbios_name operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations encode_netbios_name::

use rxchef::operations::encode_netbios_name::EncodeNetBIOSName;
use rxchef::Operation;

#[test]
fn test_encode_netbios_name_empty_input() {
    let op = EncodeNetBIOSName;
    let args = [rxchef::operation::ArgValue::Num(65.0)];
    let result = op.run(vec![], &args).unwrap();
    // Empty input should result in 32 bytes of padding (16 * 2)
    assert_eq!(result.len(), 32);
    // Empty input gets padded with spaces (32), which encode to 67 and 65
    // So we should have alternating 67 and 65 values
    for (i, &byte) in result.iter().enumerate() {
        if i % 2 == 0 {
            assert_eq!(byte, 67, "Even indices should be 67");
        } else {
            assert_eq!(byte, 65, "Odd indices should be 65");
        }
    }
}

#[test]
fn test_encode_netbios_name_short_input() {
    let op = EncodeNetBIOSName;
    let args = [rxchef::operation::ArgValue::Num(65.0)];
    let input = b"TEST".to_vec();
    let result = op.run(input, &args).unwrap();
    // Should be 32 bytes (16 * 2)
    assert_eq!(result.len(), 32);
    // First 8 bytes should be encoded "TEST" (padded with spaces)
    // 'T' (0x54) -> (0x5, 0x4) + 65 = (70, 69)
    assert_eq!(result[0], 70); // T high nibble + 65
    assert_eq!(result[1], 69); // T low nibble + 65
}

#[test]
fn test_encode_netbios_name_exact_length() {
    let op = EncodeNetBIOSName;
    let args = [rxchef::operation::ArgValue::Num(65.0)];
    let input = b"EXACTLY16CHARS".to_vec(); // Exactly 16 bytes
    let result = op.run(input, &args).unwrap();
    assert_eq!(result.len(), 32); // 16 * 2
    // Should encode all 16 characters without padding
}

#[test]
fn test_encode_netbios_name_long_input() {
    let op = EncodeNetBIOSName;
    let args = [rxchef::operation::ArgValue::Num(65.0)];
    let input = b"THISISMORETHAN16CHARACTERS".to_vec(); // More than 16 bytes
    let result = op.run(input, &args).unwrap();
    // Should return empty vector for input > 16 bytes
    assert_eq!(result.len(), 0);
}

#[test]
fn test_encode_netbios_name_custom_offset() {
    let op = EncodeNetBIOSName;
    let args = [rxchef::operation::ArgValue::Num(100.0)]; // Custom offset
    let input = b"A".to_vec();
    let result = op.run(input, &args).unwrap();
    assert_eq!(result.len(), 32);
    // 'A' (0x41) -> (0x4, 0x1) + 100 = (104, 101)
    assert_eq!(result[0], 104); // A high nibble + 100
    assert_eq!(result[1], 101); // A low nibble + 100
}
