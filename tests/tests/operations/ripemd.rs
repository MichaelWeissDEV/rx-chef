// Tests for the ripemd operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations ripemd::

use rxchef::operation::ArgValue;
use rxchef::operations::ripemd::RIPEMD;
use rxchef::Operation;

#[test]
fn test_ripemd160_basic() {
    let operation = RIPEMD;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(160.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // RIPEMD-160 hash of "Hello, World!"
    assert_eq!(output, "527a6a4b9a6da75607546842e0e00105350b1aaf");
}
#[test]
fn test_ripemd128_basic() {
    let operation = RIPEMD;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(128.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // RIPEMD-128 hash of "Hello, World!"
    assert_eq!(output, "67f9fe75ca2886dc76ad00f7276bdeba");
}
#[test]
fn test_ripemd256_basic() {
    let operation = RIPEMD;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(256.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // RIPEMD-256 hash of "Hello, World!"
    assert_eq!(
        output,
        "567750c6d34dcba7ae038a80016f3ca3260ec25bfdb0b68bbb8e730b00b2447d"
    );
}
#[test]
fn test_ripemd320_basic() {
    let operation = RIPEMD;
    let input = b"Hello, World!".to_vec();
    let result = operation.run(input, &[ArgValue::Num(320.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // RIPEMD-320 hash of "Hello, World!"
    assert_eq!(
        output,
        "f9832e5bb00576fc56c2221f404eb77addeafe49843c773f0df3fc5a996d5934f3c96e94aeb80e89"
    );
}
#[test]
fn test_ripemd_empty() {
    let operation = RIPEMD;
    let input = b"".to_vec();
    let result = operation.run(input, &[ArgValue::Num(160.0)]).unwrap();
    let output = String::from_utf8(result).unwrap();
    // RIPEMD-160 hash of empty string
    assert_eq!(output, "9c1185a5c5e9fc54612808977ee8f548b2258d31");
}
#[test]
fn test_ripemd_invalid_size() {
    let operation = RIPEMD;
    let input = b"test".to_vec();
    let args = [ArgValue::Num(100.0)]; // Invalid size
    let result = operation.run(input, &args);
    assert!(result.is_err());
}
